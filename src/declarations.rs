use super::*;

subset_declaration!(Declaration DeclarationOwned DeclarationNodeId {
    Attribute(AttributeDeclaration),
    Subtype(SubtypeDeclaration),
    Type(TypeDeclaration),
    AnonymousType(AnonymousTypeDeclaration),

    Function(FunctionDeclaration),
    FunctionBody(FunctionBody),
    Procedure(ProcedureDeclaration),
    ProcedureBody(ProcedureBody),

    Constant(ConstantDeclaration),
    Signal(SignalDeclaration),
    Variable(VariableDeclaration),

    NonObjectAlias(NonObjectAliasDeclaration),

    SuspendState(SuspendStateDeclaration),
});

subset_declaration!(InterfaceDeclaration InterfaceDeclarationOwned InterfaceDeclarationNodeId {
    Constant(InterfaceConstantDeclaration),
    Variable(InterfaceVariableDeclaration),
    Signal(InterfaceSignalDeclaration),
    View(InterfaceViewDeclaration),
    File(InterfaceFileDeclaration),
    Terminal(InterfaceTerminalDeclaration),
    Quantity(InterfaceQuantityDeclaration),
    Type(InterfaceTypeDeclaration),
    Package(InterfacePackageDeclaration),
    Function(InterfaceFunctionDeclaration),
    Procedure(InterfaceProcedureDeclaration),
});

subset_declaration!(InterfaceObjectDeclaration InterfaceObjectDeclarationOwned InterfaceObjectDeclarationNodeId {
    Constant(InterfaceConstantDeclaration),
    Variable(InterfaceVariableDeclaration),
    Signal(InterfaceSignalDeclaration),
    View(InterfaceViewDeclaration),
    File(InterfaceFileDeclaration),
});

subset_declaration!(PortInterfaceDeclaration PortInterfaceDeclarationOwned PortInterfaceDeclarationNodeId {
    Signal(InterfaceSignalDeclaration),
    View(InterfaceViewDeclaration),
    Terminal(InterfaceTerminalDeclaration),
    Quantity(InterfaceQuantityDeclaration),
});

subset_declaration!(ObjectDeclaration ObjectDeclarationOwned ObjectDeclarationNodeId {
    ObjectAlias(ObjectAliasDeclaration),
    File(FileDeclaration),
    GuardSignal(GuardSignalDeclaration),
    Signal(SignalDeclaration),
    Variable(VariableDeclaration),
    Constant(ConstantDeclaration),
    Iterator(IteratorDeclaration),
    InterfaceConstant(InterfaceConstantDeclaration),
    InterfaceVariable(InterfaceVariableDeclaration),
    InterfaceSignal(InterfaceSignalDeclaration),
    InterfaceView(InterfaceViewDeclaration),
    InterfaceFile(InterfaceFileDeclaration),
});

subset_declaration!(FunctionImplementation FunctionImplementationOwned FunctionImplementationNodeId {
    Function(FunctionDeclaration),
    InterfaceFunction(InterfaceFunctionDeclaration),
    FunctionInstantiation(FunctionInstantiationDeclaration),
});

subset_declaration!(ProcedureImplementation ProcedureImplementationOwned ProcedureImplementationNodeId {
    Procedure(ProcedureDeclaration),
    InterfaceProcedure(InterfaceProcedureDeclaration),
    ProcedureInstantiation(ProcedureInstantiationDeclaration),
});

subset_declaration!(SubprogramBody SubprogramBodyOwned SubprogramBodyNodeId {
    Function(FunctionBody),
    Procedure(ProcedureBody),
});

/// Subtype declaration (`subtype … is …`).
///
/// ```vhdl
/// subtype byte is std_logic_vector(7 downto 0);
/// subtype natural is integer range 0 to integer'high;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SubtypeDeclaration {
    /// Subtype identifier.
    pub identifier: Identifier,
    /// Subtype indication after the `is` keyword.
    pub subtype_indication: SubtypeDefinitionNodeId,
}

/// Anonymous type declaration synthesized with a first subtype.
///
/// Integer/floating/physical/array type declarations are represented as an
/// anonymous base type plus a named subtype; this node holds that pairing.
///
/// ```vhdl
/// type my_int is range 0 to 255;
/// -- AnonymousTypeDeclaration owns the IntegerTypeDefinition;
/// -- a SubtypeDeclaration / IntegerSubtypeDefinition is the named object.
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AnonymousTypeDeclaration {
    /// Anonymous base type definition.
    pub type_definition: AnonymousTypeDefinitionNodeId,
    /// First subtype of the anonymous type, when present.
    pub subtype_definition: Option<SubtypeDefinitionNodeId>,
}

/// Type declaration (`type … is …` / incomplete `type …;`).
///
/// ```vhdl
/// type state_t is (IDLE, RUN, DONE);
/// type ptr_t is access token_t;
/// type token_t;  -- incomplete
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct TypeDeclaration {
    /// Type identifier.
    pub identifier: Identifier,
    /// Type definition (`is …`), or incomplete when deferred.
    pub type_definition: TypeDefinitionNodeId,
}

/// Attribute declaration (`attribute … : …`).
///
/// Declares an attribute designator of a given type; values are supplied later
/// by attribute specifications.
///
/// ```vhdl
/// attribute keep : boolean;
/// attribute max_delay : time;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeDeclaration {
    /// Attribute identifier.
    pub identifier: Identifier,
    /// Analyzed attribute type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Type mark from the declaration.
    pub type_mark: Option<NameNodeId>,
}

/// Interface file declaration in a parameter list (`file f : …`).
///
/// ```vhdl
/// procedure dump(file f : text);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceFileDeclaration {
    /// File parameter identifier.
    pub identifier: Identifier,
    /// Analyzed file type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Subtype indication of the file type mark.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Mode when explicitly written.
    pub mode: Option<Mode>,
}

/// Secondary unit declaration inside a physical type (`units … end units`).
///
/// ```vhdl
/// type time is range … units
///   fs;
///   ps = 1000 fs;
/// end units;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct UnitDeclaration {
    /// Unit identifier.
    pub identifier: Option<Identifier>,
    /// Physical literal giving this unit in terms of a previously declared unit
    /// (primary units still carry a literal node in GHDL's representation).
    pub physical_literal: PhysicalLiteralNodeId,
    /// Analyzed physical type of the unit.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// Interface constant declaration (generic or `constant` parameter).
///
/// The identifier may be omitted in some GHDL-internal / interface positions;
/// keep [`identifier`](Self::identifier) as `Option`.
///
/// ```vhdl
/// generic (WIDTH : positive := 8);
/// procedure p(constant n : natural);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceConstantDeclaration {
    /// Constant interface identifier (`None` in rare unnamed positions).
    pub identifier: Option<Identifier>,

    /// Analyzed subtype of the interface constant.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
    /// Subtype indication as written / analyzed.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Mode (`in` by default for constants; may be unknown).
    pub mode: Option<Mode>,
    /// Default expression when present.
    pub default_value: Option<ExpressionNodeId>,
}

/// Constant declaration (`constant … : … := …`).
///
/// Deferred constants omit the value in a package declaration and complete it
/// in the package body ([`deferred_declaration`](Self::deferred_declaration)).
///
/// ```vhdl
/// constant WIDTH : natural := 8;
/// -- package: constant C : integer;        -- deferred
/// -- body:    constant C : integer := 42;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ConstantDeclaration {
    /// Constant identifier.
    pub identifier: Identifier,

    /// Analyzed subtype of the constant.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
    /// Subtype indication as written / analyzed.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Initial / deferred value expression.
    pub default_value: Option<ExpressionNodeId>,
    /// Whether this is the deferred declaration (no value yet).
    #[serde(default)]
    pub deferred_declaration_flag: bool,
    /// Link between deferred declaration and its completion.
    pub deferred_declaration: Option<NodeId<ConstantDeclaration>>,
}

/// Interface signal declaration (port or `signal` parameter).
///
/// ```vhdl
/// port (clk : in std_logic; q : out std_logic := '0');
/// procedure sample(signal s : in std_logic);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceSignalDeclaration {
    /// Signal interface identifier.
    pub identifier: Identifier,

    /// Analyzed subtype of the interface signal.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,

    /// Port / parameter mode.
    pub mode: Mode,
    /// Subtype indication as written / analyzed.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Default expression when present.
    pub default_value: Option<ExpressionNodeId>,
    /// Whether this is a guarded signal interface.
    #[serde(default)]
    pub guarded_signal_flag: bool,
    /// `register` / `bus` kind when guarded.
    pub signal_kind: Option<SignalKind>,
}

/// Signal declaration (`signal … : …`).
///
/// ```vhdl
/// signal clk : std_logic := '0';
/// signal bus_s : std_logic bus;
/// signal reg_s : std_logic register;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SignalDeclaration {
    /// Signal identifier.
    pub identifier: Identifier,

    /// Analyzed subtype of the signal.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
    /// Subtype indication as written / analyzed.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Default / initial value expression.
    pub default_value: Option<ExpressionNodeId>,
    /// Whether this is a guarded signal (`bus` / `register`).
    #[serde(default)]
    pub guarded_signal_flag: bool,
    /// `register` / `bus` kind (meaningful when guarded).
    pub signal_kind: Option<SignalKind>,
}

/// Interface variable declaration (`variable` parameter).
///
/// ```vhdl
/// procedure incr(variable x : inout integer);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceVariableDeclaration {
    /// Variable interface identifier.
    pub identifier: Identifier,

    /// Analyzed subtype of the interface variable.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,

    /// Default expression when present.
    pub default_value: Option<ExpressionNodeId>,
    /// Subtype indication as written / analyzed.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Mode of the variable parameter.
    pub mode: Option<Mode>,
}

/// Variable declaration (`variable …` / `shared variable …`).
///
/// ```vhdl
/// variable i : integer := 0;
/// shared variable counter : shared_counter;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct VariableDeclaration {
    /// Variable identifier.
    pub identifier: Identifier,

    /// Analyzed subtype of the variable.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,

    /// Initial value expression.
    pub default_value: Option<ExpressionNodeId>,
    /// Subtype indication as written / analyzed.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Whether this is a `shared variable`.
    #[serde(default)]
    pub shared_flag: bool,
}

/// Non-object alias declaration (`alias … is …` for types, subprograms, …).
///
/// Object aliases use [`ObjectAliasDeclaration`] instead.
///
/// ```vhdl
/// alias my_vec is std_logic_vector;
/// alias "+" is work.pkg."+" [integer, integer return integer];
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NonObjectAliasDeclaration {
    /// Alias identifier.
    pub identifier: Identifier,
    /// Name being aliased.
    pub name: Option<NameNodeId>,
    /// Optional signature selecting an overloaded subprogram.
    pub alias_signature: Option<NodeId<Signature>>,
}

/// Subprogram signature (`[type_marks return type_mark]`).
///
/// Disambiguates overloaded subprograms in aliases, attributes, and similar.
///
/// ```vhdl
/// alias add is "+" [integer, integer return integer];
/// -- type_marks = [integer, integer], return_type_mark = integer
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    /// Parameter type marks in order.
    #[serde(default, rename = "type_marks_list")]
    pub type_marks: Vec<NameNodeId>,
    /// Return type mark for functions.
    pub return_type_mark: Option<NameNodeId>,
    /// Prefix name the signature is attached to, when present.
    pub signature_prefix: Option<NameNodeId>,
    /// Named entity selected by the signature after resolution.
    pub named_entity: Option<NamedEntityNodeId>,
}

/// Internal suspend-state declaration inserted for sequential suspension.
///
/// Simulation artifact for processes/procedures that can wait; usually not
/// present as user-written VHDL. Linked from [`SuspendStateStatement`] nodes.
#[derive(Debug, Deserialize, Serialize)]
pub struct SuspendStateDeclaration {
    /// Last suspend-state statement in the chain owned by this declaration.
    pub suspend_state_last: Option<NodeId<SuspendStateStatement>>,
}
/// Function declaration (`function … return …`).
///
/// May be a specification only, or linked to a [`FunctionBody`] via
/// [`subprogram_body`](Self::subprogram_body). Predefined operators set
/// [`implicit_definition`](Self::implicit_definition).
///
/// ```vhdl
/// function add(a, b : integer) return integer;
/// pure function id(x : T) return T;
/// impure function rnd return real;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionDeclaration {
    /// Function identifier or operator symbol.
    pub identifier: Identifier,
    /// Predefined operator/function kind when this is an implicit declaration.
    pub implicit_definition: Option<ImplicitDefinition>,
    /// Parameter interface list.
    #[serde(default)]
    pub interface_declarations: Vec<InterfaceDeclarationNodeId>,
    /// Return subtype after analysis.
    pub return_type: Option<SubtypeDefinitionNodeId>,
    /// Function body when present in the same analysis unit.
    pub subprogram_body: Option<NodeId<FunctionBody>>,
    /// Whether the function is pure (`true`) or impure (`false`).
    #[serde(default)]
    pub pure_flag: bool,
    /// Generic interface list for generic functions (VHDL-2008).
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
    /// Return type mark as written.
    pub return_type_mark: Option<NameNodeId>,
}

/// Function body (`function … is … begin … end`).
///
/// ```vhdl
/// function add(a, b : integer) return integer is
/// begin
///   return a + b;
/// end function;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionBody {
    /// Matching function declaration / specification.
    pub subprogram_specification: NodeId<FunctionDeclaration>,
    /// Declarations in the function declarative part.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Sequential statements of the function body.
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
}

/// Procedure declaration (`procedure …`).
///
/// ```vhdl
/// procedure pulse(signal s : out std_logic; t : time);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcedureDeclaration {
    /// Procedure identifier.
    pub identifier: Identifier,
    /// Predefined procedure kind when this is an implicit declaration.
    pub implicit_definition: Option<ImplicitDefinition>,
    /// Parameter interface list.
    #[serde(default)]
    pub interface_declarations: Vec<InterfaceDeclarationNodeId>,
    /// Procedure body when present in the same analysis unit.
    pub subprogram_body: Option<NodeId<ProcedureBody>>,
    /// Generic interface list for generic procedures (VHDL-2008).
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
}

/// Procedure body (`procedure … is … begin … end`).
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcedureBody {
    /// Matching procedure declaration / specification.
    pub subprogram_specification: NodeId<ProcedureDeclaration>,
    /// Declarations in the procedure declarative part.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Sequential statements of the procedure body.
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
}

/// Array element resolution indication (`(resolution_function element_subtype)`).
///
/// ```vhdl
/// type resolved_vec is array (natural range <>) of resolved std_ulogic;
/// -- element resolution may appear in subtype indications as:
/// -- (resolved std_ulogic)
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayElementResolution {
    /// Resolution indication applied to each element.
    pub resolution_indication: Option<GenericNodeId>,
    /// Element subtype indication.
    pub element_subtype_indication: Option<SubtypeDefinitionNodeId>,
}

/// Kind of predefined/implicit operator or function synthesized by analysis.
///
/// When a type is declared, VHDL defines operators (`=`, `+`, …) and often
/// IEEE packages contribute further overloads. GHDL materializes those as
/// function/procedure declarations whose
/// [`implicit_definition`](FunctionDeclaration::implicit_definition) field is
/// set to one of these variants. Explicit user subprograms leave the field
/// unset (`None`).
///
/// Variant names mirror GHDL's `IIR_PREDEFINED_*` images. Each variant's doc
/// shows the corresponding VHDL operator/function and operand kinds.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ImplicitDefinition {
    /// `"="` (`access`)
    #[serde(rename = "IIR_PREDEFINED_ACCESS_EQUALITY")]
    AccessEquality,
    /// `"/="` (`access`)
    #[serde(rename = "IIR_PREDEFINED_ACCESS_INEQUALITY")]
    AccessInequality,
    /// `"&"` (array, array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_ARRAY_CONCAT")]
    ArrayArrayConcat,
    /// `to_string` (`character` array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_CHAR_TO_STRING")]
    ArrayCharToString,
    /// `"&"` (array, element)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_ELEMENT_CONCAT")]
    ArrayElementConcat,
    /// `"="` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_EQUALITY")]
    ArrayEquality,
    /// `">"` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_GREATER")]
    ArrayGreater,
    /// `">="` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_GREATER_EQUAL")]
    ArrayGreaterEqual,
    /// `"/="` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_INEQUALITY")]
    ArrayInequality,
    /// `"<"` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_LESS")]
    ArrayLess,
    /// `"<="` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_LESS_EQUAL")]
    ArrayLessEqual,
    /// `maximum` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_MAXIMUM")]
    ArrayMaximum,
    /// `minimum` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_MINIMUM")]
    ArrayMinimum,
    /// `"rol"` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_ROL")]
    ArrayRol,
    /// `"ror"` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_ROR")]
    ArrayRor,
    /// `"sla"` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_SLA")]
    ArraySla,
    /// `"sll"` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_SLL")]
    ArraySll,
    /// `"sra"` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_SRA")]
    ArraySra,
    /// `"srl"` (array)
    #[serde(rename = "IIR_PREDEFINED_ARRAY_SRL")]
    ArraySrl,
    /// `"and"` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_AND")]
    BitAnd,
    /// `"?="` (`bit` array)
    #[serde(rename = "IIR_PREDEFINED_BIT_ARRAY_MATCH_EQUALITY")]
    BitArrayMatchEquality,
    /// `"?/="` (`bit` array)
    #[serde(rename = "IIR_PREDEFINED_BIT_ARRAY_MATCH_INEQUALITY")]
    BitArrayMatchInequality,
    /// `"??"` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_CONDITION")]
    BitCondition,
    /// `falling_edge` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_FALLING_EDGE")]
    BitFallingEdge,
    /// `"?="` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_EQUALITY")]
    BitMatchEquality,
    /// `"?>"` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_GREATER")]
    BitMatchGreater,
    /// `"?>="` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_GREATER_EQUAL")]
    BitMatchGreaterEqual,
    /// `"?/="` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_INEQUALITY")]
    BitMatchInequality,
    /// `"?<"` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_LESS")]
    BitMatchLess,
    /// `"?<="` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_MATCH_LESS_EQUAL")]
    BitMatchLessEqual,
    /// `"nand"` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_NAND")]
    BitNand,
    /// `"nor"` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_NOR")]
    BitNor,
    /// `"not"` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_NOT")]
    BitNot,
    /// `"or"` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_OR")]
    BitOr,
    /// `rising_edge` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_RISING_EDGE")]
    BitRisingEdge,
    /// `to_hstring` (`bit_vector`)
    #[serde(rename = "IIR_PREDEFINED_BIT_VECTOR_TO_HSTRING")]
    BitVectorToHstring,
    /// `to_ostring` (`bit_vector`)
    #[serde(rename = "IIR_PREDEFINED_BIT_VECTOR_TO_OSTRING")]
    BitVectorToOstring,
    /// `"xnor"` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_XNOR")]
    BitXnor,
    /// `"xor"` (`bit`)
    #[serde(rename = "IIR_PREDEFINED_BIT_XOR")]
    BitXor,
    /// `"and"` (`boolean`)
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_AND")]
    BooleanAnd,
    /// `falling_edge` (`boolean`)
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_FALLING_EDGE")]
    BooleanFallingEdge,
    /// `"nand"` (`boolean`)
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_NAND")]
    BooleanNand,
    /// `"nor"` (`boolean`)
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_NOR")]
    BooleanNor,
    /// `"not"` (`boolean`)
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_NOT")]
    BooleanNot,
    /// `"or"` (`boolean`)
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_OR")]
    BooleanOr,
    /// `rising_edge` (`boolean`)
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_RISING_EDGE")]
    BooleanRisingEdge,
    /// `"xnor"` (`boolean`)
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_XNOR")]
    BooleanXnor,
    /// `"xor"` (`boolean`)
    #[serde(rename = "IIR_PREDEFINED_BOOLEAN_XOR")]
    BooleanXor,
    /// `deallocate` (`access`)
    #[serde(rename = "IIR_PREDEFINED_DEALLOCATE")]
    Deallocate,
    /// `"&"` (element, array)
    #[serde(rename = "IIR_PREDEFINED_ELEMENT_ARRAY_CONCAT")]
    ElementArrayConcat,
    /// `"&"` (element, element)
    #[serde(rename = "IIR_PREDEFINED_ELEMENT_ELEMENT_CONCAT")]
    ElementElementConcat,
    /// `endfile` (`file`)
    #[serde(rename = "IIR_PREDEFINED_ENDFILE")]
    Endfile,
    /// `"="` (enumeration)
    #[serde(rename = "IIR_PREDEFINED_ENUM_EQUALITY")]
    EnumEquality,
    /// `">"` (enumeration)
    #[serde(rename = "IIR_PREDEFINED_ENUM_GREATER")]
    EnumGreater,
    /// `">="` (enumeration)
    #[serde(rename = "IIR_PREDEFINED_ENUM_GREATER_EQUAL")]
    EnumGreaterEqual,
    /// `"/="` (enumeration)
    #[serde(rename = "IIR_PREDEFINED_ENUM_INEQUALITY")]
    EnumInequality,
    /// `"<"` (enumeration)
    #[serde(rename = "IIR_PREDEFINED_ENUM_LESS")]
    EnumLess,
    /// `"<="` (enumeration)
    #[serde(rename = "IIR_PREDEFINED_ENUM_LESS_EQUAL")]
    EnumLessEqual,
    /// `maximum` (enumeration)
    #[serde(rename = "IIR_PREDEFINED_ENUM_MAXIMUM")]
    EnumMaximum,
    /// `minimum` (enumeration)
    #[serde(rename = "IIR_PREDEFINED_ENUM_MINIMUM")]
    EnumMinimum,
    /// `to_string` (enumeration)
    #[serde(rename = "IIR_PREDEFINED_ENUM_TO_STRING")]
    EnumToString,
    /// Error placeholder for a failed implicit declaration.
    #[serde(rename = "IIR_PREDEFINED_ERROR")]
    Error,
    /// `file_close`
    #[serde(rename = "IIR_PREDEFINED_FILE_CLOSE")]
    FileClose,
    /// `file_open`
    #[serde(rename = "IIR_PREDEFINED_FILE_OPEN")]
    FileOpen,
    /// `file_open` (status out)
    #[serde(rename = "IIR_PREDEFINED_FILE_OPEN_STATUS")]
    FileOpenStatus,
    /// `"abs"` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_ABSOLUTE")]
    FloatingAbsolute,
    /// `"/"` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_DIV")]
    FloatingDiv,
    /// `"="` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_EQUALITY")]
    FloatingEquality,
    /// `"**"` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_EXP")]
    FloatingExp,
    /// `">"` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_GREATER")]
    FloatingGreater,
    /// `">="` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_GREATER_EQUAL")]
    FloatingGreaterEqual,
    /// unary `"+"` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_IDENTITY")]
    FloatingIdentity,
    /// `"/="` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_INEQUALITY")]
    FloatingInequality,
    /// `"<"` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_LESS")]
    FloatingLess,
    /// `"<="` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_LESS_EQUAL")]
    FloatingLessEqual,
    /// `maximum` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_MAXIMUM")]
    FloatingMaximum,
    /// `minimum` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_MINIMUM")]
    FloatingMinimum,
    /// `"-"` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_MINUS")]
    FloatingMinus,
    /// `"*"` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_MUL")]
    FloatingMul,
    /// unary `"-"` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_NEGATION")]
    FloatingNegation,
    /// `"+"` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_PLUS")]
    FloatingPlus,
    /// `to_string` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FLOATING_TO_STRING")]
    FloatingToString,
    /// `flush` (`file`)
    #[serde(rename = "IIR_PREDEFINED_FLUSH")]
    Flush,
    /// foreign TEXTIO `read` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FOREIGN_TEXTIO_READ_REAL")]
    ForeignTextioReadReal,
    /// foreign TEXTIO `write` (`real`)
    #[serde(rename = "IIR_PREDEFINED_FOREIGN_TEXTIO_WRITE_REAL")]
    ForeignTextioWriteReal,
    /// foreign untruncated text `read`
    #[serde(rename = "IIR_PREDEFINED_FOREIGN_UNTRUNCATED_TEXT_READ")]
    ForeignUntruncatedTextRead,
    /// `frequency` (AMS)
    #[serde(rename = "IIR_PREDEFINED_FREQUENCY_FUNCTION")]
    FrequencyFunction,
    /// `"and"` (`std_ulogic`, `std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_AND_LOG_SUV")]
    Ieee1164AndLogSuv,
    /// `"and"` (`std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_AND_SUV")]
    Ieee1164AndSuv,
    /// `"and"` (`std_ulogic_vector`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_AND_SUV_LOG")]
    Ieee1164AndSuvLog,
    /// `"??"` (`std_ulogic`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_CONDITION_OPERATOR")]
    Ieee1164ConditionOperator,
    /// `falling_edge` (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_FALLING_EDGE")]
    Ieee1164FallingEdge,
    /// `is_X` (`std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_IS_X_LOG")]
    Ieee1164IsXLog,
    /// `is_X` (`std_logic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_IS_X_SLV")]
    Ieee1164IsXSlv,
    /// `"nand"` (`std_ulogic`, `std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NAND_LOG_SUV")]
    Ieee1164NandLogSuv,
    /// `"nand"` (`std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NAND_SUV")]
    Ieee1164NandSuv,
    /// `"nand"` (`std_ulogic_vector`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NAND_SUV_LOG")]
    Ieee1164NandSuvLog,
    /// `"nor"` (`std_ulogic`, `std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NOR_LOG_SUV")]
    Ieee1164NorLogSuv,
    /// `"nor"` (`std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NOR_SUV")]
    Ieee1164NorSuv,
    /// `"nor"` (`std_ulogic_vector`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_NOR_SUV_LOG")]
    Ieee1164NorSuvLog,
    /// `"or"` (`std_ulogic`, `std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_OR_LOG_SUV")]
    Ieee1164OrLogSuv,
    /// `"or"` (`std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_OR_SUV")]
    Ieee1164OrSuv,
    /// `"or"` (`std_ulogic_vector`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_OR_SUV_LOG")]
    Ieee1164OrSuvLog,
    /// `rising_edge` (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_RISING_EDGE")]
    Ieee1164RisingEdge,
    /// `"and"` (scalar) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_AND")]
    Ieee1164ScalarAnd,
    /// `"nand"` (scalar) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_NAND")]
    Ieee1164ScalarNand,
    /// `"nor"` (scalar) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_NOR")]
    Ieee1164ScalarNor,
    /// `"not"` (scalar) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_NOT")]
    Ieee1164ScalarNot,
    /// `"or"` (scalar) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_OR")]
    Ieee1164ScalarOr,
    /// `"xnor"` (scalar) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_XNOR")]
    Ieee1164ScalarXnor,
    /// `"xor"` (scalar) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_SCALAR_XOR")]
    Ieee1164ScalarXor,
    /// `to_01` (`std_ulogic`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_01_LOG_LOG")]
    Ieee1164To01LogLog,
    /// `to_01` (`std_logic_vector`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_01_SLV_LOG")]
    Ieee1164To01SlvLog,
    /// `to_bit` (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_BIT")]
    Ieee1164ToBit,
    /// `to_bitvector` (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_BITVECTOR")]
    Ieee1164ToBitvector,
    /// `to_hstring` (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_HSTRING")]
    Ieee1164ToHstring,
    /// `to_ostring` (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_OSTRING")]
    Ieee1164ToOstring,
    /// `to_stdlogicvector` (`bit_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_STDLOGICVECTOR_BV")]
    Ieee1164ToStdlogicvectorBv,
    /// `to_stdlogicvector` (`std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_STDLOGICVECTOR_SUV")]
    Ieee1164ToStdlogicvectorSuv,
    /// `to_stdulogic` (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_STDULOGIC")]
    Ieee1164ToStdulogic,
    /// `to_stdulogicvector` (`bit_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_STDULOGICVECTOR_BV")]
    Ieee1164ToStdulogicvectorBv,
    /// `to_stdulogicvector` (`std_logic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_STDULOGICVECTOR_SLV")]
    Ieee1164ToStdulogicvectorSlv,
    /// `to_UX01` (`bit`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_BIT_LOG")]
    Ieee1164ToUx01BitLog,
    /// `to_UX01` (`bit_vector`, `std_logic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_BV_SLV")]
    Ieee1164ToUx01BvSlv,
    /// `to_UX01` (`bit_vector`, `std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_BV_SUV")]
    Ieee1164ToUx01BvSuv,
    /// `to_UX01` (`std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_LOG")]
    Ieee1164ToUx01Log,
    /// `to_UX01` (`std_logic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_SLV")]
    Ieee1164ToUx01Slv,
    /// `to_UX01` (`std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_UX01_SUV")]
    Ieee1164ToUx01Suv,
    /// `to_X01` (`bit`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_BIT_LOG")]
    Ieee1164ToX01BitLog,
    /// `to_X01` (`bit_vector`, `std_logic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_BV_SLV")]
    Ieee1164ToX01BvSlv,
    /// `to_X01` (`bit_vector`, `std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_BV_SUV")]
    Ieee1164ToX01BvSuv,
    /// `to_X01` (`std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_LOG")]
    Ieee1164ToX01Log,
    /// `to_X01` (`std_logic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_SLV")]
    Ieee1164ToX01Slv,
    /// `to_X01` (`std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01_SUV")]
    Ieee1164ToX01Suv,
    /// `to_X01Z` (`bit`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_BIT_LOG")]
    Ieee1164ToX01zBitLog,
    /// `to_X01Z` (`bit_vector`, `std_logic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_BV_SLV")]
    Ieee1164ToX01zBvSlv,
    /// `to_X01Z` (`bit_vector`, `std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_BV_SUV")]
    Ieee1164ToX01zBvSuv,
    /// `to_X01Z` (`std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_LOG")]
    Ieee1164ToX01zLog,
    /// `to_X01Z` (`std_logic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_SLV")]
    Ieee1164ToX01zSlv,
    /// `to_X01Z` (`std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_TO_X01Z_SUV")]
    Ieee1164ToX01zSuv,
    /// `"and"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_AND")]
    Ieee1164VectorAnd,
    /// `"nand"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_NAND")]
    Ieee1164VectorNand,
    /// `"nor"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_NOR")]
    Ieee1164VectorNor,
    /// `"not"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_NOT")]
    Ieee1164VectorNot,
    /// `"or"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_OR")]
    Ieee1164VectorOr,
    /// `"rol"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_ROL")]
    Ieee1164VectorRol,
    /// `"ror"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_ROR")]
    Ieee1164VectorRor,
    /// `"sll"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_SLL")]
    Ieee1164VectorSll,
    /// `"srl"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_SRL")]
    Ieee1164VectorSrl,
    /// `"xnor"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_XNOR")]
    Ieee1164VectorXnor,
    /// `"xor"` (vector) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_VECTOR_XOR")]
    Ieee1164VectorXor,
    /// `"xnor"` (`std_ulogic`, `std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XNOR_LOG_SUV")]
    Ieee1164XnorLogSuv,
    /// `"xnor"` (`std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XNOR_SUV")]
    Ieee1164XnorSuv,
    /// `"xnor"` (`std_ulogic_vector`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XNOR_SUV_LOG")]
    Ieee1164XnorSuvLog,
    /// `"xor"` (`std_ulogic`, `std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XOR_LOG_SUV")]
    Ieee1164XorLogSuv,
    /// `"xor"` (`std_ulogic_vector`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XOR_SUV")]
    Ieee1164XorSuv,
    /// `"xor"` (`std_ulogic_vector`, `std_ulogic`) (`ieee.std_logic_1164`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_1164_XOR_SUV_LOG")]
    Ieee1164XorSuvLog,
    /// `arccos` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCCOS")]
    IeeeMathRealArccos,
    /// `arccosh` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCCOSH")]
    IeeeMathRealArccosh,
    /// `arcsin` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCSIN")]
    IeeeMathRealArcsin,
    /// `arcsinh` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCSINH")]
    IeeeMathRealArcsinh,
    /// `arctan` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCTAN")]
    IeeeMathRealArctan,
    /// `arctan(y, x)` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCTAN_REAL_REAL")]
    IeeeMathRealArctanRealReal,
    /// `arctanh` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ARCTANH")]
    IeeeMathRealArctanh,
    /// `cbrt` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_CBRT")]
    IeeeMathRealCbrt,
    /// `ceil` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_CEIL")]
    IeeeMathRealCeil,
    /// `cos` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_COS")]
    IeeeMathRealCos,
    /// `cosh` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_COSH")]
    IeeeMathRealCosh,
    /// `exp` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_EXP")]
    IeeeMathRealExp,
    /// `floor` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_FLOOR")]
    IeeeMathRealFloor,
    /// `log` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_LOG")]
    IeeeMathRealLog,
    /// `log(x, base)` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_LOG_REAL_REAL")]
    IeeeMathRealLogRealReal,
    /// `log10` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_LOG10")]
    IeeeMathRealLog10,
    /// `log2` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_LOG2")]
    IeeeMathRealLog2,
    /// `mod` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_MOD")]
    IeeeMathRealMod,
    /// `"**"` / `pow` (`integer`, `real`) (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_POW_INT_REAL")]
    IeeeMathRealPowIntReal,
    /// `"**"` / `pow` (`real`, `real`) (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_POW_REAL_REAL")]
    IeeeMathRealPowRealReal,
    /// `realmax` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_REALMAX")]
    IeeeMathRealRealmax,
    /// `realmin` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_REALMIN")]
    IeeeMathRealRealmin,
    /// `round` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_ROUND")]
    IeeeMathRealRound,
    /// `sign` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_SIGN")]
    IeeeMathRealSign,
    /// `sin` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_SIN")]
    IeeeMathRealSin,
    /// `sinh` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_SINH")]
    IeeeMathRealSinh,
    /// `sqrt` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_SQRT")]
    IeeeMathRealSqrt,
    /// `tan` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_TAN")]
    IeeeMathRealTan,
    /// `tanh` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_TANH")]
    IeeeMathRealTanh,
    /// `trunc` (`ieee.math_real`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_MATH_REAL_TRUNC")]
    IeeeMathRealTrunc,
    /// `to_integer` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOINT_SGN_INT")]
    IeeeNumericBitToIntSgnInt,
    /// `to_integer` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOINT_UNS_NAT")]
    IeeeNumericBitToIntUnsNat,
    /// `to_signed` (`integer`, `natural`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOSGN_INT_NAT_SGN")]
    IeeeNumericBitToSgnIntNatSgn,
    /// `to_signed` (`integer`, `signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOSGN_INT_SGN_SGN")]
    IeeeNumericBitToSgnIntSgnSgn,
    /// `to_unsigned` (`natural`, `natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOUNS_NAT_NAT_UNS")]
    IeeeNumericBitToUnsNatNatUns,
    /// `to_unsigned` (`natural`, `unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_TOUNS_NAT_UNS_UNS")]
    IeeeNumericBitToUnsNatUnsUns,
    /// `"abs"` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ABS_SGN")]
    IeeeNumericStdAbsSgn,
    /// `"+"` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_INT_SGN")]
    IeeeNumericStdAddIntSgn,
    /// `"+"` (`std_ulogic`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_LOG_SGN")]
    IeeeNumericStdAddLogSgn,
    /// `"+"` (`std_ulogic`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_LOG_UNS")]
    IeeeNumericStdAddLogUns,
    /// `"+"` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_NAT_UNS")]
    IeeeNumericStdAddNatUns,
    /// `"+"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_SGN_INT")]
    IeeeNumericStdAddSgnInt,
    /// `"+"` (`signed`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_SGN_LOG")]
    IeeeNumericStdAddSgnLog,
    /// `"+"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_SGN_SGN")]
    IeeeNumericStdAddSgnSgn,
    /// `"+"` (`unsigned`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_UNS_LOG")]
    IeeeNumericStdAddUnsLog,
    /// `"+"` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_UNS_NAT")]
    IeeeNumericStdAddUnsNat,
    /// `"+"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ADD_UNS_UNS")]
    IeeeNumericStdAddUnsUns,
    /// `"and"` (`std_ulogic`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_LOG_SGN")]
    IeeeNumericStdAndLogSgn,
    /// `"and"` (`std_ulogic`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_LOG_UNS")]
    IeeeNumericStdAndLogUns,
    /// `"and"` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_SGN")]
    IeeeNumericStdAndSgn,
    /// `"and"` (`signed`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_SGN_LOG")]
    IeeeNumericStdAndSgnLog,
    /// `"and"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_SGN_SGN")]
    IeeeNumericStdAndSgnSgn,
    /// `"and"` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_UNS")]
    IeeeNumericStdAndUns,
    /// `"and"` (`unsigned`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_UNS_LOG")]
    IeeeNumericStdAndUnsLog,
    /// `"and"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_AND_UNS_UNS")]
    IeeeNumericStdAndUnsUns,
    /// `"/"` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_INT_SGN")]
    IeeeNumericStdDivIntSgn,
    /// `"/"` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_NAT_UNS")]
    IeeeNumericStdDivNatUns,
    /// `"/"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_SGN_INT")]
    IeeeNumericStdDivSgnInt,
    /// `"/"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_SGN_SGN")]
    IeeeNumericStdDivSgnSgn,
    /// `"/"` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_UNS_NAT")]
    IeeeNumericStdDivUnsNat,
    /// `"/"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_DIV_UNS_UNS")]
    IeeeNumericStdDivUnsUns,
    /// `"="` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_INT_SGN")]
    IeeeNumericStdEqIntSgn,
    /// `"="` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_NAT_UNS")]
    IeeeNumericStdEqNatUns,
    /// `"="` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_SGN_INT")]
    IeeeNumericStdEqSgnInt,
    /// `"="` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_SGN_SGN")]
    IeeeNumericStdEqSgnSgn,
    /// `"="` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_UNS_NAT")]
    IeeeNumericStdEqUnsNat,
    /// `"="` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_EQ_UNS_UNS")]
    IeeeNumericStdEqUnsUns,
    /// `find_leftmost` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_FIND_LEFTMOST_SGN")]
    IeeeNumericStdFindLeftmostSgn,
    /// `find_leftmost` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_FIND_LEFTMOST_UNS")]
    IeeeNumericStdFindLeftmostUns,
    /// `find_rightmost` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_FIND_RIGHTMOST_SGN")]
    IeeeNumericStdFindRightmostSgn,
    /// `find_rightmost` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_FIND_RIGHTMOST_UNS")]
    IeeeNumericStdFindRightmostUns,
    /// `">="` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_INT_SGN")]
    IeeeNumericStdGeIntSgn,
    /// `">="` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_NAT_UNS")]
    IeeeNumericStdGeNatUns,
    /// `">="` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_SGN_INT")]
    IeeeNumericStdGeSgnInt,
    /// `">="` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_SGN_SGN")]
    IeeeNumericStdGeSgnSgn,
    /// `">="` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_UNS_NAT")]
    IeeeNumericStdGeUnsNat,
    /// `">="` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GE_UNS_UNS")]
    IeeeNumericStdGeUnsUns,
    /// `">"` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_INT_SGN")]
    IeeeNumericStdGtIntSgn,
    /// `">"` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_NAT_UNS")]
    IeeeNumericStdGtNatUns,
    /// `">"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_SGN_INT")]
    IeeeNumericStdGtSgnInt,
    /// `">"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_SGN_SGN")]
    IeeeNumericStdGtSgnSgn,
    /// `">"` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_UNS_NAT")]
    IeeeNumericStdGtUnsNat,
    /// `">"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_GT_UNS_UNS")]
    IeeeNumericStdGtUnsUns,
    /// `is_X` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_IS_X_SGN")]
    IeeeNumericStdIsXSgn,
    /// `is_X` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_IS_X_UNS")]
    IeeeNumericStdIsXUns,
    /// `"<="` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_INT_SGN")]
    IeeeNumericStdLeIntSgn,
    /// `"<="` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_NAT_UNS")]
    IeeeNumericStdLeNatUns,
    /// `"<="` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_SGN_INT")]
    IeeeNumericStdLeSgnInt,
    /// `"<="` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_SGN_SGN")]
    IeeeNumericStdLeSgnSgn,
    /// `"<="` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_UNS_NAT")]
    IeeeNumericStdLeUnsNat,
    /// `"<="` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LE_UNS_UNS")]
    IeeeNumericStdLeUnsUns,
    /// `"<"` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_INT_SGN")]
    IeeeNumericStdLtIntSgn,
    /// `"<"` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_NAT_UNS")]
    IeeeNumericStdLtNatUns,
    /// `"<"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_SGN_INT")]
    IeeeNumericStdLtSgnInt,
    /// `"<"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_SGN_SGN")]
    IeeeNumericStdLtSgnSgn,
    /// `"<"` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_UNS_NAT")]
    IeeeNumericStdLtUnsNat,
    /// `"<"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_LT_UNS_UNS")]
    IeeeNumericStdLtUnsUns,
    /// `"?="` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_INT_SGN")]
    IeeeNumericStdMatchEqIntSgn,
    /// `"?="` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_NAT_UNS")]
    IeeeNumericStdMatchEqNatUns,
    /// `"?="` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_SGN_INT")]
    IeeeNumericStdMatchEqSgnInt,
    /// `"?="` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_SGN_SGN")]
    IeeeNumericStdMatchEqSgnSgn,
    /// `"?="` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_UNS_NAT")]
    IeeeNumericStdMatchEqUnsNat,
    /// `"?="` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_EQ_UNS_UNS")]
    IeeeNumericStdMatchEqUnsUns,
    /// `"?>="` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_INT_SGN")]
    IeeeNumericStdMatchGeIntSgn,
    /// `"?>="` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_NAT_UNS")]
    IeeeNumericStdMatchGeNatUns,
    /// `"?>="` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_SGN_INT")]
    IeeeNumericStdMatchGeSgnInt,
    /// `"?>="` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_SGN_SGN")]
    IeeeNumericStdMatchGeSgnSgn,
    /// `"?>="` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_UNS_NAT")]
    IeeeNumericStdMatchGeUnsNat,
    /// `"?>="` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GE_UNS_UNS")]
    IeeeNumericStdMatchGeUnsUns,
    /// `"?>"` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_INT_SGN")]
    IeeeNumericStdMatchGtIntSgn,
    /// `"?>"` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_NAT_UNS")]
    IeeeNumericStdMatchGtNatUns,
    /// `"?>"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_SGN_INT")]
    IeeeNumericStdMatchGtSgnInt,
    /// `"?>"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_SGN_SGN")]
    IeeeNumericStdMatchGtSgnSgn,
    /// `"?>"` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_UNS_NAT")]
    IeeeNumericStdMatchGtUnsNat,
    /// `"?>"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_GT_UNS_UNS")]
    IeeeNumericStdMatchGtUnsUns,
    /// `"?<="` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_INT_SGN")]
    IeeeNumericStdMatchLeIntSgn,
    /// `"?<="` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_NAT_UNS")]
    IeeeNumericStdMatchLeNatUns,
    /// `"?<="` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_SGN_INT")]
    IeeeNumericStdMatchLeSgnInt,
    /// `"?<="` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_SGN_SGN")]
    IeeeNumericStdMatchLeSgnSgn,
    /// `"?<="` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_UNS_NAT")]
    IeeeNumericStdMatchLeUnsNat,
    /// `"?<="` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LE_UNS_UNS")]
    IeeeNumericStdMatchLeUnsUns,
    /// `std_match` (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LOG")]
    IeeeNumericStdMatchLog,
    /// `"?<"` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_INT_SGN")]
    IeeeNumericStdMatchLtIntSgn,
    /// `"?<"` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_NAT_UNS")]
    IeeeNumericStdMatchLtNatUns,
    /// `"?<"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_SGN_INT")]
    IeeeNumericStdMatchLtSgnInt,
    /// `"?<"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_SGN_SGN")]
    IeeeNumericStdMatchLtSgnSgn,
    /// `"?<"` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_UNS_NAT")]
    IeeeNumericStdMatchLtUnsNat,
    /// `"?<"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_LT_UNS_UNS")]
    IeeeNumericStdMatchLtUnsUns,
    /// `"?/="` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_INT_SGN")]
    IeeeNumericStdMatchNeIntSgn,
    /// `"?/="` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_NAT_UNS")]
    IeeeNumericStdMatchNeNatUns,
    /// `"?/="` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_SGN_INT")]
    IeeeNumericStdMatchNeSgnInt,
    /// `"?/="` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_SGN_SGN")]
    IeeeNumericStdMatchNeSgnSgn,
    /// `"?/="` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_UNS_NAT")]
    IeeeNumericStdMatchNeUnsNat,
    /// `"?/="` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_NE_UNS_UNS")]
    IeeeNumericStdMatchNeUnsUns,
    /// `std_match` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_SGN")]
    IeeeNumericStdMatchSgn,
    /// `std_match` (`std_logic_vector`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_SLV")]
    IeeeNumericStdMatchSlv,
    /// `std_match` (`std_ulogic_vector`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_SUV")]
    IeeeNumericStdMatchSuv,
    /// `std_match` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MATCH_UNS")]
    IeeeNumericStdMatchUns,
    /// `maximum` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_INT_SGN")]
    IeeeNumericStdMaxIntSgn,
    /// `maximum` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_NAT_UNS")]
    IeeeNumericStdMaxNatUns,
    /// `maximum` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_SGN_INT")]
    IeeeNumericStdMaxSgnInt,
    /// `maximum` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_SGN_SGN")]
    IeeeNumericStdMaxSgnSgn,
    /// `maximum` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_UNS_NAT")]
    IeeeNumericStdMaxUnsNat,
    /// `maximum` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MAX_UNS_UNS")]
    IeeeNumericStdMaxUnsUns,
    /// `minimum` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_INT_SGN")]
    IeeeNumericStdMinIntSgn,
    /// `minimum` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_NAT_UNS")]
    IeeeNumericStdMinNatUns,
    /// `minimum` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_SGN_INT")]
    IeeeNumericStdMinSgnInt,
    /// `minimum` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_SGN_SGN")]
    IeeeNumericStdMinSgnSgn,
    /// `minimum` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_UNS_NAT")]
    IeeeNumericStdMinUnsNat,
    /// `minimum` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MIN_UNS_UNS")]
    IeeeNumericStdMinUnsUns,
    /// `"mod"` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_INT_SGN")]
    IeeeNumericStdModIntSgn,
    /// `"mod"` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_NAT_UNS")]
    IeeeNumericStdModNatUns,
    /// `"mod"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_SGN_INT")]
    IeeeNumericStdModSgnInt,
    /// `"mod"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_SGN_SGN")]
    IeeeNumericStdModSgnSgn,
    /// `"mod"` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_UNS_NAT")]
    IeeeNumericStdModUnsNat,
    /// `"mod"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MOD_UNS_UNS")]
    IeeeNumericStdModUnsUns,
    /// `"*"` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_INT_SGN")]
    IeeeNumericStdMulIntSgn,
    /// `"*"` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_NAT_UNS")]
    IeeeNumericStdMulNatUns,
    /// `"*"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_SGN_INT")]
    IeeeNumericStdMulSgnInt,
    /// `"*"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_SGN_SGN")]
    IeeeNumericStdMulSgnSgn,
    /// `"*"` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_UNS_NAT")]
    IeeeNumericStdMulUnsNat,
    /// `"*"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_MUL_UNS_UNS")]
    IeeeNumericStdMulUnsUns,
    /// `"nand"` (`std_ulogic`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_LOG_SGN")]
    IeeeNumericStdNandLogSgn,
    /// `"nand"` (`std_ulogic`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_LOG_UNS")]
    IeeeNumericStdNandLogUns,
    /// `"nand"` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_SGN")]
    IeeeNumericStdNandSgn,
    /// `"nand"` (`signed`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_SGN_LOG")]
    IeeeNumericStdNandSgnLog,
    /// `"nand"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_SGN_SGN")]
    IeeeNumericStdNandSgnSgn,
    /// `"nand"` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_UNS")]
    IeeeNumericStdNandUns,
    /// `"nand"` (`unsigned`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_UNS_LOG")]
    IeeeNumericStdNandUnsLog,
    /// `"nand"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NAND_UNS_UNS")]
    IeeeNumericStdNandUnsUns,
    /// `"/="` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_INT_SGN")]
    IeeeNumericStdNeIntSgn,
    /// `"/="` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_NAT_UNS")]
    IeeeNumericStdNeNatUns,
    /// `"/="` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_SGN_INT")]
    IeeeNumericStdNeSgnInt,
    /// `"/="` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_SGN_SGN")]
    IeeeNumericStdNeSgnSgn,
    /// `"/="` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_UNS_NAT")]
    IeeeNumericStdNeUnsNat,
    /// `"/="` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NE_UNS_UNS")]
    IeeeNumericStdNeUnsUns,
    /// unary `"-"` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NEG_SGN")]
    IeeeNumericStdNegSgn,
    /// unary `"-"` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NEG_UNS")]
    IeeeNumericStdNegUns,
    /// `"nor"` (`std_ulogic`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_LOG_SGN")]
    IeeeNumericStdNorLogSgn,
    /// `"nor"` (`std_ulogic`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_LOG_UNS")]
    IeeeNumericStdNorLogUns,
    /// `"nor"` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_SGN")]
    IeeeNumericStdNorSgn,
    /// `"nor"` (`signed`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_SGN_LOG")]
    IeeeNumericStdNorSgnLog,
    /// `"nor"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_SGN_SGN")]
    IeeeNumericStdNorSgnSgn,
    /// `"nor"` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_UNS")]
    IeeeNumericStdNorUns,
    /// `"nor"` (`unsigned`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_UNS_LOG")]
    IeeeNumericStdNorUnsLog,
    /// `"nor"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOR_UNS_UNS")]
    IeeeNumericStdNorUnsUns,
    /// `"not"` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOT_SGN")]
    IeeeNumericStdNotSgn,
    /// `"not"` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_NOT_UNS")]
    IeeeNumericStdNotUns,
    /// `"or"` (`std_ulogic`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_LOG_SGN")]
    IeeeNumericStdOrLogSgn,
    /// `"or"` (`std_ulogic`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_LOG_UNS")]
    IeeeNumericStdOrLogUns,
    /// `"or"` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_SGN")]
    IeeeNumericStdOrSgn,
    /// `"or"` (`signed`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_SGN_LOG")]
    IeeeNumericStdOrSgnLog,
    /// `"or"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_SGN_SGN")]
    IeeeNumericStdOrSgnSgn,
    /// `"or"` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_UNS")]
    IeeeNumericStdOrUns,
    /// `"or"` (`unsigned`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_UNS_LOG")]
    IeeeNumericStdOrUnsLog,
    /// `"or"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_OR_UNS_UNS")]
    IeeeNumericStdOrUnsUns,
    /// `"rem"` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_INT_SGN")]
    IeeeNumericStdRemIntSgn,
    /// `"rem"` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_NAT_UNS")]
    IeeeNumericStdRemNatUns,
    /// `"rem"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_SGN_INT")]
    IeeeNumericStdRemSgnInt,
    /// `"rem"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_SGN_SGN")]
    IeeeNumericStdRemSgnSgn,
    /// `"rem"` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_UNS_NAT")]
    IeeeNumericStdRemUnsNat,
    /// `"rem"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_REM_UNS_UNS")]
    IeeeNumericStdRemUnsUns,
    /// `resize` (`signed`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_RESIZE_SGN_NAT")]
    IeeeNumericStdResizeSgnNat,
    /// `resize` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_RESIZE_SGN_SGN")]
    IeeeNumericStdResizeSgnSgn,
    /// `resize` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_RESIZE_UNS_NAT")]
    IeeeNumericStdResizeUnsNat,
    /// `resize` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_RESIZE_UNS_UNS")]
    IeeeNumericStdResizeUnsUns,
    /// `"rol"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROL_SGN_INT")]
    IeeeNumericStdRolSgnInt,
    /// `"rol"` (`unsigned`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROL_UNS_INT")]
    IeeeNumericStdRolUnsInt,
    /// `"ror"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROR_SGN_INT")]
    IeeeNumericStdRorSgnInt,
    /// `"ror"` (`unsigned`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROR_UNS_INT")]
    IeeeNumericStdRorUnsInt,
    /// `rotate_left` (`signed`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROT_LEFT_SGN_NAT")]
    IeeeNumericStdRotLeftSgnNat,
    /// `rotate_left` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROT_LEFT_UNS_NAT")]
    IeeeNumericStdRotLeftUnsNat,
    /// `rotate_right` (`signed`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROT_RIGHT_SGN_NAT")]
    IeeeNumericStdRotRightSgnNat,
    /// `rotate_right` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_ROT_RIGHT_UNS_NAT")]
    IeeeNumericStdRotRightUnsNat,
    /// `shift_left` (`signed`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SHF_LEFT_SGN_NAT")]
    IeeeNumericStdShfLeftSgnNat,
    /// `shift_left` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SHF_LEFT_UNS_NAT")]
    IeeeNumericStdShfLeftUnsNat,
    /// `shift_right` (`signed`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SHF_RIGHT_SGN_NAT")]
    IeeeNumericStdShfRightSgnNat,
    /// `shift_right` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SHF_RIGHT_UNS_NAT")]
    IeeeNumericStdShfRightUnsNat,
    /// `"sla"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SLA_SGN_INT")]
    IeeeNumericStdSlaSgnInt,
    /// `"sla"` (`unsigned`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SLA_UNS_INT")]
    IeeeNumericStdSlaUnsInt,
    /// `"sll"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SLL_SGN_INT")]
    IeeeNumericStdSllSgnInt,
    /// `"sll"` (`unsigned`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SLL_UNS_INT")]
    IeeeNumericStdSllUnsInt,
    /// `"sra"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SRA_SGN_INT")]
    IeeeNumericStdSraSgnInt,
    /// `"sra"` (`unsigned`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SRA_UNS_INT")]
    IeeeNumericStdSraUnsInt,
    /// `"srl"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SRL_SGN_INT")]
    IeeeNumericStdSrlSgnInt,
    /// `"srl"` (`unsigned`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SRL_UNS_INT")]
    IeeeNumericStdSrlUnsInt,
    /// `"-"` (`integer`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_INT_SGN")]
    IeeeNumericStdSubIntSgn,
    /// `"-"` (`std_ulogic`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_LOG_SGN")]
    IeeeNumericStdSubLogSgn,
    /// `"-"` (`std_ulogic`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_LOG_UNS")]
    IeeeNumericStdSubLogUns,
    /// `"-"` (`natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_NAT_UNS")]
    IeeeNumericStdSubNatUns,
    /// `"-"` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_SGN_INT")]
    IeeeNumericStdSubSgnInt,
    /// `"-"` (`signed`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_SGN_LOG")]
    IeeeNumericStdSubSgnLog,
    /// `"-"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_SGN_SGN")]
    IeeeNumericStdSubSgnSgn,
    /// `"-"` (`unsigned`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_UNS_LOG")]
    IeeeNumericStdSubUnsLog,
    /// `"-"` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_UNS_NAT")]
    IeeeNumericStdSubUnsNat,
    /// `"-"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_SUB_UNS_UNS")]
    IeeeNumericStdSubUnsUns,
    /// `to_01` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_01_SGN")]
    IeeeNumericStdTo01Sgn,
    /// `to_01` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_01_UNS")]
    IeeeNumericStdTo01Uns,
    /// `to_hstring` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_HSTRING_SGN")]
    IeeeNumericStdToHstringSgn,
    /// `to_hstring` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_HSTRING_UNS")]
    IeeeNumericStdToHstringUns,
    /// `to_ostring` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_OSTRING_SGN")]
    IeeeNumericStdToOstringSgn,
    /// `to_ostring` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_OSTRING_UNS")]
    IeeeNumericStdToOstringUns,
    /// `to_UX01` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_UX01_SGN")]
    IeeeNumericStdToUx01Sgn,
    /// `to_UX01` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_UX01_UNS")]
    IeeeNumericStdToUx01Uns,
    /// `to_X01` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_X01_SGN")]
    IeeeNumericStdToX01Sgn,
    /// `to_X01` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_X01_UNS")]
    IeeeNumericStdToX01Uns,
    /// `to_X01Z` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_X01Z_SGN")]
    IeeeNumericStdToX01zSgn,
    /// `to_X01Z` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TO_X01Z_UNS")]
    IeeeNumericStdToX01zUns,
    /// `to_integer` (`signed`, `integer`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOINT_SGN_INT")]
    IeeeNumericStdToIntSgnInt,
    /// `to_integer` (`unsigned`, `natural`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOINT_UNS_NAT")]
    IeeeNumericStdToIntUnsNat,
    /// `to_signed` (`integer`, `natural`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOSGN_INT_NAT_SGN")]
    IeeeNumericStdToSgnIntNatSgn,
    /// `to_signed` (`integer`, `signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOSGN_INT_SGN_SGN")]
    IeeeNumericStdToSgnIntSgnSgn,
    /// `to_unsigned` (`natural`, `natural`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOUNS_NAT_NAT_UNS")]
    IeeeNumericStdToUnsNatNatUns,
    /// `to_unsigned` (`natural`, `unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_TOUNS_NAT_UNS_UNS")]
    IeeeNumericStdToUnsNatUnsUns,
    /// `"+"` (`natural`, `std_logic_vector`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_ADD_NAT_SLV")]
    IeeeNumericStdUnsignedAddNatSlv,
    /// `"+"` (`std_logic_vector`, `natural`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_ADD_SLV_NAT")]
    IeeeNumericStdUnsignedAddSlvNat,
    /// `"+"` (`std_logic_vector`, `std_logic_vector`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_ADD_SLV_SLV")]
    IeeeNumericStdUnsignedAddSlvSlv,
    /// `find_leftmost` (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_FIND_LEFTMOST")]
    IeeeNumericStdUnsignedFindLeftmost,
    /// `find_rightmost` (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_FIND_RIGHTMOST")]
    IeeeNumericStdUnsignedFindRightmost,
    /// `maximum` (`std_logic_vector`, `std_logic_vector`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_MAXIMUM_SLV_SLV")]
    IeeeNumericStdUnsignedMaximumSlvSlv,
    /// `minimum` (`std_logic_vector`, `std_logic_vector`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_MINIMUM_SLV_SLV")]
    IeeeNumericStdUnsignedMinimumSlvSlv,
    /// `resize` (`std_logic_vector`, `natural`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_RESIZE_SLV_NAT")]
    IeeeNumericStdUnsignedResizeSlvNat,
    /// `resize` (`std_logic_vector`, `std_logic_vector`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_RESIZE_SLV_SLV")]
    IeeeNumericStdUnsignedResizeSlvSlv,
    /// `rotate_left` (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_ROTATE_LEFT")]
    IeeeNumericStdUnsignedRotateLeft,
    /// `rotate_right` (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_ROTATE_RIGHT")]
    IeeeNumericStdUnsignedRotateRight,
    /// `shift_left` (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_SHIFT_LEFT")]
    IeeeNumericStdUnsignedShiftLeft,
    /// `shift_right` (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_SHIFT_RIGHT")]
    IeeeNumericStdUnsignedShiftRight,
    /// `"-"` (`natural`, `std_logic_vector`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_SUB_NAT_SLV")]
    IeeeNumericStdUnsignedSubNatSlv,
    /// `"-"` (`std_logic_vector`, `natural`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_SUB_SLV_NAT")]
    IeeeNumericStdUnsignedSubSlvNat,
    /// `"-"` (`std_logic_vector`, `std_logic_vector`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_SUB_SLV_SLV")]
    IeeeNumericStdUnsignedSubSlvSlv,
    /// `to_integer` (`std_logic_vector`, `natural`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_TO_INTEGER_SLV_NAT")]
    IeeeNumericStdUnsignedToIntegerSlvNat,
    /// `to_stdlogicvector` (`natural`, `natural`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_TO_SLV_NAT_NAT")]
    IeeeNumericStdUnsignedToSlvNatNat,
    /// `to_stdlogicvector` (`natural`, `std_logic_vector`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_TO_SLV_NAT_SLV")]
    IeeeNumericStdUnsignedToSlvNatSlv,
    /// `to_stdulogicvector` (`natural`, `natural`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_TO_SUV_NAT_NAT")]
    IeeeNumericStdUnsignedToSuvNatNat,
    /// `to_stdulogicvector` (`natural`, `std_ulogic_vector`) (`ieee.numeric_std_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_UNSIGNED_TO_SUV_NAT_SUV")]
    IeeeNumericStdUnsignedToSuvNatSuv,
    /// `"xnor"` (`std_ulogic`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_LOG_SGN")]
    IeeeNumericStdXnorLogSgn,
    /// `"xnor"` (`std_ulogic`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_LOG_UNS")]
    IeeeNumericStdXnorLogUns,
    /// `"xnor"` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_SGN")]
    IeeeNumericStdXnorSgn,
    /// `"xnor"` (`signed`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_SGN_LOG")]
    IeeeNumericStdXnorSgnLog,
    /// `"xnor"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_SGN_SGN")]
    IeeeNumericStdXnorSgnSgn,
    /// `"xnor"` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_UNS")]
    IeeeNumericStdXnorUns,
    /// `"xnor"` (`unsigned`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_UNS_LOG")]
    IeeeNumericStdXnorUnsLog,
    /// `"xnor"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XNOR_UNS_UNS")]
    IeeeNumericStdXnorUnsUns,
    /// `"xor"` (`std_ulogic`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_LOG_SGN")]
    IeeeNumericStdXorLogSgn,
    /// `"xor"` (`std_ulogic`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_LOG_UNS")]
    IeeeNumericStdXorLogUns,
    /// `"xor"` (`signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_SGN")]
    IeeeNumericStdXorSgn,
    /// `"xor"` (`signed`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_SGN_LOG")]
    IeeeNumericStdXorSgnLog,
    /// `"xor"` (`signed`, `signed`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_SGN_SGN")]
    IeeeNumericStdXorSgnSgn,
    /// `"xor"` (`unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_UNS")]
    IeeeNumericStdXorUns,
    /// `"xor"` (`unsigned`, `std_ulogic`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_UNS_LOG")]
    IeeeNumericStdXorUnsLog,
    /// `"xor"` (`unsigned`, `unsigned`) (`ieee.numeric_std`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_STD_XOR_UNS_UNS")]
    IeeeNumericStdXorUnsUns,
    /// `"abs"` (`signed`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ABS_SGN_SGN")]
    IeeeStdLogicArithAbsSgnSgn,
    /// `"abs"` (`signed`, `std_logic_vector`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ABS_SGN_SLV")]
    IeeeStdLogicArithAbsSgnSlv,
    /// `"+"` (`integer`, `signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_INT_SGN_SGN")]
    IeeeStdLogicArithAddIntSgnSgn,
    /// `"+"` (`integer`, `signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_INT_SGN_SLV")]
    IeeeStdLogicArithAddIntSgnSlv,
    /// `"+"` (`integer`, `unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_INT_UNS_SLV")]
    IeeeStdLogicArithAddIntUnsSlv,
    /// `"+"` (`integer`, `unsigned`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_INT_UNS_UNS")]
    IeeeStdLogicArithAddIntUnsUns,
    /// `"+"` (`std_ulogic`, `signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_LOG_SGN_SGN")]
    IeeeStdLogicArithAddLogSgnSgn,
    /// `"+"` (`std_ulogic`, `signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_LOG_SGN_SLV")]
    IeeeStdLogicArithAddLogSgnSlv,
    /// `"+"` (`std_ulogic`, `unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_LOG_UNS_SLV")]
    IeeeStdLogicArithAddLogUnsSlv,
    /// `"+"` (`std_ulogic`, `unsigned`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_LOG_UNS_UNS")]
    IeeeStdLogicArithAddLogUnsUns,
    /// `"+"` (`signed`, `integer`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_INT_SGN")]
    IeeeStdLogicArithAddSgnIntSgn,
    /// `"+"` (`signed`, `integer`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_INT_SLV")]
    IeeeStdLogicArithAddSgnIntSlv,
    /// `"+"` (`signed`, `std_ulogic`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_LOG_SGN")]
    IeeeStdLogicArithAddSgnLogSgn,
    /// `"+"` (`signed`, `std_ulogic`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_LOG_SLV")]
    IeeeStdLogicArithAddSgnLogSlv,
    /// `"+"` (`signed`, `signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_SGN_SGN")]
    IeeeStdLogicArithAddSgnSgnSgn,
    /// `"+"` (`signed`, `signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_SGN_SLV")]
    IeeeStdLogicArithAddSgnSgnSlv,
    /// `"+"` (`signed`, `unsigned`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_UNS_SGN")]
    IeeeStdLogicArithAddSgnUnsSgn,
    /// `"+"` (`signed`, `unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_SGN_UNS_SLV")]
    IeeeStdLogicArithAddSgnUnsSlv,
    /// `"+"` (`unsigned`, `integer`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_INT_SLV")]
    IeeeStdLogicArithAddUnsIntSlv,
    /// `"+"` (`unsigned`, `integer`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_INT_UNS")]
    IeeeStdLogicArithAddUnsIntUns,
    /// `"+"` (`unsigned`, `std_ulogic`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_LOG_SLV")]
    IeeeStdLogicArithAddUnsLogSlv,
    /// `"+"` (`unsigned`, `std_ulogic`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_LOG_UNS")]
    IeeeStdLogicArithAddUnsLogUns,
    /// `"+"` (`unsigned`, `signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_SGN_SGN")]
    IeeeStdLogicArithAddUnsSgnSgn,
    /// `"+"` (`unsigned`, `signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_SGN_SLV")]
    IeeeStdLogicArithAddUnsSgnSlv,
    /// `"+"` (`unsigned`, `unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_UNS_SLV")]
    IeeeStdLogicArithAddUnsUnsSlv,
    /// `"+"` (`unsigned`, `unsigned`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ADD_UNS_UNS_UNS")]
    IeeeStdLogicArithAddUnsUnsUns,
    /// `CONV_INTEGER` (`integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_INTEGER_INT")]
    IeeeStdLogicArithConvIntegerInt,
    /// `CONV_INTEGER` (`std_ulogic`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_INTEGER_LOG")]
    IeeeStdLogicArithConvIntegerLog,
    /// `CONV_INTEGER` (`signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_INTEGER_SGN")]
    IeeeStdLogicArithConvIntegerSgn,
    /// `CONV_INTEGER` (`unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_INTEGER_UNS")]
    IeeeStdLogicArithConvIntegerUns,
    /// `CONV_SIGNED` (`integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_SIGNED_INT")]
    IeeeStdLogicArithConvSignedInt,
    /// `CONV_SIGNED` (`std_ulogic`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_SIGNED_LOG")]
    IeeeStdLogicArithConvSignedLog,
    /// `CONV_SIGNED` (`signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_SIGNED_SGN")]
    IeeeStdLogicArithConvSignedSgn,
    /// `CONV_SIGNED` (`unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_SIGNED_UNS")]
    IeeeStdLogicArithConvSignedUns,
    /// `CONV_UNSIGNED` (`integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_UNSIGNED_INT")]
    IeeeStdLogicArithConvUnsignedInt,
    /// `CONV_UNSIGNED` (`std_ulogic`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_UNSIGNED_LOG")]
    IeeeStdLogicArithConvUnsignedLog,
    /// `CONV_UNSIGNED` (`signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_UNSIGNED_SGN")]
    IeeeStdLogicArithConvUnsignedSgn,
    /// `CONV_UNSIGNED` (`unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_UNSIGNED_UNS")]
    IeeeStdLogicArithConvUnsignedUns,
    /// `CONV_VECTOR` (`integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_VECTOR_INT")]
    IeeeStdLogicArithConvVectorInt,
    /// `CONV_VECTOR` (`std_ulogic`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_VECTOR_LOG")]
    IeeeStdLogicArithConvVectorLog,
    /// `CONV_VECTOR` (`signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_VECTOR_SGN")]
    IeeeStdLogicArithConvVectorSgn,
    /// `CONV_VECTOR` (`unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_CONV_VECTOR_UNS")]
    IeeeStdLogicArithConvVectorUns,
    /// `"="` (`integer`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_INT_SGN")]
    IeeeStdLogicArithEqIntSgn,
    /// `"="` (`integer`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_INT_UNS")]
    IeeeStdLogicArithEqIntUns,
    /// `"="` (`signed`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_SGN_INT")]
    IeeeStdLogicArithEqSgnInt,
    /// `"="` (`signed`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_SGN_SGN")]
    IeeeStdLogicArithEqSgnSgn,
    /// `"="` (`signed`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_SGN_UNS")]
    IeeeStdLogicArithEqSgnUns,
    /// `"="` (`unsigned`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_UNS_INT")]
    IeeeStdLogicArithEqUnsInt,
    /// `"="` (`unsigned`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_UNS_SGN")]
    IeeeStdLogicArithEqUnsSgn,
    /// `"="` (`unsigned`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EQ_UNS_UNS")]
    IeeeStdLogicArithEqUnsUns,
    /// `EXT` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_EXT")]
    IeeeStdLogicArithExt,
    /// `">="` (`integer`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_INT_SGN")]
    IeeeStdLogicArithGeIntSgn,
    /// `">="` (`integer`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_INT_UNS")]
    IeeeStdLogicArithGeIntUns,
    /// `">="` (`signed`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_SGN_INT")]
    IeeeStdLogicArithGeSgnInt,
    /// `">="` (`signed`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_SGN_SGN")]
    IeeeStdLogicArithGeSgnSgn,
    /// `">="` (`signed`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_SGN_UNS")]
    IeeeStdLogicArithGeSgnUns,
    /// `">="` (`unsigned`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_UNS_INT")]
    IeeeStdLogicArithGeUnsInt,
    /// `">="` (`unsigned`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_UNS_SGN")]
    IeeeStdLogicArithGeUnsSgn,
    /// `">="` (`unsigned`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GE_UNS_UNS")]
    IeeeStdLogicArithGeUnsUns,
    /// `">"` (`integer`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_INT_SGN")]
    IeeeStdLogicArithGtIntSgn,
    /// `">"` (`integer`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_INT_UNS")]
    IeeeStdLogicArithGtIntUns,
    /// `">"` (`signed`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_SGN_INT")]
    IeeeStdLogicArithGtSgnInt,
    /// `">"` (`signed`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_SGN_SGN")]
    IeeeStdLogicArithGtSgnSgn,
    /// `">"` (`signed`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_SGN_UNS")]
    IeeeStdLogicArithGtSgnUns,
    /// `">"` (`unsigned`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_UNS_INT")]
    IeeeStdLogicArithGtUnsInt,
    /// `">"` (`unsigned`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_UNS_SGN")]
    IeeeStdLogicArithGtUnsSgn,
    /// `">"` (`unsigned`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_GT_UNS_UNS")]
    IeeeStdLogicArithGtUnsUns,
    /// unary `"+"` (`signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ID_SGN_SGN")]
    IeeeStdLogicArithIdSgnSgn,
    /// unary `"+"` (`signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ID_SGN_SLV")]
    IeeeStdLogicArithIdSgnSlv,
    /// unary `"+"` (`unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ID_UNS_SLV")]
    IeeeStdLogicArithIdUnsSlv,
    /// unary `"+"` (`unsigned`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_ID_UNS_UNS")]
    IeeeStdLogicArithIdUnsUns,
    /// `"<="` (`integer`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_INT_SGN")]
    IeeeStdLogicArithLeIntSgn,
    /// `"<="` (`integer`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_INT_UNS")]
    IeeeStdLogicArithLeIntUns,
    /// `"<="` (`signed`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_SGN_INT")]
    IeeeStdLogicArithLeSgnInt,
    /// `"<="` (`signed`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_SGN_SGN")]
    IeeeStdLogicArithLeSgnSgn,
    /// `"<="` (`signed`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_SGN_UNS")]
    IeeeStdLogicArithLeSgnUns,
    /// `"<="` (`unsigned`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_UNS_INT")]
    IeeeStdLogicArithLeUnsInt,
    /// `"<="` (`unsigned`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_UNS_SGN")]
    IeeeStdLogicArithLeUnsSgn,
    /// `"<="` (`unsigned`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LE_UNS_UNS")]
    IeeeStdLogicArithLeUnsUns,
    /// `"<"` (`integer`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_INT_SGN")]
    IeeeStdLogicArithLtIntSgn,
    /// `"<"` (`integer`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_INT_UNS")]
    IeeeStdLogicArithLtIntUns,
    /// `"<"` (`signed`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_SGN_INT")]
    IeeeStdLogicArithLtSgnInt,
    /// `"<"` (`signed`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_SGN_SGN")]
    IeeeStdLogicArithLtSgnSgn,
    /// `"<"` (`signed`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_SGN_UNS")]
    IeeeStdLogicArithLtSgnUns,
    /// `"<"` (`unsigned`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_UNS_INT")]
    IeeeStdLogicArithLtUnsInt,
    /// `"<"` (`unsigned`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_UNS_SGN")]
    IeeeStdLogicArithLtUnsSgn,
    /// `"<"` (`unsigned`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_LT_UNS_UNS")]
    IeeeStdLogicArithLtUnsUns,
    /// `"*"` (`signed`, `signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_SGN_SGN_SGN")]
    IeeeStdLogicArithMulSgnSgnSgn,
    /// `"*"` (`signed`, `signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_SGN_SGN_SLV")]
    IeeeStdLogicArithMulSgnSgnSlv,
    /// `"*"` (`signed`, `unsigned`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_SGN_UNS_SGN")]
    IeeeStdLogicArithMulSgnUnsSgn,
    /// `"*"` (`signed`, `unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_SGN_UNS_SLV")]
    IeeeStdLogicArithMulSgnUnsSlv,
    /// `"*"` (`unsigned`, `signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_UNS_SGN_SGN")]
    IeeeStdLogicArithMulUnsSgnSgn,
    /// `"*"` (`unsigned`, `signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_UNS_SGN_SLV")]
    IeeeStdLogicArithMulUnsSgnSlv,
    /// `"*"` (`unsigned`, `unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_UNS_UNS_SLV")]
    IeeeStdLogicArithMulUnsUnsSlv,
    /// `"*"` (`unsigned`, `unsigned`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_MUL_UNS_UNS_UNS")]
    IeeeStdLogicArithMulUnsUnsUns,
    /// `"/="` (`integer`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_INT_SGN")]
    IeeeStdLogicArithNeIntSgn,
    /// `"/="` (`integer`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_INT_UNS")]
    IeeeStdLogicArithNeIntUns,
    /// `"/="` (`signed`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_SGN_INT")]
    IeeeStdLogicArithNeSgnInt,
    /// `"/="` (`signed`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_SGN_SGN")]
    IeeeStdLogicArithNeSgnSgn,
    /// `"/="` (`signed`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_SGN_UNS")]
    IeeeStdLogicArithNeSgnUns,
    /// `"/="` (`unsigned`, `integer`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_UNS_INT")]
    IeeeStdLogicArithNeUnsInt,
    /// `"/="` (`unsigned`, `signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_UNS_SGN")]
    IeeeStdLogicArithNeUnsSgn,
    /// `"/="` (`unsigned`, `unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NE_UNS_UNS")]
    IeeeStdLogicArithNeUnsUns,
    /// unary `"-"` (`signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NEG_SGN_SGN")]
    IeeeStdLogicArithNegSgnSgn,
    /// unary `"-"` (`signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_NEG_SGN_SLV")]
    IeeeStdLogicArithNegSgnSlv,
    /// `shift_left` (`signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SHL_SGN")]
    IeeeStdLogicArithShlSgn,
    /// `shift_left` (`unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SHL_UNS")]
    IeeeStdLogicArithShlUns,
    /// `shift_right` (`signed`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SHR_SGN")]
    IeeeStdLogicArithShrSgn,
    /// `shift_right` (`unsigned`) (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SHR_UNS")]
    IeeeStdLogicArithShrUns,
    /// `"-"` (`integer`, `signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_INT_SGN_SGN")]
    IeeeStdLogicArithSubIntSgnSgn,
    /// `"-"` (`integer`, `signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_INT_SGN_SLV")]
    IeeeStdLogicArithSubIntSgnSlv,
    /// `"-"` (`integer`, `unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_INT_UNS_SLV")]
    IeeeStdLogicArithSubIntUnsSlv,
    /// `"-"` (`integer`, `unsigned`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_INT_UNS_UNS")]
    IeeeStdLogicArithSubIntUnsUns,
    /// `"-"` (`std_ulogic`, `signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_LOG_SGN_SGN")]
    IeeeStdLogicArithSubLogSgnSgn,
    /// `"-"` (`std_ulogic`, `signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_LOG_SGN_SLV")]
    IeeeStdLogicArithSubLogSgnSlv,
    /// `"-"` (`std_ulogic`, `unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_LOG_UNS_SLV")]
    IeeeStdLogicArithSubLogUnsSlv,
    /// `"-"` (`std_ulogic`, `unsigned`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_LOG_UNS_UNS")]
    IeeeStdLogicArithSubLogUnsUns,
    /// `"-"` (`signed`, `integer`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_INT_SGN")]
    IeeeStdLogicArithSubSgnIntSgn,
    /// `"-"` (`signed`, `integer`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_INT_SLV")]
    IeeeStdLogicArithSubSgnIntSlv,
    /// `"-"` (`signed`, `std_ulogic`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_LOG_SGN")]
    IeeeStdLogicArithSubSgnLogSgn,
    /// `"-"` (`signed`, `std_ulogic`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_LOG_SLV")]
    IeeeStdLogicArithSubSgnLogSlv,
    /// `"-"` (`signed`, `signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_SGN_SGN")]
    IeeeStdLogicArithSubSgnSgnSgn,
    /// `"-"` (`signed`, `signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_SGN_SLV")]
    IeeeStdLogicArithSubSgnSgnSlv,
    /// `"-"` (`signed`, `unsigned`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_UNS_SGN")]
    IeeeStdLogicArithSubSgnUnsSgn,
    /// `"-"` (`signed`, `unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_SGN_UNS_SLV")]
    IeeeStdLogicArithSubSgnUnsSlv,
    /// `"-"` (`unsigned`, `integer`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_INT_SLV")]
    IeeeStdLogicArithSubUnsIntSlv,
    /// `"-"` (`unsigned`, `integer`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_INT_UNS")]
    IeeeStdLogicArithSubUnsIntUns,
    /// `"-"` (`unsigned`, `std_ulogic`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_LOG_SLV")]
    IeeeStdLogicArithSubUnsLogSlv,
    /// `"-"` (`unsigned`, `std_ulogic`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_LOG_UNS")]
    IeeeStdLogicArithSubUnsLogUns,
    /// `"-"` (`unsigned`, `signed`) → `signed` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_SGN_SGN")]
    IeeeStdLogicArithSubUnsSgnSgn,
    /// `"-"` (`unsigned`, `signed`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_SGN_SLV")]
    IeeeStdLogicArithSubUnsSgnSlv,
    /// `"-"` (`unsigned`, `unsigned`) → `std_logic_vector` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_UNS_SLV")]
    IeeeStdLogicArithSubUnsUnsSlv,
    /// `"-"` (`unsigned`, `unsigned`) → `unsigned` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SUB_UNS_UNS_UNS")]
    IeeeStdLogicArithSubUnsUnsUns,
    /// `SXT` (`ieee.std_logic_arith`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_ARITH_SXT")]
    IeeeStdLogicArithSxt,
    /// `AND_REDUCE` (`std_logic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_AND_REDUCE_SLV")]
    IeeeStdLogicMiscAndReduceSlv,
    /// `AND_REDUCE` (`std_ulogic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_AND_REDUCE_SUV")]
    IeeeStdLogicMiscAndReduceSuv,
    /// `NAND_REDUCE` (`std_logic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_NAND_REDUCE_SLV")]
    IeeeStdLogicMiscNandReduceSlv,
    /// `NAND_REDUCE` (`std_ulogic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_NAND_REDUCE_SUV")]
    IeeeStdLogicMiscNandReduceSuv,
    /// `NOR_REDUCE` (`std_logic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_NOR_REDUCE_SLV")]
    IeeeStdLogicMiscNorReduceSlv,
    /// `NOR_REDUCE` (`std_ulogic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_NOR_REDUCE_SUV")]
    IeeeStdLogicMiscNorReduceSuv,
    /// `OR_REDUCE` (`std_logic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_OR_REDUCE_SLV")]
    IeeeStdLogicMiscOrReduceSlv,
    /// `OR_REDUCE` (`std_ulogic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_OR_REDUCE_SUV")]
    IeeeStdLogicMiscOrReduceSuv,
    /// `XNOR_REDUCE` (`std_logic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_XNOR_REDUCE_SLV")]
    IeeeStdLogicMiscXnorReduceSlv,
    /// `XNOR_REDUCE` (`std_ulogic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_XNOR_REDUCE_SUV")]
    IeeeStdLogicMiscXnorReduceSuv,
    /// `XOR_REDUCE` (`std_logic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_XOR_REDUCE_SLV")]
    IeeeStdLogicMiscXorReduceSlv,
    /// `XOR_REDUCE` (`std_ulogic_vector`) (`ieee.std_logic_misc`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_MISC_XOR_REDUCE_SUV")]
    IeeeStdLogicMiscXorReduceSuv,
    /// `"abs"` (`std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ABS_SLV")]
    IeeeStdLogicSignedAbsSlv,
    /// `"+"` (`integer`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ADD_INT_SLV")]
    IeeeStdLogicSignedAddIntSlv,
    /// `"+"` (`std_ulogic`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ADD_LOG_SLV")]
    IeeeStdLogicSignedAddLogSlv,
    /// `"+"` (`std_logic_vector`, `integer`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ADD_SLV_INT")]
    IeeeStdLogicSignedAddSlvInt,
    /// `"+"` (`std_logic_vector`, `std_ulogic`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ADD_SLV_LOG")]
    IeeeStdLogicSignedAddSlvLog,
    /// `"+"` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ADD_SLV_SLV")]
    IeeeStdLogicSignedAddSlvSlv,
    /// `CONV_INTEGER` (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_CONV_INTEGER")]
    IeeeStdLogicSignedConvInteger,
    /// `"="` (`integer`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_EQ_INT_SLV")]
    IeeeStdLogicSignedEqIntSlv,
    /// `"="` (`std_logic_vector`, `integer`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_EQ_SLV_INT")]
    IeeeStdLogicSignedEqSlvInt,
    /// `"="` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_EQ_SLV_SLV")]
    IeeeStdLogicSignedEqSlvSlv,
    /// `">="` (`integer`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GE_INT_SLV")]
    IeeeStdLogicSignedGeIntSlv,
    /// `">="` (`std_logic_vector`, `integer`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GE_SLV_INT")]
    IeeeStdLogicSignedGeSlvInt,
    /// `">="` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GE_SLV_SLV")]
    IeeeStdLogicSignedGeSlvSlv,
    /// `">"` (`integer`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GT_INT_SLV")]
    IeeeStdLogicSignedGtIntSlv,
    /// `">"` (`std_logic_vector`, `integer`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GT_SLV_INT")]
    IeeeStdLogicSignedGtSlvInt,
    /// `">"` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_GT_SLV_SLV")]
    IeeeStdLogicSignedGtSlvSlv,
    /// unary `"+"` (`std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_ID_SLV")]
    IeeeStdLogicSignedIdSlv,
    /// `"<="` (`integer`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LE_INT_SLV")]
    IeeeStdLogicSignedLeIntSlv,
    /// `"<="` (`std_logic_vector`, `integer`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LE_SLV_INT")]
    IeeeStdLogicSignedLeSlvInt,
    /// `"<="` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LE_SLV_SLV")]
    IeeeStdLogicSignedLeSlvSlv,
    /// `"<"` (`integer`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LT_INT_SLV")]
    IeeeStdLogicSignedLtIntSlv,
    /// `"<"` (`std_logic_vector`, `integer`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LT_SLV_INT")]
    IeeeStdLogicSignedLtSlvInt,
    /// `"<"` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_LT_SLV_SLV")]
    IeeeStdLogicSignedLtSlvSlv,
    /// `"*"` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_MUL_SLV_SLV")]
    IeeeStdLogicSignedMulSlvSlv,
    /// `"/="` (`integer`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_NE_INT_SLV")]
    IeeeStdLogicSignedNeIntSlv,
    /// `"/="` (`std_logic_vector`, `integer`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_NE_SLV_INT")]
    IeeeStdLogicSignedNeSlvInt,
    /// `"/="` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_NE_SLV_SLV")]
    IeeeStdLogicSignedNeSlvSlv,
    /// unary `"-"` (`std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_NEG_SLV")]
    IeeeStdLogicSignedNegSlv,
    /// `shift_left` (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SHL")]
    IeeeStdLogicSignedShl,
    /// `shift_right` (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SHR")]
    IeeeStdLogicSignedShr,
    /// `"-"` (`integer`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SUB_INT_SLV")]
    IeeeStdLogicSignedSubIntSlv,
    /// `"-"` (`std_ulogic`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SUB_LOG_SLV")]
    IeeeStdLogicSignedSubLogSlv,
    /// `"-"` (`std_logic_vector`, `integer`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SUB_SLV_INT")]
    IeeeStdLogicSignedSubSlvInt,
    /// `"-"` (`std_logic_vector`, `std_ulogic`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SUB_SLV_LOG")]
    IeeeStdLogicSignedSubSlvLog,
    /// `"-"` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_signed`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_SIGNED_SUB_SLV_SLV")]
    IeeeStdLogicSignedSubSlvSlv,
    /// `"+"` (`integer`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ADD_INT_SLV")]
    IeeeStdLogicUnsignedAddIntSlv,
    /// `"+"` (`std_ulogic`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ADD_LOG_SLV")]
    IeeeStdLogicUnsignedAddLogSlv,
    /// `"+"` (`std_logic_vector`, `integer`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ADD_SLV_INT")]
    IeeeStdLogicUnsignedAddSlvInt,
    /// `"+"` (`std_logic_vector`, `std_ulogic`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ADD_SLV_LOG")]
    IeeeStdLogicUnsignedAddSlvLog,
    /// `"+"` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ADD_SLV_SLV")]
    IeeeStdLogicUnsignedAddSlvSlv,
    /// `CONV_INTEGER` (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_CONV_INTEGER")]
    IeeeStdLogicUnsignedConvInteger,
    /// `"="` (`integer`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_EQ_INT_SLV")]
    IeeeStdLogicUnsignedEqIntSlv,
    /// `"="` (`std_logic_vector`, `integer`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_EQ_SLV_INT")]
    IeeeStdLogicUnsignedEqSlvInt,
    /// `"="` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_EQ_SLV_SLV")]
    IeeeStdLogicUnsignedEqSlvSlv,
    /// `">="` (`integer`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GE_INT_SLV")]
    IeeeStdLogicUnsignedGeIntSlv,
    /// `">="` (`std_logic_vector`, `integer`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GE_SLV_INT")]
    IeeeStdLogicUnsignedGeSlvInt,
    /// `">="` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GE_SLV_SLV")]
    IeeeStdLogicUnsignedGeSlvSlv,
    /// `">"` (`integer`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GT_INT_SLV")]
    IeeeStdLogicUnsignedGtIntSlv,
    /// `">"` (`std_logic_vector`, `integer`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GT_SLV_INT")]
    IeeeStdLogicUnsignedGtSlvInt,
    /// `">"` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_GT_SLV_SLV")]
    IeeeStdLogicUnsignedGtSlvSlv,
    /// unary `"+"` (`std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_ID_SLV")]
    IeeeStdLogicUnsignedIdSlv,
    /// `"<="` (`integer`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LE_INT_SLV")]
    IeeeStdLogicUnsignedLeIntSlv,
    /// `"<="` (`std_logic_vector`, `integer`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LE_SLV_INT")]
    IeeeStdLogicUnsignedLeSlvInt,
    /// `"<="` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LE_SLV_SLV")]
    IeeeStdLogicUnsignedLeSlvSlv,
    /// `"<"` (`integer`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LT_INT_SLV")]
    IeeeStdLogicUnsignedLtIntSlv,
    /// `"<"` (`std_logic_vector`, `integer`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LT_SLV_INT")]
    IeeeStdLogicUnsignedLtSlvInt,
    /// `"<"` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_LT_SLV_SLV")]
    IeeeStdLogicUnsignedLtSlvSlv,
    /// `"*"` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_MUL_SLV_SLV")]
    IeeeStdLogicUnsignedMulSlvSlv,
    /// `"/="` (`integer`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_NE_INT_SLV")]
    IeeeStdLogicUnsignedNeIntSlv,
    /// `"/="` (`std_logic_vector`, `integer`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_NE_SLV_INT")]
    IeeeStdLogicUnsignedNeSlvInt,
    /// `"/="` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_NE_SLV_SLV")]
    IeeeStdLogicUnsignedNeSlvSlv,
    /// `shift_left` (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SHL")]
    IeeeStdLogicUnsignedShl,
    /// `shift_right` (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SHR")]
    IeeeStdLogicUnsignedShr,
    /// `"-"` (`integer`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SUB_INT_SLV")]
    IeeeStdLogicUnsignedSubIntSlv,
    /// `"-"` (`std_ulogic`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SUB_LOG_SLV")]
    IeeeStdLogicUnsignedSubLogSlv,
    /// `"-"` (`std_logic_vector`, `integer`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SUB_SLV_INT")]
    IeeeStdLogicUnsignedSubSlvInt,
    /// `"-"` (`std_logic_vector`, `std_ulogic`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SUB_SLV_LOG")]
    IeeeStdLogicUnsignedSubSlvLog,
    /// `"-"` (`std_logic_vector`, `std_logic_vector`) (`ieee.std_logic_unsigned`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_STD_LOGIC_UNSIGNED_SUB_SLV_SLV")]
    IeeeStdLogicUnsignedSubSlvSlv,
    /// `"abs"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_ABSOLUTE")]
    IntegerAbsolute,
    /// `"/"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_DIV")]
    IntegerDiv,
    /// `"="` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_EQUALITY")]
    IntegerEquality,
    /// `"**"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_EXP")]
    IntegerExp,
    /// `">"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_GREATER")]
    IntegerGreater,
    /// `">="` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_GREATER_EQUAL")]
    IntegerGreaterEqual,
    /// unary `"+"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_IDENTITY")]
    IntegerIdentity,
    /// `"/="` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_INEQUALITY")]
    IntegerInequality,
    /// `"<"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_LESS")]
    IntegerLess,
    /// `"<="` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_LESS_EQUAL")]
    IntegerLessEqual,
    /// `maximum` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_MAXIMUM")]
    IntegerMaximum,
    /// `minimum` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_MINIMUM")]
    IntegerMinimum,
    /// `"-"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_MINUS")]
    IntegerMinus,
    /// `"mod"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_MOD")]
    IntegerMod,
    /// `"*"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_MUL")]
    IntegerMul,
    /// unary `"-"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_NEGATION")]
    IntegerNegation,
    /// `"*"` (`integer`, `physical`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_PHYSICAL_MUL")]
    IntegerPhysicalMul,
    /// `"+"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_PLUS")]
    IntegerPlus,
    /// `"rem"` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_REM")]
    IntegerRem,
    /// `to_string` (`integer`)
    #[serde(rename = "IIR_PREDEFINED_INTEGER_TO_STRING")]
    IntegerToString,
    /// `now` (`STANDARD`)
    #[serde(rename = "IIR_PREDEFINED_NOW_FUNCTION")]
    NowFunction,
    /// `"abs"` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_ABSOLUTE")]
    PhysicalAbsolute,
    /// `"="` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_EQUALITY")]
    PhysicalEquality,
    /// `">"` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_GREATER")]
    PhysicalGreater,
    /// `">="` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_GREATER_EQUAL")]
    PhysicalGreaterEqual,
    /// unary `"+"` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_IDENTITY")]
    PhysicalIdentity,
    /// `"/="` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_INEQUALITY")]
    PhysicalInequality,
    /// `"/"` (`physical`, `integer`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_INTEGER_DIV")]
    PhysicalIntegerDiv,
    /// `"*"` (`physical`, `integer`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_INTEGER_MUL")]
    PhysicalIntegerMul,
    /// `"<"` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_LESS")]
    PhysicalLess,
    /// `"<="` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_LESS_EQUAL")]
    PhysicalLessEqual,
    /// `maximum` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_MAXIMUM")]
    PhysicalMaximum,
    /// `minimum` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_MINIMUM")]
    PhysicalMinimum,
    /// `"-"` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_MINUS")]
    PhysicalMinus,
    /// `"mod"` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_MOD")]
    PhysicalMod,
    /// unary `"-"` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_NEGATION")]
    PhysicalNegation,
    /// `"/"` (`physical`, `physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_PHYSICAL_DIV")]
    PhysicalPhysicalDiv,
    /// `"+"` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_PLUS")]
    PhysicalPlus,
    /// `"/"` (`physical`, `real`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_REAL_DIV")]
    PhysicalRealDiv,
    /// `"*"` (`physical`, `real`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_REAL_MUL")]
    PhysicalRealMul,
    /// `"rem"` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_REM")]
    PhysicalRem,
    /// `to_string` (`physical`)
    #[serde(rename = "IIR_PREDEFINED_PHYSICAL_TO_STRING")]
    PhysicalToString,
    /// `read` (`file`)
    #[serde(rename = "IIR_PREDEFINED_READ")]
    Read,
    /// `read` (`file` → length)
    #[serde(rename = "IIR_PREDEFINED_READ_LENGTH")]
    ReadLength,
    /// `now` (`real`; AMS)
    #[serde(rename = "IIR_PREDEFINED_REAL_NOW_FUNCTION")]
    RealNowFunction,
    /// `"*"` (`real`, `physical`)
    #[serde(rename = "IIR_PREDEFINED_REAL_PHYSICAL_MUL")]
    RealPhysicalMul,
    /// `to_string(value, digits)` (`real`)
    #[serde(rename = "IIR_PREDEFINED_REAL_TO_STRING_DIGITS")]
    RealToStringDigits,
    /// `to_string(value, format)` (`real`)
    #[serde(rename = "IIR_PREDEFINED_REAL_TO_STRING_FORMAT")]
    RealToStringFormat,
    /// `"="` (`record`)
    #[serde(rename = "IIR_PREDEFINED_RECORD_EQUALITY")]
    RecordEquality,
    /// `"/="` (`record`)
    #[serde(rename = "IIR_PREDEFINED_RECORD_INEQUALITY")]
    RecordInequality,
    /// `finish` (`STD.ENV`)
    #[serde(rename = "IIR_PREDEFINED_STD_ENV_FINISH")]
    StdEnvFinish,
    /// `finish(status)` (`STD.ENV`)
    #[serde(rename = "IIR_PREDEFINED_STD_ENV_FINISH_STATUS")]
    StdEnvFinishStatus,
    /// `resolution_limit` (`STD.ENV`)
    #[serde(rename = "IIR_PREDEFINED_STD_ENV_RESOLUTION_LIMIT")]
    StdEnvResolutionLimit,
    /// `stop` (`STD.ENV`)
    #[serde(rename = "IIR_PREDEFINED_STD_ENV_STOP")]
    StdEnvStop,
    /// `stop(status)` (`STD.ENV`)
    #[serde(rename = "IIR_PREDEFINED_STD_ENV_STOP_STATUS")]
    StdEnvStopStatus,
    /// `"?="` (`std_ulogic` array)
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_ARRAY_MATCH_EQUALITY")]
    StdUlogicArrayMatchEquality,
    /// `"?/="` (`std_ulogic` array)
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_ARRAY_MATCH_INEQUALITY")]
    StdUlogicArrayMatchInequality,
    /// `"?="` (`std_ulogic`)
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_EQUALITY")]
    StdUlogicMatchEquality,
    /// `"?>"` (`std_ulogic`)
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_GREATER")]
    StdUlogicMatchGreater,
    /// `"?>="` (`std_ulogic`)
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_GREATER_EQUAL")]
    StdUlogicMatchGreaterEqual,
    /// `"?/="` (`std_ulogic`)
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_INEQUALITY")]
    StdUlogicMatchInequality,
    /// `"?<"` (`std_ulogic`)
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_LESS")]
    StdUlogicMatchLess,
    /// `"?<="` (`std_ulogic`)
    #[serde(rename = "IIR_PREDEFINED_STD_ULOGIC_MATCH_LESS_EQUAL")]
    StdUlogicMatchLessEqual,
    /// `"and"` (array, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_AND")]
    TfArrayAnd,
    /// `"and"` (array, element)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_AND")]
    TfArrayElementAnd,
    /// `"nand"` (array, element)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_NAND")]
    TfArrayElementNand,
    /// `"nor"` (array, element)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_NOR")]
    TfArrayElementNor,
    /// `"or"` (array, element)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_OR")]
    TfArrayElementOr,
    /// `"xnor"` (array, element)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_XNOR")]
    TfArrayElementXnor,
    /// `"xor"` (array, element)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_ELEMENT_XOR")]
    TfArrayElementXor,
    /// `"nand"` (array, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_NAND")]
    TfArrayNand,
    /// `"nor"` (array, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_NOR")]
    TfArrayNor,
    /// `"not"` (array)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_NOT")]
    TfArrayNot,
    /// `"or"` (array, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_OR")]
    TfArrayOr,
    /// `"xnor"` (array, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_XNOR")]
    TfArrayXnor,
    /// `"xor"` (array, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ARRAY_XOR")]
    TfArrayXor,
    /// `"and"` (element, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_AND")]
    TfElementArrayAnd,
    /// `"nand"` (element, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_NAND")]
    TfElementArrayNand,
    /// `"nor"` (element, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_NOR")]
    TfElementArrayNor,
    /// `"or"` (element, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_OR")]
    TfElementArrayOr,
    /// `"xnor"` (element, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_XNOR")]
    TfElementArrayXnor,
    /// `"xor"` (element, array)
    #[serde(rename = "IIR_PREDEFINED_TF_ELEMENT_ARRAY_XOR")]
    TfElementArrayXor,
    /// reduction `"and"` (array)
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_AND")]
    TfReductionAnd,
    /// reduction `"nand"` (array)
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_NAND")]
    TfReductionNand,
    /// reduction `"nor"` (array)
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_NOR")]
    TfReductionNor,
    /// reduction `"not"` (array)
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_NOT")]
    TfReductionNot,
    /// reduction `"or"` (array)
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_OR")]
    TfReductionOr,
    /// reduction `"xnor"` (array)
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_XNOR")]
    TfReductionXnor,
    /// reduction `"xor"` (array)
    #[serde(rename = "IIR_PREDEFINED_TF_REDUCTION_XOR")]
    TfReductionXor,
    /// `to_string(value, unit)` (`time`)
    #[serde(rename = "IIR_PREDEFINED_TIME_TO_STRING_UNIT")]
    TimeToStringUnit,
    /// `"*"` (`universal_integer`, `universal_real`)
    #[serde(rename = "IIR_PREDEFINED_UNIVERSAL_I_R_MUL")]
    UniversalIRMul,
    /// `"/"` (`universal_real`, `universal_integer`)
    #[serde(rename = "IIR_PREDEFINED_UNIVERSAL_R_I_DIV")]
    UniversalRIDiv,
    /// `"*"` (`universal_real`, `universal_integer`)
    #[serde(rename = "IIR_PREDEFINED_UNIVERSAL_R_I_MUL")]
    UniversalRIMul,
    /// `maximum` (vector)
    #[serde(rename = "IIR_PREDEFINED_VECTOR_MAXIMUM")]
    VectorMaximum,
    /// `minimum` (vector)
    #[serde(rename = "IIR_PREDEFINED_VECTOR_MINIMUM")]
    VectorMinimum,
    /// `write` (`file`)
    #[serde(rename = "IIR_PREDEFINED_WRITE")]
    Write,
    /// `T'image` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_IMAGE")]
    AttributeImage,
    /// `T'value` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_VALUE")]
    AttributeValue,
    /// `T'pos` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_POS")]
    AttributePos,
    /// `T'val` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_VAL")]
    AttributeVal,
    /// `T'succ` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_SUCC")]
    AttributeSucc,
    /// `T'pred` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_PRED")]
    AttributePred,
    /// `T'leftof` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_LEFTOF")]
    AttributeLeftof,
    /// `T'rightof` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_RIGHTOF")]
    AttributeRightof,
    /// `T'left` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_LEFT")]
    AttributeLeft,
    /// `T'right` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_RIGHT")]
    AttributeRight,
    /// `S'event` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_EVENT")]
    AttributeEvent,
    /// `S'active` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_ACTIVE")]
    AttributeActive,
    /// `S'last_event` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_LAST_EVENT")]
    AttributeLastEvent,
    /// `S'last_active` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_LAST_ACTIVE")]
    AttributeLastActive,
    /// `S'last_value` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_LAST_VALUE")]
    AttributeLastValue,
    /// `S'driving` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_DRIVING")]
    AttributeDriving,
    /// `S'driving_value` (predefined attribute function)
    #[serde(rename = "IIR_PREDEFINED_ATTRIBUTE_DRIVING_VALUE")]
    AttributeDrivingValue,
    /// `"not"` (`unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NOT_UNS")]
    IeeeNumericBitNotUns,
    /// `"not"` (`signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NOT_SGN")]
    IeeeNumericBitNotSgn,
    /// `"abs"` (`signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ABS_SGN")]
    IeeeNumericBitAbsSgn,
    /// unary `"-"` (`signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NEG_SGN")]
    IeeeNumericBitNegSgn,
    /// `"+"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ADD_UNS_UNS")]
    IeeeNumericBitAddUnsUns,
    /// `"+"` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ADD_UNS_NAT")]
    IeeeNumericBitAddUnsNat,
    /// `"+"` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ADD_NAT_UNS")]
    IeeeNumericBitAddNatUns,
    /// `"+"` (`unsigned`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ADD_UNS_BIT")]
    IeeeNumericBitAddUnsBit,
    /// `"+"` (`bit`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ADD_BIT_UNS")]
    IeeeNumericBitAddBitUns,
    /// `"+"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ADD_SGN_SGN")]
    IeeeNumericBitAddSgnSgn,
    /// `"+"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ADD_SGN_INT")]
    IeeeNumericBitAddSgnInt,
    /// `"+"` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ADD_INT_SGN")]
    IeeeNumericBitAddIntSgn,
    /// `"+"` (`signed`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ADD_SGN_BIT")]
    IeeeNumericBitAddSgnBit,
    /// `"+"` (`bit`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ADD_BIT_SGN")]
    IeeeNumericBitAddBitSgn,
    /// `"-"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SUB_UNS_UNS")]
    IeeeNumericBitSubUnsUns,
    /// `"-"` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SUB_UNS_NAT")]
    IeeeNumericBitSubUnsNat,
    /// `"-"` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SUB_NAT_UNS")]
    IeeeNumericBitSubNatUns,
    /// `"-"` (`unsigned`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SUB_UNS_BIT")]
    IeeeNumericBitSubUnsBit,
    /// `"-"` (`bit`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SUB_BIT_UNS")]
    IeeeNumericBitSubBitUns,
    /// `"-"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SUB_SGN_SGN")]
    IeeeNumericBitSubSgnSgn,
    /// `"-"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SUB_SGN_INT")]
    IeeeNumericBitSubSgnInt,
    /// `"-"` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SUB_INT_SGN")]
    IeeeNumericBitSubIntSgn,
    /// `"-"` (`signed`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SUB_SGN_BIT")]
    IeeeNumericBitSubSgnBit,
    /// `"-"` (`bit`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SUB_BIT_SGN")]
    IeeeNumericBitSubBitSgn,
    /// `"*"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MUL_UNS_UNS")]
    IeeeNumericBitMulUnsUns,
    /// `"*"` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MUL_UNS_NAT")]
    IeeeNumericBitMulUnsNat,
    /// `"*"` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MUL_NAT_UNS")]
    IeeeNumericBitMulNatUns,
    /// `"*"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MUL_SGN_SGN")]
    IeeeNumericBitMulSgnSgn,
    /// `"*"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MUL_SGN_INT")]
    IeeeNumericBitMulSgnInt,
    /// `"*"` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MUL_INT_SGN")]
    IeeeNumericBitMulIntSgn,
    /// `"/"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_DIV_UNS_UNS")]
    IeeeNumericBitDivUnsUns,
    /// `"/"` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_DIV_UNS_NAT")]
    IeeeNumericBitDivUnsNat,
    /// `"/"` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_DIV_NAT_UNS")]
    IeeeNumericBitDivNatUns,
    /// `"/"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_DIV_SGN_SGN")]
    IeeeNumericBitDivSgnSgn,
    /// `"/"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_DIV_SGN_INT")]
    IeeeNumericBitDivSgnInt,
    /// `"/"` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_DIV_INT_SGN")]
    IeeeNumericBitDivIntSgn,
    /// `"rem"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_REM_UNS_UNS")]
    IeeeNumericBitRemUnsUns,
    /// `"rem"` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_REM_UNS_NAT")]
    IeeeNumericBitRemUnsNat,
    /// `"rem"` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_REM_NAT_UNS")]
    IeeeNumericBitRemNatUns,
    /// `"rem"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_REM_SGN_SGN")]
    IeeeNumericBitRemSgnSgn,
    /// `"rem"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_REM_SGN_INT")]
    IeeeNumericBitRemSgnInt,
    /// `"rem"` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_REM_INT_SGN")]
    IeeeNumericBitRemIntSgn,
    /// `"mod"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MOD_UNS_UNS")]
    IeeeNumericBitModUnsUns,
    /// `"mod"` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MOD_UNS_NAT")]
    IeeeNumericBitModUnsNat,
    /// `"mod"` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MOD_NAT_UNS")]
    IeeeNumericBitModNatUns,
    /// `"mod"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MOD_SGN_SGN")]
    IeeeNumericBitModSgnSgn,
    /// `"mod"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MOD_SGN_INT")]
    IeeeNumericBitModSgnInt,
    /// `"mod"` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MOD_INT_SGN")]
    IeeeNumericBitModIntSgn,
    /// `">"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GT_UNS_UNS")]
    IeeeNumericBitGtUnsUns,
    /// `">"` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GT_UNS_NAT")]
    IeeeNumericBitGtUnsNat,
    /// `">"` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GT_NAT_UNS")]
    IeeeNumericBitGtNatUns,
    /// `">"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GT_SGN_SGN")]
    IeeeNumericBitGtSgnSgn,
    /// `">"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GT_SGN_INT")]
    IeeeNumericBitGtSgnInt,
    /// `">"` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GT_INT_SGN")]
    IeeeNumericBitGtIntSgn,
    /// `"<"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LT_UNS_UNS")]
    IeeeNumericBitLtUnsUns,
    /// `"<"` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LT_UNS_NAT")]
    IeeeNumericBitLtUnsNat,
    /// `"<"` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LT_NAT_UNS")]
    IeeeNumericBitLtNatUns,
    /// `"<"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LT_SGN_SGN")]
    IeeeNumericBitLtSgnSgn,
    /// `"<"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LT_SGN_INT")]
    IeeeNumericBitLtSgnInt,
    /// `"<"` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LT_INT_SGN")]
    IeeeNumericBitLtIntSgn,
    /// `"<="` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LE_UNS_UNS")]
    IeeeNumericBitLeUnsUns,
    /// `"<="` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LE_UNS_NAT")]
    IeeeNumericBitLeUnsNat,
    /// `"<="` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LE_NAT_UNS")]
    IeeeNumericBitLeNatUns,
    /// `"<="` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LE_SGN_SGN")]
    IeeeNumericBitLeSgnSgn,
    /// `"<="` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LE_SGN_INT")]
    IeeeNumericBitLeSgnInt,
    /// `"<="` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_LE_INT_SGN")]
    IeeeNumericBitLeIntSgn,
    /// `">="` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GE_UNS_UNS")]
    IeeeNumericBitGeUnsUns,
    /// `">="` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GE_UNS_NAT")]
    IeeeNumericBitGeUnsNat,
    /// `">="` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GE_NAT_UNS")]
    IeeeNumericBitGeNatUns,
    /// `">="` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GE_SGN_SGN")]
    IeeeNumericBitGeSgnSgn,
    /// `">="` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GE_SGN_INT")]
    IeeeNumericBitGeSgnInt,
    /// `">="` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_GE_INT_SGN")]
    IeeeNumericBitGeIntSgn,
    /// `"="` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_EQ_UNS_UNS")]
    IeeeNumericBitEqUnsUns,
    /// `"="` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_EQ_UNS_NAT")]
    IeeeNumericBitEqUnsNat,
    /// `"="` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_EQ_NAT_UNS")]
    IeeeNumericBitEqNatUns,
    /// `"="` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_EQ_SGN_SGN")]
    IeeeNumericBitEqSgnSgn,
    /// `"="` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_EQ_SGN_INT")]
    IeeeNumericBitEqSgnInt,
    /// `"="` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_EQ_INT_SGN")]
    IeeeNumericBitEqIntSgn,
    /// `"/="` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NE_UNS_UNS")]
    IeeeNumericBitNeUnsUns,
    /// `"/="` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NE_UNS_NAT")]
    IeeeNumericBitNeUnsNat,
    /// `"/="` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NE_NAT_UNS")]
    IeeeNumericBitNeNatUns,
    /// `"/="` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NE_SGN_SGN")]
    IeeeNumericBitNeSgnSgn,
    /// `"/="` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NE_SGN_INT")]
    IeeeNumericBitNeSgnInt,
    /// `"/="` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NE_INT_SGN")]
    IeeeNumericBitNeIntSgn,
    /// `"?>"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GT_UNS_UNS")]
    IeeeNumericBitMatchGtUnsUns,
    /// `"?>"` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GT_UNS_NAT")]
    IeeeNumericBitMatchGtUnsNat,
    /// `"?>"` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GT_NAT_UNS")]
    IeeeNumericBitMatchGtNatUns,
    /// `"?>"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GT_SGN_SGN")]
    IeeeNumericBitMatchGtSgnSgn,
    /// `"?>"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GT_SGN_INT")]
    IeeeNumericBitMatchGtSgnInt,
    /// `"?>"` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GT_INT_SGN")]
    IeeeNumericBitMatchGtIntSgn,
    /// `"?<"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LT_UNS_UNS")]
    IeeeNumericBitMatchLtUnsUns,
    /// `"?<"` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LT_UNS_NAT")]
    IeeeNumericBitMatchLtUnsNat,
    /// `"?<"` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LT_NAT_UNS")]
    IeeeNumericBitMatchLtNatUns,
    /// `"?<"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LT_SGN_SGN")]
    IeeeNumericBitMatchLtSgnSgn,
    /// `"?<"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LT_SGN_INT")]
    IeeeNumericBitMatchLtSgnInt,
    /// `"?<"` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LT_INT_SGN")]
    IeeeNumericBitMatchLtIntSgn,
    /// `"?<="` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LE_UNS_UNS")]
    IeeeNumericBitMatchLeUnsUns,
    /// `"?<="` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LE_UNS_NAT")]
    IeeeNumericBitMatchLeUnsNat,
    /// `"?<="` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LE_NAT_UNS")]
    IeeeNumericBitMatchLeNatUns,
    /// `"?<="` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LE_SGN_SGN")]
    IeeeNumericBitMatchLeSgnSgn,
    /// `"?<="` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LE_SGN_INT")]
    IeeeNumericBitMatchLeSgnInt,
    /// `"?<="` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_LE_INT_SGN")]
    IeeeNumericBitMatchLeIntSgn,
    /// `"?>="` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GE_UNS_UNS")]
    IeeeNumericBitMatchGeUnsUns,
    /// `"?>="` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GE_UNS_NAT")]
    IeeeNumericBitMatchGeUnsNat,
    /// `"?>="` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GE_NAT_UNS")]
    IeeeNumericBitMatchGeNatUns,
    /// `"?>="` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GE_SGN_SGN")]
    IeeeNumericBitMatchGeSgnSgn,
    /// `"?>="` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GE_SGN_INT")]
    IeeeNumericBitMatchGeSgnInt,
    /// `"?>="` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_GE_INT_SGN")]
    IeeeNumericBitMatchGeIntSgn,
    /// `"?="` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_EQ_UNS_UNS")]
    IeeeNumericBitMatchEqUnsUns,
    /// `"?="` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_EQ_UNS_NAT")]
    IeeeNumericBitMatchEqUnsNat,
    /// `"?="` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_EQ_NAT_UNS")]
    IeeeNumericBitMatchEqNatUns,
    /// `"?="` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_EQ_SGN_SGN")]
    IeeeNumericBitMatchEqSgnSgn,
    /// `"?="` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_EQ_SGN_INT")]
    IeeeNumericBitMatchEqSgnInt,
    /// `"?="` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_EQ_INT_SGN")]
    IeeeNumericBitMatchEqIntSgn,
    /// `"?/="` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_NE_UNS_UNS")]
    IeeeNumericBitMatchNeUnsUns,
    /// `"?/="` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_NE_UNS_NAT")]
    IeeeNumericBitMatchNeUnsNat,
    /// `"?/="` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_NE_NAT_UNS")]
    IeeeNumericBitMatchNeNatUns,
    /// `"?/="` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_NE_SGN_SGN")]
    IeeeNumericBitMatchNeSgnSgn,
    /// `"?/="` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_NE_SGN_INT")]
    IeeeNumericBitMatchNeSgnInt,
    /// `"?/="` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MATCH_NE_INT_SGN")]
    IeeeNumericBitMatchNeIntSgn,
    /// `shift_left` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SHF_LEFT_UNS_NAT")]
    IeeeNumericBitShfLeftUnsNat,
    /// `shift_right` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SHF_RIGHT_UNS_NAT")]
    IeeeNumericBitShfRightUnsNat,
    /// `shift_left` (`signed`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SHF_LEFT_SGN_NAT")]
    IeeeNumericBitShfLeftSgnNat,
    /// `shift_right` (`signed`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SHF_RIGHT_SGN_NAT")]
    IeeeNumericBitShfRightSgnNat,
    /// `rotate_left` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ROT_LEFT_UNS_NAT")]
    IeeeNumericBitRotLeftUnsNat,
    /// `rotate_right` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ROT_RIGHT_UNS_NAT")]
    IeeeNumericBitRotRightUnsNat,
    /// `rotate_left` (`signed`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ROT_LEFT_SGN_NAT")]
    IeeeNumericBitRotLeftSgnNat,
    /// `rotate_right` (`signed`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ROT_RIGHT_SGN_NAT")]
    IeeeNumericBitRotRightSgnNat,
    /// `resize` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_RESIZE_UNS_NAT")]
    IeeeNumericBitResizeUnsNat,
    /// `resize` (`signed`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_RESIZE_SGN_NAT")]
    IeeeNumericBitResizeSgnNat,
    /// `resize` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_RESIZE_UNS_UNS")]
    IeeeNumericBitResizeUnsUns,
    /// `resize` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_RESIZE_SGN_SGN")]
    IeeeNumericBitResizeSgnSgn,
    /// `"and"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_AND_UNS_UNS")]
    IeeeNumericBitAndUnsUns,
    /// `"and"` (`unsigned`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_AND_UNS_BIT")]
    IeeeNumericBitAndUnsBit,
    /// `"and"` (`bit`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_AND_BIT_UNS")]
    IeeeNumericBitAndBitUns,
    /// `"and"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_AND_SGN_SGN")]
    IeeeNumericBitAndSgnSgn,
    /// `"and"` (`signed`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_AND_SGN_BIT")]
    IeeeNumericBitAndSgnBit,
    /// `"and"` (`bit`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_AND_BIT_SGN")]
    IeeeNumericBitAndBitSgn,
    /// `"nand"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NAND_UNS_UNS")]
    IeeeNumericBitNandUnsUns,
    /// `"nand"` (`unsigned`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NAND_UNS_BIT")]
    IeeeNumericBitNandUnsBit,
    /// `"nand"` (`bit`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NAND_BIT_UNS")]
    IeeeNumericBitNandBitUns,
    /// `"nand"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NAND_SGN_SGN")]
    IeeeNumericBitNandSgnSgn,
    /// `"nand"` (`signed`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NAND_SGN_BIT")]
    IeeeNumericBitNandSgnBit,
    /// `"nand"` (`bit`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NAND_BIT_SGN")]
    IeeeNumericBitNandBitSgn,
    /// `"or"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_OR_UNS_UNS")]
    IeeeNumericBitOrUnsUns,
    /// `"or"` (`unsigned`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_OR_UNS_BIT")]
    IeeeNumericBitOrUnsBit,
    /// `"or"` (`bit`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_OR_BIT_UNS")]
    IeeeNumericBitOrBitUns,
    /// `"or"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_OR_SGN_SGN")]
    IeeeNumericBitOrSgnSgn,
    /// `"or"` (`signed`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_OR_SGN_BIT")]
    IeeeNumericBitOrSgnBit,
    /// `"or"` (`bit`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_OR_BIT_SGN")]
    IeeeNumericBitOrBitSgn,
    /// `"nor"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NOR_UNS_UNS")]
    IeeeNumericBitNorUnsUns,
    /// `"nor"` (`unsigned`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NOR_UNS_BIT")]
    IeeeNumericBitNorUnsBit,
    /// `"nor"` (`bit`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NOR_BIT_UNS")]
    IeeeNumericBitNorBitUns,
    /// `"nor"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NOR_SGN_SGN")]
    IeeeNumericBitNorSgnSgn,
    /// `"nor"` (`signed`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NOR_SGN_BIT")]
    IeeeNumericBitNorSgnBit,
    /// `"nor"` (`bit`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_NOR_BIT_SGN")]
    IeeeNumericBitNorBitSgn,
    /// `"xor"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XOR_UNS_UNS")]
    IeeeNumericBitXorUnsUns,
    /// `"xor"` (`unsigned`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XOR_UNS_BIT")]
    IeeeNumericBitXorUnsBit,
    /// `"xor"` (`bit`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XOR_BIT_UNS")]
    IeeeNumericBitXorBitUns,
    /// `"xor"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XOR_SGN_SGN")]
    IeeeNumericBitXorSgnSgn,
    /// `"xor"` (`signed`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XOR_SGN_BIT")]
    IeeeNumericBitXorSgnBit,
    /// `"xor"` (`bit`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XOR_BIT_SGN")]
    IeeeNumericBitXorBitSgn,
    /// `"xnor"` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XNOR_UNS_UNS")]
    IeeeNumericBitXnorUnsUns,
    /// `"xnor"` (`unsigned`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XNOR_UNS_BIT")]
    IeeeNumericBitXnorUnsBit,
    /// `"xnor"` (`bit`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XNOR_BIT_UNS")]
    IeeeNumericBitXnorBitUns,
    /// `"xnor"` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XNOR_SGN_SGN")]
    IeeeNumericBitXnorSgnSgn,
    /// `"xnor"` (`signed`, `bit`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XNOR_SGN_BIT")]
    IeeeNumericBitXnorSgnBit,
    /// `"xnor"` (`bit`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_XNOR_BIT_SGN")]
    IeeeNumericBitXnorBitSgn,
    /// `"sll"` (`unsigned`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SLL_UNS_INT")]
    IeeeNumericBitSllUnsInt,
    /// `"sll"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SLL_SGN_INT")]
    IeeeNumericBitSllSgnInt,
    /// `"srl"` (`unsigned`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SRL_UNS_INT")]
    IeeeNumericBitSrlUnsInt,
    /// `"srl"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SRL_SGN_INT")]
    IeeeNumericBitSrlSgnInt,
    /// `"sla"` (`unsigned`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SLA_UNS_INT")]
    IeeeNumericBitSlaUnsInt,
    /// `"sla"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SLA_SGN_INT")]
    IeeeNumericBitSlaSgnInt,
    /// `"sra"` (`unsigned`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SRA_UNS_INT")]
    IeeeNumericBitSraUnsInt,
    /// `"sra"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_SRA_SGN_INT")]
    IeeeNumericBitSraSgnInt,
    /// `"rol"` (`unsigned`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ROL_UNS_INT")]
    IeeeNumericBitRolUnsInt,
    /// `"rol"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ROL_SGN_INT")]
    IeeeNumericBitRolSgnInt,
    /// `"ror"` (`unsigned`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ROR_UNS_INT")]
    IeeeNumericBitRorUnsInt,
    /// `"ror"` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_ROR_SGN_INT")]
    IeeeNumericBitRorSgnInt,
    /// `find_leftmost` (`unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_FIND_LEFTMOST_UNS")]
    IeeeNumericBitFindLeftmostUns,
    /// `find_rightmost` (`unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_FIND_RIGHTMOST_UNS")]
    IeeeNumericBitFindRightmostUns,
    /// `find_leftmost` (`signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_FIND_LEFTMOST_SGN")]
    IeeeNumericBitFindLeftmostSgn,
    /// `find_rightmost` (`signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_FIND_RIGHTMOST_SGN")]
    IeeeNumericBitFindRightmostSgn,
    /// `minimum` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MIN_UNS_UNS")]
    IeeeNumericBitMinUnsUns,
    /// `minimum` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MIN_UNS_NAT")]
    IeeeNumericBitMinUnsNat,
    /// `minimum` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MIN_NAT_UNS")]
    IeeeNumericBitMinNatUns,
    /// `minimum` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MIN_SGN_SGN")]
    IeeeNumericBitMinSgnSgn,
    /// `minimum` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MIN_SGN_INT")]
    IeeeNumericBitMinSgnInt,
    /// `minimum` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MIN_INT_SGN")]
    IeeeNumericBitMinIntSgn,
    /// `maximum` (`unsigned`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MAX_UNS_UNS")]
    IeeeNumericBitMaxUnsUns,
    /// `maximum` (`unsigned`, `natural`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MAX_UNS_NAT")]
    IeeeNumericBitMaxUnsNat,
    /// `maximum` (`natural`, `unsigned`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MAX_NAT_UNS")]
    IeeeNumericBitMaxNatUns,
    /// `maximum` (`signed`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MAX_SGN_SGN")]
    IeeeNumericBitMaxSgnSgn,
    /// `maximum` (`signed`, `integer`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MAX_SGN_INT")]
    IeeeNumericBitMaxSgnInt,
    /// `maximum` (`integer`, `signed`) (`ieee.numeric_bit`)
    #[serde(rename = "IIR_PREDEFINED_IEEE_NUMERIC_BIT_MAX_INT_SGN")]
    IeeeNumericBitMaxIntSgn,
    /// Unrecognized or `Iir_Predefined_None`.
    Unknown,
}

/// Record element declaration (`identifier_list : subtype_indication`).
///
/// ```vhdl
/// type rec_t is record
///   a, b : integer;           -- two ElementDeclaration nodes
///   data : std_logic_vector(7 downto 0);
/// end record;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ElementDeclaration {
    /// Element identifier.
    pub identifier: Identifier,
    /// Subtype indication of the element.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed type of the element.
    #[serde(rename = "type")]
    pub typ: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Zero-based position in the record type.
    pub element_position: Option<i32>,
}

/// File declaration (`file … : … is …`).
///
/// ```vhdl
/// file f : text open read_mode is "input.txt";
/// file g : text is in "data.txt";
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FileDeclaration {
    /// File identifier.
    pub identifier: Option<Identifier>,
    /// Analyzed file type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Subtype indication of the file type mark.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Explicit mode when written (`in` / `out`).
    pub mode: Option<Mode>,
    /// Logical name expression (file path string expression).
    pub file_logical_name: Option<ExpressionNodeId>,
    /// Open-kind expression (`read_mode`, `write_mode`, …).
    pub file_open_kind: Option<ExpressionNodeId>,
}

/// Object alias declaration (`alias … : … is object_name`).
///
/// Creates another name for an existing object (signal, variable, constant, …).
///
/// ```vhdl
/// alias msb : std_logic is data(7);
/// alias all_bits : std_logic_vector(7 downto 0) is data;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectAliasDeclaration {
    /// Alias identifier.
    pub identifier: Option<Identifier>,
    /// Name of the aliased object (possibly sliced / selected).
    pub name: Option<NameNodeId>,
    /// Optional subtype indication of the alias.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed type of the alias.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// Component declaration (`component … is … end component`).
///
/// ```vhdl
/// component nand2 is
///   generic (Tpd : time := 1 ns);
///   port (a, b : in std_logic; y : out std_logic);
/// end component;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentDeclaration {
    /// Component identifier.
    pub identifier: Option<Identifier>,
    /// Generic interface list.
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
    /// Port interface list.
    #[serde(default)]
    pub ports: Vec<PortInterfaceDeclarationNodeId>,
}

/// Iterator / parameter specification of a `for` loop or `for` generate.
///
/// ```vhdl
/// for i in 0 to 7 loop … end loop;
/// for g in generate_range generate … end generate;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IteratorDeclaration {
    /// Iterator identifier.
    pub identifier: Option<Identifier>,
    /// Discrete range the iterator covers.
    pub discrete_range: Option<RangeConstraintNodeId>,
    /// Subtype indication of the iterator.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed type of the iterator.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// Implicit guard signal of a guarded block (`block (guard_expr)`).
///
/// ```vhdl
/// b: block (en = '1')
/// begin
///   s <= guarded d;
/// end block;
/// -- GuardSignalDeclaration holds the implicit GUARD signal
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct GuardSignalDeclaration {
    /// Guard signal identifier (typically `guard`).
    pub identifier: Option<Identifier>,
    /// Analyzed type (boolean).
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Guard expression from the block header.
    pub guard_expression: Option<ExpressionNodeId>,
    /// Signals read by the guard expression.
    #[serde(default)]
    pub guard_sensitivity_list: Vec<ExpressionNodeId>,
    /// Whether the guard is treated as a guarded signal.
    #[serde(default)]
    pub guarded_signal_flag: bool,
    /// `register` / `bus` kind when applicable.
    pub signal_kind: Option<SignalKind>,
    /// Owning block statement.
    pub block_statement: Option<GenericNodeId>,
}

/// Chain of implicitly declared attribute objects for a design region.
///
/// GHDL inserts these to hold predefined attribute state; user code does not
/// declare them explicitly.
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeImplicitDeclaration {
    /// Linked implicit attribute objects.
    #[serde(default)]
    pub attribute_implicits: Vec<GenericNodeId>,
}

/// Nature declaration (`nature … is …`) for VHDL-AMS.
///
/// ```vhdl
/// nature electrical is
///   voltage across
///   current through
///   ground reference;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NatureDeclaration {
    /// Nature identifier.
    pub identifier: Option<Identifier>,
    /// Nature definition.
    pub nature_definition: Option<GenericNodeId>,
}

/// Subnature declaration (`subnature … is …`).
///
/// ```vhdl
/// subnature small_elec is electrical;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SubnatureDeclaration {
    /// Subnature identifier.
    pub identifier: Option<Identifier>,
    /// Subnature indication.
    pub subnature_indication: Option<GenericNodeId>,
    /// Analyzed nature.
    pub nature: Option<GenericNodeId>,
}

/// Group template declaration (`group … is (…)`).
///
/// ```vhdl
/// group pin2pin is (signal, signal);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct GroupTemplateDeclaration {
    /// Template identifier.
    pub identifier: Option<Identifier>,
    /// Entity-class entries of the template.
    #[serde(default)]
    pub entity_class_entries: Vec<NodeId<EntityClass>>,
}

/// Group declaration (`group … : template (…)`).
///
/// ```vhdl
/// group g : pin2pin (a, b);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct GroupDeclaration {
    /// Group identifier.
    pub identifier: Option<Identifier>,
    /// Constituents of the group.
    #[serde(default)]
    pub group_constituent_list: Vec<GenericNodeId>,
    /// Name of the group template.
    pub group_template_name: Option<NameNodeId>,
}

/// Element of a record nature (`identifier : subnature`).
#[derive(Debug, Deserialize, Serialize)]
pub struct NatureElementDeclaration {
    /// Element identifier.
    pub identifier: Option<Identifier>,
    /// Subnature indication of the element.
    pub subnature_indication: Option<GenericNodeId>,
    /// Analyzed nature of the element.
    pub nature: Option<GenericNodeId>,
    /// Zero-based position in the record nature.
    pub element_position: Option<i32>,
}

/// Mode-view declaration (VHDL-2019) (`view … of … is … end view`).
///
/// ```vhdl
/// view master_view of bus_t is
///   req  : out;
///   ack  : in;
///   data : out;
/// end view;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ModeViewDeclaration {
    /// Mode-view identifier.
    pub identifier: Option<Identifier>,
    /// Subtype indication of the viewed type.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Mode-view element definitions (list form).
    #[serde(default)]
    pub elements_definition_list: Vec<GenericNodeId>,
    /// Mode-view element definitions (chain form).
    #[serde(default)]
    pub elements_definitions: Vec<GenericNodeId>,
}

/// Body of a subprogram instantiation.
///
/// Completes an instantiated function/procedure with declarations and
/// statements when an instance body is required.
#[derive(Debug, Deserialize, Serialize)]
pub struct SubprogramInstantiationBody {
    /// Declarations in the instance body.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Sequential statements of the instance body.
    #[serde(default)]
    pub sequential_statements: Vec<SequentialStatementNodeId>,
    /// Matching subprogram specification / instantiation.
    pub subprogram_specification: Option<GenericNodeId>,
}

/// Function instantiation declaration (`function … is new …`).
///
/// ```vhdl
/// function my_id is new id_fn
///   generic map (T => integer);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionInstantiationDeclaration {
    /// Instantiated function identifier.
    pub identifier: Option<Identifier>,
    /// Whether the instance is pure.
    #[serde(default)]
    pub pure_flag: bool,
    /// Generic interface list of the instance.
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
    /// Parameter interface list of the instance.
    #[serde(default)]
    pub interface_declarations: Vec<InterfaceDeclarationNodeId>,
    /// Return subtype after analysis.
    pub return_type: Option<SubtypeDefinitionNodeId>,
    /// Uninstantiated function being instantiated.
    pub uninstantiated_subprogram_name: Option<NameNodeId>,
    /// Generic map associations.
    #[serde(default)]
    pub generic_map_aspects: Vec<AssociationElementNodeId>,
    /// Instance body, when present.
    pub instance_subprogram_body: Option<NodeId<SubprogramInstantiationBody>>,
}

/// Procedure instantiation declaration (`procedure … is new …`).
///
/// ```vhdl
/// procedure my_pulse is new pulse_proc
///   generic map (T => std_logic);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcedureInstantiationDeclaration {
    /// Instantiated procedure identifier.
    pub identifier: Option<Identifier>,
    /// Generic interface list of the instance.
    #[serde(default)]
    pub generics: Vec<InterfaceDeclarationNodeId>,
    /// Parameter interface list of the instance.
    #[serde(default)]
    pub interface_declarations: Vec<InterfaceDeclarationNodeId>,
    /// Uninstantiated procedure being instantiated.
    pub uninstantiated_subprogram_name: Option<NameNodeId>,
    /// Generic map associations.
    #[serde(default)]
    pub generic_map_aspects: Vec<AssociationElementNodeId>,
    /// Instance body, when present.
    pub instance_subprogram_body: Option<NodeId<SubprogramInstantiationBody>>,
}

/// Terminal declaration (VHDL-AMS) (`terminal … : …`).
///
/// ```vhdl
/// terminal n1, n2 : electrical;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct TerminalDeclaration {
    /// Terminal identifier.
    pub identifier: Option<Identifier>,
    /// Subnature indication.
    pub subnature_indication: Option<GenericNodeId>,
    /// Analyzed nature.
    pub nature: Option<GenericNodeId>,
    /// Whether this is a reference terminal.
    #[serde(default)]
    pub reference_terminal_flag: bool,
}

/// Free quantity declaration (`quantity … : …`).
///
/// ```vhdl
/// quantity q : real;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FreeQuantityDeclaration {
    /// Quantity identifier.
    pub identifier: Option<Identifier>,
    /// Subtype indication.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Default / initial value.
    pub default_value: Option<ExpressionNodeId>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// Spectrum quantity declaration (`quantity … : … spectrum …, …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct SpectrumQuantityDeclaration {
    /// Quantity identifier.
    pub identifier: Option<Identifier>,
    /// Subtype indication.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Magnitude expression.
    pub magnitude_expression: Option<ExpressionNodeId>,
    /// Phase expression.
    pub phase_expression: Option<ExpressionNodeId>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// Noise quantity declaration (`quantity … : … noise …`).
#[derive(Debug, Deserialize, Serialize)]
pub struct NoiseQuantityDeclaration {
    /// Quantity identifier.
    pub identifier: Option<Identifier>,
    /// Subtype indication.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Power expression.
    pub power_expression: Option<ExpressionNodeId>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// Across quantity declaration (`quantity … across … to …`).
///
/// ```vhdl
/// quantity v across n1 to n2;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AcrossQuantityDeclaration {
    /// Quantity identifier.
    pub identifier: Option<Identifier>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Default value expression.
    pub default_value: Option<ExpressionNodeId>,
    /// Tolerance expression.
    pub tolerance: Option<ExpressionNodeId>,
    /// Plus terminal name.
    pub plus_terminal_name: Option<NameNodeId>,
    /// Minus terminal name.
    pub minus_terminal_name: Option<NameNodeId>,
    /// Resolved plus terminal.
    pub plus_terminal: Option<GenericNodeId>,
    /// Resolved minus terminal.
    pub minus_terminal: Option<GenericNodeId>,
}

/// Through quantity declaration (`quantity … through … to …`).
///
/// ```vhdl
/// quantity i through n1 to n2;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ThroughQuantityDeclaration {
    /// Quantity identifier.
    pub identifier: Option<Identifier>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Default value expression.
    pub default_value: Option<ExpressionNodeId>,
    /// Tolerance expression.
    pub tolerance: Option<ExpressionNodeId>,
    /// Plus terminal name.
    pub plus_terminal_name: Option<NameNodeId>,
    /// Minus terminal name.
    pub minus_terminal_name: Option<NameNodeId>,
    /// Resolved plus terminal.
    pub plus_terminal: Option<GenericNodeId>,
    /// Resolved minus terminal.
    pub minus_terminal: Option<GenericNodeId>,
}

/// Interface view declaration (VHDL-2019 mode-view port).
///
/// ```vhdl
/// port (bus_if : view master_view of bus_t);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceViewDeclaration {
    /// View interface identifier.
    pub identifier: Option<Identifier>,
    /// Mode when present.
    pub mode: Option<Mode>,
    /// Subtype indication of the viewed object.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Mode-view indication.
    pub mode_view_indication: Option<GenericNodeId>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// Interface quantity declaration (AMS generic/port quantity).
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceQuantityDeclaration {
    /// Quantity interface identifier.
    pub identifier: Option<Identifier>,
    /// Mode of the quantity interface.
    pub mode: Option<Mode>,
    /// Subtype indication.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Default value.
    pub default_value: Option<ExpressionNodeId>,
    /// Analyzed type.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// Interface terminal declaration (AMS).
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceTerminalDeclaration {
    /// Terminal interface identifier.
    pub identifier: Option<Identifier>,
    /// Subnature indication.
    pub subnature_indication: Option<GenericNodeId>,
    /// Analyzed nature.
    pub nature: Option<GenericNodeId>,
}

/// Interface type declaration (`generic (type T)`).
///
/// ```vhdl
/// generic (type element_t);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceTypeDeclaration {
    /// Type generic identifier.
    pub identifier: Option<Identifier>,
    /// Interface type definition (associated actual after map).
    pub interface_type_definition: Option<NodeId<InterfaceTypeDefinition>>,
    /// Analyzed type when associated.
    #[serde(rename = "type")]
    pub typ: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Implicit subprograms associated with the type generic.
    #[serde(default)]
    pub interface_type_subprograms: Vec<GenericNodeId>,
}

/// Interface function declaration (subprogram generic).
///
/// ```vhdl
/// generic (
///   function "=" (l, r : element_t) return boolean is <>
/// );
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceFunctionDeclaration {
    /// Function interface identifier or operator symbol.
    pub identifier: Option<Identifier>,
    /// Whether the interface function is pure.
    #[serde(default)]
    pub pure_flag: bool,
    /// Parameter interface list.
    #[serde(default)]
    pub interface_declarations: Vec<InterfaceDeclarationNodeId>,
    /// Return subtype after analysis.
    pub return_type: Option<SubtypeDefinitionNodeId>,
    /// Return type mark as written.
    pub return_type_mark: Option<NameNodeId>,
    /// Default subprogram (`<>` or an explicit name).
    pub default_subprogram: Option<GenericNodeId>,
    /// Associated actual subprogram after generic map.
    pub associated_subprogram: Option<GenericNodeId>,
}

/// Interface procedure declaration (subprogram generic).
///
/// ```vhdl
/// generic (
///   procedure dump(x : element_t) is <>
/// );
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceProcedureDeclaration {
    /// Procedure interface identifier.
    pub identifier: Option<Identifier>,
    /// Parameter interface list.
    #[serde(default)]
    pub interface_declarations: Vec<InterfaceDeclarationNodeId>,
    /// Return type mark (unused for procedures; may appear in GHDL export).
    pub return_type_mark: Option<NameNodeId>,
    /// Default subprogram (`<>` or an explicit name).
    pub default_subprogram: Option<GenericNodeId>,
    /// Associated actual subprogram after generic map.
    pub associated_subprogram: Option<GenericNodeId>,
}
