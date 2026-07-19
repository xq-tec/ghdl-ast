//! Name forms and named entities (LRM clause 8).
//!
//! Covers simple / selected / indexed / sliced names, attribute names, operator
//! symbols, external names (VHDL-2008), and pathname fragments used by them.
//!
//! [`NamedEntity`] is the resolution target of a denoting name after analysis
//! (LRM §8.1 / §6.1). Denoting names ([`SimpleName`], [`SelectedName`],
//! [`OperatorSymbol`], …) point at one of these entities. The set mirrors the
//! LRM “named entities” list used for overload resolution, visibility, and
//! attribute association—not every AST declaration kind.
//!
//! Typical simulation uses:
//! - object declarations → storage / drivers
//! - type / element / unit / enum literal → typing and literals
//! - subprograms / instantiations → call targets
//! - design units / libraries → elaboration hierarchy
//!
//! [`NamedEntity::Unresolved`] is retained for the rare case where GHDL could
//! not bind a name (see that variant's docs).

use super::*;

subset_declaration!(DenotingName DenotingNameOwned DenotingNameNodeId {
    CharacterLiteral(CharacterLiteral),
    SimpleName(SimpleName),
    SelectedName(SelectedName),
    OperatorSymbol(OperatorSymbol),
    ReferenceName(ReferenceName),
});

subset_declaration!(Name NameOwned NameNodeId {
    AttributeName(AttributeName),
    IndexedName(IndexedName),
    SelectedByAllName(SelectedByAllName),
    SelectedName(SelectedName),
    SimpleName(SimpleName),
    SliceName(SliceName),
});

impl Name<'_> {
    /// Returns the resolved named entity for name forms that bind one.
    #[must_use]
    pub fn named_entity(&self) -> Option<NamedEntityNodeId> {
        match self {
            Self::AttributeName(attribute_name) => Some(attribute_name.named_entity),
            Self::SelectedName(selected_name) => Some(selected_name.named_entity),
            Self::SimpleName(simple_name) => Some(simple_name.named_entity),
            Self::IndexedName(_) | Self::SelectedByAllName(_) | Self::SliceName(_) => None,
        }
    }

    /// Walks the name prefix chain and returns its elements in reverse order.
    #[must_use]
    pub fn elements(&self, ast: &Ast) -> NameElements {
        let mut elements = Vec::with_capacity(4);

        let mut next_prefix = match self {
            Self::AttributeName(attribute_name) => {
                elements.push(NameElement::NamedEntity(attribute_name.named_entity));
                attribute_name.prefix
            },
            Self::IndexedName(indexed_name) => {
                elements.push(NameElement::Other);
                indexed_name.prefix
            },
            Self::SelectedByAllName(selected_by_all_name) => {
                elements.push(NameElement::All);
                selected_by_all_name.prefix
            },
            Self::SelectedName(selected_name) => {
                elements.push(NameElement::NamedEntity(selected_name.named_entity));
                selected_name.prefix
            },
            Self::SimpleName(simple_name) => {
                elements.push(NameElement::NamedEntity(simple_name.named_entity));
                return NameElements { elements };
            },
            Self::SliceName(slice_name) => {
                elements.push(NameElement::Other);
                slice_name.prefix
            },
        };

        loop {
            next_prefix = match next_prefix.get(ast) {
                Prefix::AttributeName(attribute_name) => {
                    elements.push(NameElement::NamedEntity(attribute_name.named_entity));
                    attribute_name.prefix
                },
                Prefix::IndexedName(indexed_name) => {
                    elements.push(NameElement::Other);
                    indexed_name.prefix
                },
                Prefix::SelectedName(selected_name) => {
                    elements.push(NameElement::NamedEntity(selected_name.named_entity));
                    selected_name.prefix
                },
                Prefix::SimpleName(simple_name) => {
                    elements.push(NameElement::NamedEntity(simple_name.named_entity));
                    return NameElements { elements };
                },
                Prefix::SliceName(slice_name) => {
                    elements.push(NameElement::Other);
                    slice_name.prefix
                },

                Prefix::SelectedElement(selected_element) => {
                    elements.push(NameElement::NamedEntity(selected_element.named_entity));
                    selected_element.prefix
                },
                Prefix::FunctionCall(function_call) => {
                    elements.push(NameElement::Other);
                    function_call.prefix
                },
                Prefix::Dereference(dereference) => {
                    // Dereference operation, not an element of the name
                    dereference.prefix
                },
                Prefix::ImplicitDereference(implicit_dereference) => {
                    // Implicit operation, not an element of the name
                    implicit_dereference.prefix
                },
                Prefix::OperatorSymbol(_) => {
                    elements.push(NameElement::Other);
                    return NameElements { elements };
                },
            };
        }
    }
}

/// Elements of a decomposed VHDL name, stored innermost-first.
pub struct NameElements {
    /// The elements of the name, in reverse order.
    elements: Vec<NameElement>,
}

impl NameElements {
    /// Iterates over the name elements from outermost to innermost.
    #[must_use]
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = NameElement> + ExactSizeIterator {
        self.elements.iter().copied().rev()
    }

    /// Iterates over the named-entity elements from outermost to innermost.
    #[must_use]
    pub fn named_entities(&self) -> impl DoubleEndedIterator<Item = NamedEntityNodeId> {
        self.elements
            .iter()
            .rev()
            .filter_map(|element| match element {
                &NameElement::NamedEntity(named_entity) => Some(named_entity),
                _ => None,
            })
    }
}

/// One step in a decomposed VHDL name.
#[derive(Clone, Copy, Debug)]
pub enum NameElement {
    /// A suffix that resolves to a named entity.
    NamedEntity(NamedEntityNodeId),
    /// A `.all` selection.
    All,
    /// An index, slice, call, or other non-entity suffix.
    Other,
}

subset_declaration!(AnySelectedName AnySelectedNameOwned AnySelectedNameNodeId {
    SelectedName(SelectedName),
    SelectedByAllName(SelectedByAllName),
});

impl<'node> From<AnySelectedName<'node>> for Name<'node> {
    fn from(value: AnySelectedName<'node>) -> Self {
        match value {
            AnySelectedName::SelectedName(selected_name) => Name::SelectedName(selected_name),
            AnySelectedName::SelectedByAllName(selected_by_all_name) => {
                Name::SelectedByAllName(selected_by_all_name)
            },
        }
    }
}

subset_declaration!(Prefix PrefixOwned PrefixNodeId {
    AttributeName(AttributeName),
    IndexedName(IndexedName),
    SelectedName(SelectedName),
    SimpleName(SimpleName),
    SliceName(SliceName),

    SelectedElement(SelectedElement),
    FunctionCall(FunctionCall),
    Dereference(Dereference),
    ImplicitDereference(ImplicitDereference),
    OperatorSymbol(OperatorSymbol),
});

/// Attribute name (`prefix'attr`).
///
/// The named entity is typically an attribute declaration or an implicit
/// attribute value/function created during analysis.
///
/// ```vhdl
/// clk'event
/// vec'length
/// my_sig'delayed(1 ns)
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeName {
    /// Prefix to which the attribute is applied.
    pub prefix: PrefixNodeId,
    /// Resolved attribute (declaration, implicit attribute, …).
    pub named_entity: NamedEntityNodeId,
}

/// Explicit access-value dereference (`prefix.all` for an access type).
///
/// ```vhdl
/// ptr.all := 5;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct Dereference {
    /// Access-valued prefix being dereferenced.
    pub prefix: PrefixNodeId,
}

/// Implicit dereference inserted by analysis when an access prefix is used as
/// a designated object without writing `.all`.
#[derive(Debug, Deserialize, Serialize)]
pub struct ImplicitDereference {
    /// Access-valued prefix being implicitly dereferenced.
    pub prefix: PrefixNodeId,
}

/// Indexed name (`prefix(index, …)`).
///
/// ```vhdl
/// mem(0)
/// matrix(i, j)
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IndexedName {
    /// Array (or similar) prefix being indexed.
    pub prefix: PrefixNodeId,
    /// Index expressions, or the special `others` encoding.
    pub index_list: IndexList,
    /// Analyzed type of the indexed element.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
}

/// Operator symbol used as a denoting name (`"+"` , `"and"`, …).
///
/// Appears in overload resolution, alias targets, and attribute designators for
/// operators. The identifier holds the operator string (including quotes in the
/// source representation GHDL stores).
///
/// ```vhdl
/// function "+" (l, r : my_t) return my_t;
/// alias add is "+" [my_t, my_t return my_t];
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct OperatorSymbol {
    /// Operator designator string.
    pub identifier: Option<Identifier>,
    /// Analyzed type when the operator denotes a specific overload.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Resolved subprogram or other named entity, when known.
    pub named_entity: Option<NamedEntityNodeId>,
}

/// Selected element name used as an intermediate prefix after analysis.
///
/// Similar to [`SelectedName`], but GHDL may use this form for record-element
/// selections that participate in longer name chains.
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedElement {
    /// Prefix before the selected suffix.
    pub prefix: PrefixNodeId,
    /// Named entity denoted by the selection (often an element declaration).
    pub named_entity: NamedEntityNodeId,
}

/// Selected name with `.all` (`prefix.all`).
///
/// For access types this is usually rewritten to a [`Dereference`]; the
/// `.all` form also appears for package/use contexts.
///
/// ```vhdl
/// use work.pkg.all;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedByAllName {
    /// Prefix before `.all`.
    pub prefix: PrefixNodeId,
}

/// Selected name (`prefix.suffix`).
///
/// ```vhdl
/// work.pkg
/// rec.field
/// lib.entity_name
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedName {
    /// Suffix identifier.
    pub identifier: Identifier,
    /// Resolved named entity of the selection.
    pub named_entity: NamedEntityNodeId,
    /// Prefix before the selected suffix.
    pub prefix: PrefixNodeId,
}

/// Simple name (a single identifier).
///
/// [`named_entity`](Self::named_entity) is the declaration or other named
/// entity this identifier resolved to. When resolution failed for an explicit
/// architecture in a component instantiation, it may be
/// [`NamedEntity::Unresolved`].
///
/// ```vhdl
/// clk
/// std_logic
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleName {
    /// Identifier as written / stored by GHDL.
    pub identifier: Identifier,
    /// Resolved named entity (or the global error node when unresolved).
    #[serde(default = "unresolved_named_entity")]
    pub named_entity: NamedEntityNodeId,
}

fn unresolved_named_entity() -> NamedEntityNodeId {
    Error::GLOBAL_ID.into()
}

/// Slice name (`prefix(discrete_range)`).
///
/// ```vhdl
/// vec(7 downto 0)
/// str(1 to 3)
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SliceName {
    /// Array prefix being sliced.
    pub prefix: PrefixNodeId,
    /// Discrete range of the slice.
    pub suffix: RangeConstraintNodeId,
}

subset_declaration!(NamedEntity NamedEntityOwned NamedEntityNodeId {
    TypeDeclaration(TypeDeclaration),
    VariableDeclaration(VariableDeclaration),
    ConstantDeclaration(ConstantDeclaration),
    SignalDeclaration(SignalDeclaration),
    FileDeclaration(FileDeclaration),
    ObjectAliasDeclaration(ObjectAliasDeclaration),

    InterfaceTypeDeclaration(InterfaceTypeDeclaration),
    InterfaceVariableDeclaration(InterfaceVariableDeclaration),
    InterfaceConstantDeclaration(InterfaceConstantDeclaration),
    InterfaceSignalDeclaration(InterfaceSignalDeclaration),
    InterfaceViewDeclaration(InterfaceViewDeclaration),
    InterfaceFileDeclaration(InterfaceFileDeclaration),
    InterfacePackageDeclaration(InterfacePackageDeclaration),
    InterfaceFunctionDeclaration(InterfaceFunctionDeclaration),
    InterfaceProcedureDeclaration(InterfaceProcedureDeclaration),

    AttributeDeclaration(AttributeDeclaration),
    ComponentDeclaration(ComponentDeclaration),
    // TODO "A group template declaration" (LRM § 6.1)
    // TODO "A group declaration" (LRM § 6.1)
    FunctionDeclaration(FunctionDeclaration),
    ProcedureDeclaration(ProcedureDeclaration),
    FunctionInstantiationDeclaration(FunctionInstantiationDeclaration),
    ProcedureInstantiationDeclaration(ProcedureInstantiationDeclaration),
    ConfigurationDeclaration(ConfigurationDeclaration),
    ContextDeclaration(ContextDeclaration),
    EntityDeclaration(EntityDeclaration),
    PackageDeclaration(PackageDeclaration),
    PackageInstantiationDeclaration(PackageInstantiationDeclaration),
    ArchitectureBody(ArchitectureBody),
    // TODO "A PSL property declaration" (LRM § 6.1)
    // TODO "A PSL sequence declaration" (LRM § 6.1)
    EnumerationLiteral(EnumerationLiteral),
    UnitDeclaration(UnitDeclaration),
    ElementDeclaration(ElementDeclaration),
    /// Parameter specification iterator of a `for` loop or `for` generate.
    IteratorDeclaration(IteratorDeclaration),
    // TODO "An implicit label declaration" (LRM § 6.1)
    Library(Library),
    /// Dummy node for unresolved names.
    ///
    /// There should only be one case where this variant occurs: as the named entity of a
    /// `SimpleName` for the explicit architecture body of a component instantiation, and no
    /// architecture of that name exists:
    ///
    /// ```vhdl
    /// mult: entity work.Mult(parallel) ...
    /// ```
    Unresolved(Error),
});

/// Synthetic name that re-refers to an already resolved named entity.
///
/// Used by GHDL when a later reference must point at the same declaration as
/// an earlier name without re-parsing the original identifier spelling.
#[derive(Debug, Deserialize, Serialize)]
pub struct ReferenceName {
    /// Named entity being re-referenced.
    pub named_entity: Option<NamedEntityNodeId>,
    /// Original name this reference was created from.
    pub referenced_name: Option<NameNodeId>,
    /// Analyzed type of the referenced object, when applicable.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// External constant name (`<< constant pathname : subtype >>`).
///
/// VHDL-2008 hierarchical reference to a constant declared elsewhere in the
/// elaborated design.
///
/// ```vhdl
/// << constant .tb.dut.WIDTH : natural >>
/// << constant ^.sibling.C : integer >>
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalConstantName {
    /// Absolute, relative, or package pathname.
    pub external_pathname: Option<GenericNodeId>,
    /// Resolved constant, when bound.
    pub named_entity: Option<NamedEntityNodeId>,
    /// Declared subtype indication of the external name.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// External signal name (`<< signal pathname : subtype >>`).
///
/// ```vhdl
/// << signal .tb.dut.clk : std_logic >>
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalSignalName {
    /// Absolute, relative, or package pathname.
    pub external_pathname: Option<GenericNodeId>,
    /// Resolved signal, when bound.
    pub named_entity: Option<NamedEntityNodeId>,
    /// Declared subtype indication of the external name.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Whether the external signal is treated as guarded.
    #[serde(default)]
    pub guarded_signal_flag: bool,
    /// `register` / `bus` kind when guarded.
    pub signal_kind: Option<SignalKind>,
}

/// External variable name (`<< variable pathname : subtype >>`).
///
/// ```vhdl
/// << variable .tb.shared_pkg.COUNT : natural >>
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalVariableName {
    /// Absolute, relative, or package pathname.
    pub external_pathname: Option<GenericNodeId>,
    /// Resolved variable, when bound.
    pub named_entity: Option<NamedEntityNodeId>,
    /// Declared subtype indication of the external name.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Whether the target is a shared variable.
    #[serde(default)]
    pub shared_flag: bool,
}

/// Parenthesis name (`prefix(…)` before it is rewritten as an indexed name,
/// slice, function call, or type conversion).
///
/// After analysis most parenthesis names are replaced by a more specific node;
/// remaining ones still appear in incompletely rewritten trees.
///
/// ```vhdl
/// foo(1)          -- may become IndexedName or FunctionCall
/// integer(expr)   -- type conversion
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ParenthesisName {
    /// Prefix before the parentheses.
    pub prefix: Option<PrefixNodeId>,
    /// Association list inside the parentheses.
    #[serde(default)]
    pub associations: Vec<AssociationElementNodeId>,
    /// Suffix expression when GHDL stores a single suffix instead of associations.
    pub suffix: Option<ExpressionNodeId>,
    /// Resolved named entity, when applicable.
    pub named_entity: Option<NamedEntityNodeId>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// Package pathname root (`@library.package.…`) for an external name.
///
/// ```vhdl
/// << constant @work.pkg.C : integer >>
/// -- PackagePathname.identifier = work (or library), suffix continues the path
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PackagePathname {
    /// Library or package identifier at this pathname segment.
    pub identifier: Option<Identifier>,
    /// Remaining pathname suffix.
    pub pathname_suffix: Option<GenericNodeId>,
    /// Named entity bound at this segment, when known.
    pub named_entity: Option<NamedEntityNodeId>,
}

/// Absolute pathname root (`.…`) for an external name.
///
/// ```vhdl
/// << signal .tb.dut.clk : std_logic >>
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AbsolutePathname {
    /// First pathname element after the leading `.`.
    pub pathname_suffix: Option<GenericNodeId>,
}

/// Relative pathname root (`^.…` or `^.^.…`) for an external name.
///
/// ```vhdl
/// << signal ^.sibling.data : std_logic_vector >>
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RelativePathname {
    /// Pathname continuing after the `^` segment(s).
    pub pathname_suffix: Option<GenericNodeId>,
}

/// One segment of an external pathname (`label` or `label(index)`).
///
/// ```vhdl
/// .tb.gen(0).cell.clk
/// -- PathnameElement nodes for tb, gen (with pathname_expression = 0), cell, clk
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PathnameElement {
    /// Pathname segment identifier (instance label, generate label, …).
    pub identifier: Option<Identifier>,
    /// Next pathname segment.
    pub pathname_suffix: Option<GenericNodeId>,
    /// Named entity bound at this segment, when known.
    pub named_entity: Option<NamedEntityNodeId>,
    /// Optional index expression for generate / array instance labels.
    pub pathname_expression: Option<ExpressionNodeId>,
}

/// Box name (`<>`) used as an association actual for an interface default.
///
/// ```vhdl
/// generic map (WIDTH => <>)
/// function "=" (l, r : T) return boolean is <>
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BoxName {}
