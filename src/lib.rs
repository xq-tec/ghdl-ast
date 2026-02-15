#![allow(missing_docs, reason = "// TODO remove before release")]
#![expect(clippy::todo, reason = "// TODO remove before release")]

mod concurrent_statements;
mod declarations;
mod definitions;
mod expressions;
mod identifier;
mod libraries;
mod names;
mod nodes;
mod sequential_statements;
mod unsorted;

use std::env;
use std::fmt;
use std::fs;
use std::io::BufRead;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use compact_str::CompactString;
use log::debug;
use log::error;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

pub use self::concurrent_statements::*;
pub use self::declarations::*;
pub use self::definitions::*;
pub use self::expressions::*;
pub use self::identifier::Identifier;
pub use self::identifier::NormalizedIdentifier;
pub use self::libraries::*;
pub use self::names::*;
pub use self::nodes::AstNodeId;
pub use self::nodes::DowncastNodeId;
pub use self::nodes::GenericNodeId;
pub use self::nodes::Node;
pub use self::nodes::NodeId;
pub use self::sequential_statements::*;
pub use self::unsorted::*;

type Map<K, V> = rustc_hash::FxHashMap<K, V>;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(from = "(u32, u32, u32)", into = "(u32, u32, u32)")]
pub struct Location {
    pub file_name: u32,
    pub line: u32,
    pub column: u32,
}

impl From<(u32, u32, u32)> for Location {
    fn from((file_name, line, column): (u32, u32, u32)) -> Self {
        Self {
            file_name,
            line,
            column,
        }
    }
}

impl From<Location> for (u32, u32, u32) {
    fn from(loc: Location) -> Self {
        (loc.file_name, loc.line, loc.column)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AstMetadata {
    #[serde(default)]
    files: Vec<FileMetadata>,
    #[serde(default)]
    libraries: Vec<NodeId<Library>>,
}

#[derive(Serialize)]
struct FileMetadata {
    source: GhdlSource,
    start: usize,
    end: usize,

    #[serde(skip)]
    content: Vec<u8>,
}

impl<'de> Deserialize<'de> for FileMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            source: GhdlSource,
            start: usize,
            end: usize,
        }

        let helper = Helper::deserialize(deserializer)?;
        let content = if let GhdlSource::File(path) = &helper.source {
            fs::read(path).unwrap_or_else(|error| {
                error!(
                    "failed to read source file {path}: {error}",
                    path = path.display(),
                );
                vec![]
            })
        } else {
            vec![]
        };
        Ok(Self {
            source: helper.source,
            start: helper.start,
            end: helper.end,
            content,
        })
    }
}

impl fmt::Debug for FileMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{source:?} ({start}..{end})",
            source = self.source,
            start = self.start,
            end = self.end,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum GhdlSource {
    #[serde(rename = "*libraries*")]
    Libraries,
    #[serde(rename = "*command line*")]
    CommandLine,
    #[serde(rename = "*std_standard*")]
    StdStandard,
    #[serde(untagged)]
    File(PathBuf),
}

#[derive(Debug)]
pub struct Ast {
    nodes: Vec<Node>,
    /// Map from library identifier to library node ID.
    libraries: Map<NormalizedIdentifier, NodeId<Library>>,
    /// Map from (library node ID, package name) to package declaration node IDs.
    package_declarations: Map<(NodeId<Library>, NormalizedIdentifier), NodeId<PackageDeclaration>>,
    /// Map from (library node ID, entity name) to entity declaration node IDs.
    entity_declarations: Map<(NodeId<Library>, NormalizedIdentifier), NodeId<EntityDeclaration>>,
    /// Map from entity declaration node ID to their architectures.
    architecture_bodies: Map<NodeId<EntityDeclaration>, Vec<NodeId<ArchitectureBody>>>,
}

impl Ast {
    /// Constructs an `Ast` from a JSON stream.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the buffer or parsing the JSON fails.
    pub fn from_json(reader: &mut dyn BufRead) -> Result<Self> {
        let mut line_number: u32 = 1;
        let mut line_buffer = String::new();
        reader.read_line(&mut line_buffer)?;
        let metadata: AstMetadata =
            serde_json::from_str(&line_buffer).context("could not parse AST metadata")?;
        debug!("AST metadata: {metadata:#?}");
        let files_metadata = Arc::<[FileMetadata]>::from(metadata.files);
        identifier::set_file_metadata(Some(files_metadata));

        let mut nodes = vec![Node::Empty, Node::Empty];
        loop {
            line_number += 1;
            line_buffer.clear();
            reader.read_line(&mut line_buffer)?;
            let line = line_buffer.trim();
            if line.is_empty() {
                break;
            }

            let node_opt = serde_json::from_str::<Option<Node>>(line)
                .with_context(|| format!("parse error in line {line_number}: {line}"))?;
            nodes.push(node_opt.unwrap_or(Node::Empty));
        }

        if let Ok(path) = env::var("RISIM_DUMP_AST") {
            use std::fs::File;
            use std::io::BufWriter;
            use std::io::Write as _;

            let file = File::create(&path)
                .with_context(|| format!("failed to create AST dump file at {path}"))?;
            let mut writer = BufWriter::new(file);
            for (index, node) in nodes.iter().enumerate() {
                writeln!(writer, "{index:6}: {node:?}")?;
            }
        }

        let mut instance = Self {
            nodes,
            libraries: Map::default(),
            package_declarations: Map::default(),
            entity_declarations: Map::default(),
            architecture_bodies: Map::default(),
        };
        instance.build_maps(&metadata.libraries);
        debug_assert!(
            Error::GLOBAL_ID.try_get(&instance).is_ok(),
            "sanity check for global error node failed",
        );

        identifier::set_file_metadata(None);

        Ok(instance)
    }

    fn build_maps(&mut self, libraries_list: &[NodeId<Library>]) {
        let mut libraries = Map::<NormalizedIdentifier, NodeId<Library>>::default();
        let mut package_declarations =
            Map::<(NodeId<Library>, NormalizedIdentifier), NodeId<PackageDeclaration>>::default();
        let mut entity_declarations =
            Map::<(NodeId<Library>, NormalizedIdentifier), NodeId<EntityDeclaration>>::default();
        let mut architecture_bodies =
            Map::<NodeId<EntityDeclaration>, Vec<NodeId<ArchitectureBody>>>::default();

        for &library_id in libraries_list {
            let library = library_id.get(self);
            libraries.insert(library.identifier.normalized.clone(), library_id);

            for (library_unit_id, library_unit) in library.library_units_iter(self) {
                match library_unit {
                    LibraryUnit::PackageDeclaration(package) => {
                        package_declarations.insert(
                            (library_id, package.identifier.normalized.clone()),
                            library_unit_id.downcast(),
                        );
                    },

                    LibraryUnit::EntityDeclaration(entity_declaration) => {
                        entity_declarations.insert(
                            (library_id, entity_declaration.identifier.normalized.clone()),
                            library_unit_id.downcast(),
                        );
                    },

                    LibraryUnit::ArchitectureBody(architecture) => {
                        let entity_name = architecture.entity_name.get(self);
                        // TODO shouldn't we check that the id is of the correct variant rather than just down-casting?
                        let entity_declaration_id = entity_name.named_entity.downcast();
                        architecture_bodies
                            .entry(entity_declaration_id)
                            .or_default()
                            .push(library_unit_id.downcast());
                    },

                    _ => {
                        // TODO
                    },
                }
            }
        }

        self.libraries = libraries;
        self.package_declarations = package_declarations;
        self.entity_declarations = entity_declarations;
        self.architecture_bodies = architecture_bodies;
    }

    #[must_use]
    pub fn lookup_library(&self, identifier: &NormalizedIdentifier) -> Option<NodeId<Library>> {
        self.libraries.get(identifier).copied()
    }

    /// Returns the single user library, or an error if none or multiple are found.
    ///
    /// # Errors
    ///
    /// Returns an error if no library is found or if multiple libraries are found.
    pub fn single_library(&self) -> Result<(&NormalizedIdentifier, NodeId<Library>)> {
        let mut libraries = self
            .libraries
            .iter()
            .filter(|(name, _)| !["std", "ieee"].contains(&name.as_str()));

        let Some((library_name, &library_id)) = libraries.next() else {
            bail!("no library found");
        };
        if libraries.next().is_some() {
            bail!("multiple libraries found");
        }
        Ok((library_name, library_id))
    }

    #[must_use]
    pub fn lookup_package_declaration(
        &self,
        library_id: NodeId<Library>,
        identifier: NormalizedIdentifier,
    ) -> Option<NodeId<PackageDeclaration>> {
        self.package_declarations
            .get(&(library_id, identifier))
            .copied()
    }

    #[must_use]
    pub fn package_declarations(
        &self,
    ) -> &Map<(NodeId<Library>, NormalizedIdentifier), NodeId<PackageDeclaration>> {
        &self.package_declarations
    }

    #[must_use]
    pub fn lookup_entity_declaration(
        &self,
        library_id: NodeId<Library>,
        identifier: NormalizedIdentifier,
    ) -> Option<NodeId<EntityDeclaration>> {
        self.entity_declarations
            .get(&(library_id, identifier))
            .copied()
    }

    /// Returns the single entity declaration in the given library, or an error if none or multiple are found.
    ///
    /// # Errors
    ///
    /// Returns an error if no entity is found in the library or if multiple entities are found.
    pub fn single_entity_declaration(
        &self,
        library_id: NodeId<Library>,
    ) -> Result<(&Identifier, NodeId<EntityDeclaration>)> {
        let mut library_entities =
            self.entity_declarations
                .iter()
                .filter_map(|(&(entity_library_id, _), &entity_id)| {
                    (entity_library_id == library_id).then_some(entity_id)
                });
        let Some(entity_id) = library_entities.next() else {
            bail!("no entity found in this library");
        };
        if library_entities.next().is_some() {
            bail!("multiple entities found in this library");
        }
        let entity_declaration = entity_id.get(self);
        Ok((&entity_declaration.identifier, entity_id))
    }

    pub fn lookup_architecture_bodies(
        &self,
        entity_declaration_id: NodeId<EntityDeclaration>,
    ) -> &[NodeId<ArchitectureBody>] {
        self.architecture_bodies
            .get(&entity_declaration_id)
            .map_or(&[], Vec::as_slice)
    }
}
