use std::any::type_name;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::num::NonZeroU32;

use serde::Deserialize;
use serde::Deserializer;

use super::*;

pub(crate) type IdPrimitive = NonZeroU32;

#[derive(PartialOrd, Ord)]
pub struct NodeId<T>(pub(crate) IdPrimitive, pub(crate) PhantomData<T>);

impl<'de, T> Deserialize<'de> for NodeId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = IdPrimitive::deserialize(deserializer)?;
        Ok(Self(id, PhantomData))
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
        #[derive(Deserialize)]
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
                fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
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
    // Libraries, design files, and design units
    #[serde(rename = "library_declaration")]
    Library,
    DesignFile,
    DesignUnit,
    ConfigurationDeclaration,
    ContextDeclaration,
    EntityDeclaration,
    PackageDeclaration,
    PackageInstantiationDeclaration,
    ArchitectureBody,
    PackageBody,

    // Context items
    LibraryClause,
    UseClause,

    // Declarations
    UnitDeclaration,
    ConstantDeclaration,
    InterfaceConstantDeclaration,
    SignalDeclaration,
    InterfaceSignalDeclaration,
    #[serde(rename = "variable_declaration", alias = "interface_variable_declaration")]
    VariableDeclaration,

    NonObjectAliasDeclaration,
    Signature,

    // Concurrent statements
    ProcessStatement,

    // Sequential statements
    ProcedureCallStatement,
    ReportStatement,
    ReturnStatement,
    SimpleSignalAssignmentStatement,
    VariableAssignmentStatement,
    WaitStatement,
    WaveformElement,

    // Expressions
    UnaryOperator,
    BinaryOperator,
    #[serde(rename = "procedure_call", alias = "function_call")]
    SubprogramCall,
    FloatingPointLiteral,
    IntegerLiteral,
    PhysicalIntLiteral,
    OverflowLiteral,
    #[serde(rename = "string_literal8")]
    StringLiteral,
    RangeExpression,
    Aggregate,
    AssociationElementByExpression,
    EnumerationLiteral,

    // Names
    AttributeName,
    IndexedName,
    ReferenceName,
    SelectedByAllName,
    SelectedName,
    SimpleName,
    SliceName,

    // Subprograms
    #[serde(rename = "procedure_declaration", alias = "function_declaration")]
    SubprogramDeclaration,
    #[serde(rename = "procedure_body", alias = "function_body")]
    SubprogramBody,

    // Types
    IntegerTypeDefinition,
    IntegerSubtypeDefinition,
    FloatingTypeDefinition,
    FloatingSubtypeDefinition,
    ArrayTypeDefinition,
    ArraySubtypeDefinition,
    WildcardTypeDefinition,
    PhysicalTypeDefinition,
    PhysicalSubtypeDefinition,
    EnumerationTypeDefinition,
    EnumerationSubtypeDefinition,
    AccessTypeDefinition,
    FileTypeDefinition,
    FileDefinition,

    TypeDeclaration,
    AnonymousTypeDeclaration,
    SubtypeDeclaration,

    // Miscellaneous
    PackageHeader,
    InterfacePackageDeclaration,
    AttributeDeclaration,
    InterfaceFileDeclaration,
    SuspendStateStatement,
    SuspendStateDeclaration,
    Error,
    Attribute,
    AttributeValue,
    ArrayElementResolution,
    OverloadList,

    CharacterLiteral,
    ChoiceByExpression,
    ImplicitDereference,
    EqualityOperator,
    IfStatement,
    Elsif,
    ChoiceByNone,
    CaseStatement,
    AssertionStatement,
    ChoiceByOthers,
    AndOperator,
    OrOperator,
    ChoiceByRange,
    InequalityOperator,
    ExitStatement,
    SelectedElement,
    NullStatement,
    Dereference,
    LessThanOperator,
    IteratorDeclaration,
    ForLoopStatement,
    MultiplicationOperator,
    QualifiedExpression,
    DivisionOperator,
    NullLiteral,
    AllocatorByExpression,
    AggregateInfo,
    NegationOperator,
    NotOperator,
    LessThanOrEqualOperator,
    AllocatorBySubtype,
    WhileLoopStatement,
    GreaterThanOperator,
    ElementDeclaration,
    AttributeSpecification,
    FileDeclaration,
    GreaterThanOrEqualOperator,
    AbsoluteOperator,
    ExponentiationOperator,
    AssociationElementByName,
    RecordTypeDefinition,
    RemainderOperator,
    ObjectAliasDeclaration,
    SensitizedProcessStatement,
    ConcurrentAssertionStatement,
    ConcurrentSimpleSignalAssignment,
    ConcatenationOperator,
    ComponentInstantiationStatement,
    BindingIndication,
    ComponentDeclaration,
    ChoiceByName,
    BlockConfiguration,
    EntityAspectEntity,
    ConfigurationSpecification,
    ComponentConfiguration,
    XorOperator,
    ModulusOperator,
    NandOperator,
    NorOperator,
    BlockStatement,
    EntityAspectConfiguration,
    IdentityOperator,
    PhysicalFpLiteral,
    RecordSubtypeDefinition,
    SimpleAggregate,
    TypeConversion,
    AssociationElementOpen,
    AssociationElementPackage,
    GenerateStatementBody,
    BlockHeader,
    ForGenerateStatement,
    ConcurrentProcedureCallStatement,
    ConditionalWaveform,
    NextStatement,
    GuardSignalDeclaration,
    AssociationElementByIndividual,
    OperatorSymbol,
    ParenthesisExpression,
    IfGenerateStatement,
    ConcurrentSelectedSignalAssignment,
    DisconnectionSpecification,
    AttributeImplicitDeclaration,
    ConcurrentConditionalSignalAssignment,
    EntityAspectOpen,
    AccessSubtypeDefinition,
    IncompleteTypeDefinition,
}

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
                $variant(&'ast $type),
            )+
        }

        impl<'ast> TryFrom<&'ast $crate::nodes::Node> for $name<'ast> {
            type Error = $crate::nodes::TryFromNodeError;

            fn try_from(value: &'ast $crate::nodes::Node) -> ::std::result::Result<Self, Self::Error> {
                match value {
                    $(
                        $crate::nodes::Node::$type(inner) => Ok(Self::$variant(inner)),
                    )+
                    _ => Err($crate::nodes::TryFromNodeError {
                        actual: value.type_str(),
                        expected: stringify!($name),
                    }),
                }
            }
        }

        #[derive(Clone, Copy, Hash, PartialEq, Eq)]
        pub struct $name_id($crate::nodes::IdPrimitive);

        impl $name_id {
            #[expect(dead_code, reason = "generated code")]
            #[must_use]
            pub(crate) fn new(id: $crate::nodes::IdPrimitive) -> Self {
                Self(id)
            }
        }

        impl $crate::nodes::AstNodeId for $name_id {
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

        impl Default for $name_id {
            fn default() -> Self {
                Self($crate::nodes::IdPrimitive::new(1).unwrap_or_else(|| unreachable!()))
            }
        }

        impl From<$name_id> for $crate::nodes::GenericNodeId {
            fn from(value: $name_id) -> Self {
                Self(value.0, ::std::marker::PhantomData)
            }
        }

        impl From<&$name_id> for $crate::nodes::GenericNodeId {
            fn from(value: &$name_id) -> Self {
                Self(value.0, ::std::marker::PhantomData)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name_id {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let id = $crate::nodes::IdPrimitive::deserialize(deserializer)?;
                Ok(Self(id))
            }
        }

        $(
            impl $crate::nodes::DowncastNodeId<$type> for $name_id {}
        )+

        $(
            impl From<NodeId<$type>> for $name_id {
                fn from(value: NodeId<$type>) -> Self {
                    Self(value.0)
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
