use std::any::type_name;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::num::NonZeroU32;

use super::*;

pub type IdPrimitive = NonZeroU32;

#[derive(PartialOrd, Ord)]
pub struct NodeId<T>(pub(crate) IdPrimitive, pub(crate) PhantomData<T>);

impl<T> NodeId<T> {
    /// Creates a node ID from a non-zero raw index.
    #[must_use]
    pub const fn from_raw(id: IdPrimitive) -> Self {
        Self(id, PhantomData)
    }

    #[must_use]
    pub fn to_raw(self) -> IdPrimitive {
        self.0
    }
}

impl<'de, T> Deserialize<'de> for NodeId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = IdPrimitive::deserialize(deserializer)?;
        Ok(Self(id, PhantomData))
    }
}

impl<T> Serialize for NodeId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

/// Deserializes to an optional AST node ID, where `0` is treated as `None`.
///
/// # Errors
///
/// Returns an error if deserialization to `u32` fails.
pub fn deserialize_optional_node_id<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: From<IdPrimitive>,
{
    let id = u32::deserialize(deserializer)?;
    Ok(IdPrimitive::new(id).map(T::from))
}

impl<T> From<IdPrimitive> for NodeId<T> {
    fn from(id: IdPrimitive) -> Self {
        Self(id, PhantomData)
    }
}

// this manual implementation is required to get rid of the `T: PartialEq` trait bound which a derived implementation would imply
impl<T> PartialEq for NodeId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

// this manual implementation is required to get rid of the `T: Eq` trait bound which a derived implementation would imply
impl<T> Eq for NodeId<T> {}

// this manual implementation is required to get rid of the `T: Hash` trait bound which a derived implementation would imply
impl<T> Hash for NodeId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

// this manual implementation is required to get rid of the `T: Clone` trait bound which a derived implementation would imply
impl<T> Clone for NodeId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

// this manual implementation is required to get rid of the `T: Copy` trait bound which a derived implementation would imply
impl<T> Copy for NodeId<T> {}

impl<T> Default for NodeId<T> {
    fn default() -> Self {
        Self(
            IdPrimitive::new(1).unwrap_or_else(|| unreachable!()),
            PhantomData,
        )
    }
}

pub type GenericNodeId = NodeId<Node>;

pub trait AstNodeId: Into<GenericNodeId> {
    type NodeType<'ast>;

    #[track_caller]
    fn get<'ast>(self, ast: &'ast Ast) -> Self::NodeType<'ast>
    where
        Self::NodeType<'ast>: TryFrom<&'ast Node>,
    {
        #[expect(
            clippy::unwrap_used,
            reason = "this may only happen on programming errors or invalid input from GHDL"
        )]
        self.try_get(ast).unwrap()
    }

    /// Tries to get the node of the expected type from the AST.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the node is not found or is not of the expected type.
    fn try_get<'ast>(self, ast: &'ast Ast) -> Result<Self::NodeType<'ast>, LookupNodeError>
    where
        Self::NodeType<'ast>: TryFrom<&'ast Node>,
    {
        let id: GenericNodeId = self.into();
        let index = id.0.get() as usize;
        match ast.nodes.get(index) {
            Some(Node::Empty) | None => {
                Err(LookupNodeError::not_found::<Self::NodeType<'ast>>(id.0))
            },
            Some(node) => node.try_into().map_err(|_ignore| {
                LookupNodeError::wrong_type::<Self::NodeType<'ast>>(id.0, node.type_str())
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum LookupNodeError {
    #[error("node #{id} not found in AST; expected {expected}")]
    NotFound {
        id: IdPrimitive,
        expected: &'static str,
    },

    #[error("node #{id} is of type {actual}; expected {expected}")]
    WrongType {
        id: IdPrimitive,
        expected: &'static str,
        actual: &'static str,
    },
}

impl LookupNodeError {
    #[must_use]
    fn not_found<T>(id: IdPrimitive) -> Self {
        Self::NotFound {
            id,
            expected: type_name::<T>(),
        }
    }

    #[must_use]
    fn wrong_type<T>(id: IdPrimitive, actual: &'static str) -> Self {
        Self::WrongType {
            id,
            expected: type_name::<T>(),
            actual,
        }
    }
}

pub trait DowncastNodeId<T>: Into<GenericNodeId> {
    fn downcast(self) -> NodeId<T> {
        NodeId(self.into().0, PhantomData)
    }
}

impl AstNodeId for GenericNodeId {
    type NodeType<'ast> = Node;
}

impl<T: 'static> AstNodeId for &NodeId<T>
where
    for<'a> &'a NodeId<T>: Into<GenericNodeId>,
{
    type NodeType<'ast> = &'ast T;
}

impl<T> Display for NodeId<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        Display::fmt(&self.0.get(), formatter)
    }
}

impl Debug for GenericNodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(formatter, "NodeId({index})", index = self.0.get())
    }
}

macro_rules! node_declaration {
    (
        $(
            $(#[$($variant_attr:meta),*])?
            $variant:ident
        ),+ $(,)?
    ) => {
        #[derive(Deserialize, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub enum Node {
            $(
                $($(#[$variant_attr])*)?
                $variant($variant),
            )+

            Empty,
        }

        impl Node {
            #[must_use]
            pub fn type_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant(..) => stringify!($variant),
                    )+
                    Self::Empty => "<empty>",
                }
            }

        }

        impl ::std::fmt::Debug for Node {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        Self::$variant(inner) => ::std::fmt::Debug::fmt(inner, formatter),
                    )+
                    Self::Empty => formatter.write_str("<empty>"),
                }
            }
        }

        $(
            impl<'ast> TryFrom<&'ast Node> for &'ast $variant {
                type Error = ();

                fn try_from(value: &'ast Node) -> Result<Self, Self::Error> {
                    match value {
                        Node::$variant(val) => Ok(val),
                        _ => Err(()),
                    }
                }
            }

            impl TryFrom<Node> for $variant {
                type Error = ();

                fn try_from(value: Node) -> Result<Self, Self::Error> {
                    match value {
                        Node::$variant(val) => Ok(val),
                        _ => Err(()),
                    }
                }
            }

            impl From<NodeId<$variant>> for GenericNodeId {
                fn from(value: NodeId<$variant>) -> Self {
                    Self(value.0, PhantomData)
                }
            }

            impl From<&NodeId<$variant>> for GenericNodeId {
                fn from(value: &NodeId<$variant>) -> Self {
                    Self(value.0, PhantomData)
                }
            }

            impl ::std::fmt::Debug for NodeId<$variant> {
                fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                    write!(
                        formatter,
                        concat!("NodeId<", stringify!($variant), ">({index})"),
                        index = self.0.get(),
                    )
                }
            }

        )+
    };
}

node_declaration! {
    // Associations
    AssociationElementByExpression,
    AssociationElementByIndividual,
    AssociationElementByName,
    AssociationElementOpen,
    AssociationElementPackage,
    AssociationElementSubprogram,
    AssociationElementTerminal,
    AssociationElementType,

    // Attributes
    Attribute,
    AttributeValue,

    // Choices
    ChoiceByExpression,
    ChoiceByName,
    ChoiceByNone,
    ChoiceByOthers,
    ChoiceByRange,

    // Common
    Error,
    OverloadList,

    // Concurrent Statements
    BlockStatement,
    CaseGenerateStatement,
    ComponentInstantiationStatement,
    ConcurrentAssertionStatement,
    ConcurrentBreakStatement,
    ConcurrentConditionalSignalAssignment,
    ConcurrentProcedureCallStatement,
    ConcurrentSelectedSignalAssignment,
    ConcurrentSimpleSignalAssignment,
    ForGenerateStatement,
    GenerateStatementBody,
    IfGenerateElseClause,
    IfGenerateStatement,
    ProcessStatement,
    SensitizedProcessStatement,
    SimpleSimultaneousStatement,
    SimultaneousCaseStatement,
    SimultaneousElsif,
    SimultaneousIfStatement,
    SimultaneousNullStatement,
    SimultaneousProceduralStatement,

    // Configuration
    BindingIndication,
    BlockConfiguration,
    BlockHeader,
    ComponentConfiguration,
    EntityAspectConfiguration,
    EntityAspectEntity,
    EntityAspectOpen,

    // Declarations
    AcrossQuantityDeclaration,
    AnonymousTypeDeclaration,
    ArrayElementResolution,
    AttributeDeclaration,
    AttributeImplicitDeclaration,
    ComponentDeclaration,
    ConstantDeclaration,
    ElementDeclaration,
    FileDeclaration,
    FreeQuantityDeclaration,
    FunctionBody,
    FunctionDeclaration,
    FunctionInstantiationDeclaration,
    GroupDeclaration,
    GroupTemplateDeclaration,
    GuardSignalDeclaration,
    InterfaceConstantDeclaration,
    InterfaceFileDeclaration,
    InterfaceFunctionDeclaration,
    InterfaceProcedureDeclaration,
    InterfaceQuantityDeclaration,
    InterfaceSignalDeclaration,
    InterfaceTerminalDeclaration,
    InterfaceTypeDeclaration,
    InterfaceVariableDeclaration,
    InterfaceViewDeclaration,
    IteratorDeclaration,
    ModeViewDeclaration,
    NatureDeclaration,
    NatureElementDeclaration,
    NoiseQuantityDeclaration,
    NonObjectAliasDeclaration,
    ObjectAliasDeclaration,
    ProcedureBody,
    ProcedureDeclaration,
    ProcedureInstantiationDeclaration,
    SignalDeclaration,
    Signature,
    SpectrumQuantityDeclaration,
    SubnatureDeclaration,
    SubprogramInstantiationBody,
    SubtypeDeclaration,
    SuspendStateDeclaration,
    TerminalDeclaration,
    ThroughQuantityDeclaration,
    TypeDeclaration,
    UnitDeclaration,
    VariableDeclaration,

    // Expressions
    Aggregate,
    AggregateInfo,
    AllocatorByExpression,
    AllocatorBySubtype,
    BinaryOperator,
    CharacterLiteral,
    EnumerationLiteral,
    FloatingPointLiteral,
    FunctionCall,
    IntegerLiteral,
    NullLiteral,
    OverflowLiteral,
    ParenthesisExpression,
    PhysicalFpLiteral,
    PhysicalIntLiteral,
    QualifiedExpression,
    RangeExpression,
    SimpleAggregate,
    #[serde(rename = "string_literal8")]
    StringLiteral,
    TypeConversion,
    UnaryOperator,

    // Libraries
    ArchitectureBody,
    ConfigurationDeclaration,
    ContextDeclaration,
    ContextReference,
    DesignFile,
    DesignUnit,
    EntityDeclaration,
    ForeignModule,
    InterfacePackageDeclaration,
    #[serde(rename = "library_declaration")]
    Library,
    LibraryClause,
    PackageBody,
    PackageDeclaration,
    PackageHeader,
    PackageInstantiationBody,
    PackageInstantiationDeclaration,
    UseClause,
    VmodeDeclaration,
    VpropDeclaration,
    VunitDeclaration,

    // Names
    AbsolutePathname,
    AttributeName,
    BoxName,
    Dereference,
    ExternalConstantName,
    ExternalSignalName,
    ExternalVariableName,
    ImplicitDereference,
    IndexedName,
    OperatorSymbol,
    PackagePathname,
    ParenthesisName,
    PathnameElement,
    ReferenceName,
    RelativePathname,
    SelectedByAllName,
    SelectedElement,
    SelectedName,
    SimpleName,
    SliceName,

    // PSL
    PslAssertDirective,
    PslAssumeDirective,
    PslBooleanParameter,
    PslCoverDirective,
    PslDeclaration,
    PslDefaultClock,
    PslEndpointDeclaration,
    PslExpression,
    PslFell,
    PslHierarchicalName,
    PslInheritSpec,
    PslOnehot,
    PslOnehot0,
    PslPrev,
    PslRestrictDirective,
    PslRose,
    PslStable,

    // Sequential Statements
    AssertionStatement,
    BreakElement,
    BreakStatement,
    CaseStatement,
    ConditionalSignalAssignmentStatement,
    ConditionalVariableAssignmentStatement,
    Elsif,
    ExitStatement,
    ForLoopStatement,
    IfStatement,
    NextStatement,
    NullStatement,
    ProcedureCall,
    ProcedureCallStatement,
    ReportStatement,
    ReturnStatement,
    SelectedVariableAssignmentStatement,
    SelectedWaveformAssignmentStatement,
    SignalForceAssignmentStatement,
    SignalReleaseAssignmentStatement,
    SimpleSignalAssignmentStatement,
    SuspendStateStatement,
    VariableAssignmentStatement,
    WaitStatement,
    WhileLoopStatement,

    // Specifications
    AttributeSpecification,
    ConfigurationSpecification,
    DisconnectionSpecification,
    EntityClass,
    StepLimitSpecification,

    // Types
    AccessSubtypeDefinition,
    AccessTypeDefinition,
    ArrayModeViewElement,
    ArrayModeViewIndication,
    ArrayNatureDefinition,
    ArraySubnatureDefinition,
    ArraySubtypeDefinition,
    ArrayTypeDefinition,
    EnumerationSubtypeDefinition,
    EnumerationTypeDefinition,
    FileDefinition,
    FileSubtypeDefinition,
    FileTypeDefinition,
    FloatingSubtypeDefinition,
    FloatingTypeDefinition,
    ForeignVectorTypeDefinition,
    IncompleteTypeDefinition,
    IntegerSubtypeDefinition,
    IntegerTypeDefinition,
    InterfaceTypeDefinition,
    PhysicalSubtypeDefinition,
    PhysicalTypeDefinition,
    ProtectedTypeBody,
    ProtectedTypeDeclaration,
    RecordElementConstraint,
    RecordElementResolution,
    RecordModeViewElement,
    RecordModeViewIndication,
    RecordNatureDefinition,
    RecordResolution,
    RecordSubtypeDefinition,
    RecordTypeDefinition,
    ScalarNatureDefinition,
    SimpleModeViewElement,
    WildcardTypeDefinition,

    // Waveforms
    ConditionalExpression,
    ConditionalWaveform,
    UnaffectedWaveform,
    WaveformElement,
}

/// Declares a typed subset of [`Node`] variants and a corresponding node ID type.
///
/// # Example
///
/// ```ignore
/// ghdl_ast::subset_declaration!(MySubset MySubsetNodeId {
///     SimpleName(SimpleName),
///     SelectedName(SelectedName),
/// });
/// ```
#[macro_export]
macro_rules! subset_declaration {
    ( $name:ident $name_id:ident {
        $(
            $(#[$variant_attr:meta])*
            $variant:ident($type:ident)
        ),+ $(,)?
    } ) => {
        #[derive(Clone, Copy, Debug)]
        pub enum $name<'ast> {
            $(
                $(#[$variant_attr])*
                $variant(&'ast $crate::$type),
            )+
        }

        impl<'ast> ::std::convert::TryFrom<&'ast $crate::Node> for $name<'ast> {
            type Error = $crate::TryFromNodeError;

            fn try_from(value: &'ast $crate::Node) -> ::std::result::Result<Self, Self::Error> {
                match value {
                    $(
                        $crate::Node::$type(inner) => Ok(Self::$variant(inner)),
                    )+
                    _ => Err($crate::TryFromNodeError {
                        actual: value.type_str(),
                        expected: stringify!($name),
                    }),
                }
            }
        }

        #[derive(Clone, Copy, Hash, PartialEq, Eq)]
        pub struct $name_id($crate::IdPrimitive);

        impl $name_id {
            #[expect(dead_code, reason = "generated code")]
            #[must_use]
            pub(crate) fn new(id: $crate::IdPrimitive) -> Self {
                Self(id)
            }
        }

        impl ::std::convert::From<$crate::IdPrimitive> for $name_id {
            fn from(id: $crate::IdPrimitive) -> Self {
                Self(id)
            }
        }

        impl $crate::AstNodeId for $name_id {
            type NodeType<'ast> = $name<'ast>;
        }

        impl ::std::fmt::Display for $name_id {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0.get(), formatter)
            }
        }

        impl ::std::fmt::Debug for $name_id {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    formatter,
                    "{typ}({index})",
                    typ = stringify!($name_id),
                    index = self.0.get(),
                )
            }
        }

        impl ::std::default::Default for $name_id {
            fn default() -> Self {
                Self($crate::IdPrimitive::new(1).unwrap_or_else(|| unreachable!()))
            }
        }

        impl ::std::convert::From<$name_id> for $crate::GenericNodeId {
            fn from(value: $name_id) -> Self {
                $crate::NodeId::from_raw(value.0)
            }
        }

        impl ::std::convert::From<&$name_id> for $crate::GenericNodeId {
            fn from(value: &$name_id) -> Self {
                $crate::NodeId::from_raw(value.0)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name_id {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                let id = $crate::IdPrimitive::deserialize(deserializer)?;
                Ok(Self(id))
            }
        }

        impl ::serde::Serialize for $name_id {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                self.0.serialize(serializer)
            }
        }

        $(
            impl $crate::DowncastNodeId<$crate::$type> for $name_id {}
        )+

        $(
            impl ::std::convert::From<$crate::NodeId<$crate::$type>> for $name_id {
                fn from(value: $crate::NodeId<$crate::$type>) -> Self {
                    Self(value.to_raw())
                }
            }
        )+
    };
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("node is of type {actual}; expected {expected}")]
pub struct TryFromNodeError {
    pub actual: &'static str,
    pub expected: &'static str,
}
