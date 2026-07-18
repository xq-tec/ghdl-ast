//! Type and subtype definitions (LRM clauses 5 and 6).
//!
//! These nodes describe VHDL type marks, anonymous type definitions, subtype
//! indications after analysis, AMS natures, VHDL-2019 mode views, and resolution
//! indications. Simulation mainly needs ranges, element layouts, designated
//! types, and resolution structure.

use super::*;

subset_declaration!(SubtypeDefinition SubtypeDefinitionNodeId {
    Integer(IntegerSubtypeDefinition),
    Floating(FloatingSubtypeDefinition),
    Physical(PhysicalSubtypeDefinition),
    Enumeration(EnumerationSubtypeDefinition),
    Array(ArraySubtypeDefinition),
    Record(RecordSubtypeDefinition),
    Access(AccessSubtypeDefinition),
    File(FileSubtypeDefinition),
});

subset_declaration!(TypeDefinition TypeDefinitionNodeId {
    Array(ArrayTypeDefinition),
    Enumeration(EnumerationTypeDefinition),
    Access(AccessTypeDefinition),
    File(FileTypeDefinition),
    Record(RecordTypeDefinition),
    Incomplete(IncompleteTypeDefinition),
    Wildcard(WildcardTypeDefinition),
    /// Error node used when GHDL could not form a type definition.
    ErrorNode(Error),
});

subset_declaration!(AnonymousTypeDefinition AnonymousTypeDefinitionNodeId {
    Integer(IntegerTypeDefinition),
    Floating(FloatingTypeDefinition),
    Physical(PhysicalTypeDefinition),
    Array(ArrayTypeDefinition),
});

subset_declaration!(TypeAndSubtypeDefinition TypeAndSubtypeDefinitionNodeId {
    AccessType(AccessTypeDefinition),
    IncompleteType(IncompleteTypeDefinition),
    FileType(FileTypeDefinition),
    RecordType(RecordTypeDefinition),
    ArrayType(ArrayTypeDefinition),
    ArraySubtype(ArraySubtypeDefinition),
    RecordSubtype(RecordSubtypeDefinition),
    AccessSubtype(AccessSubtypeDefinition),
    FileSubtype(FileSubtypeDefinition),
    PhysicalSubtype(PhysicalSubtypeDefinition),
    FloatingSubtype(FloatingSubtypeDefinition),
    IntegerSubtype(IntegerSubtypeDefinition),
    EnumerationSubtype(EnumerationSubtypeDefinition),
    EnumerationType(EnumerationTypeDefinition),
    IntegerType(IntegerTypeDefinition),
    FloatingType(FloatingTypeDefinition),
    PhysicalType(PhysicalTypeDefinition),
    WildcardType(WildcardTypeDefinition),
});

subset_declaration!(PhysicalTypeOrSubtype PhysicalTypeOrSubtypeNodeId {
    Type(PhysicalTypeDefinition),
    Subtype(PhysicalSubtypeDefinition),
});

subset_declaration!(RangeConstraint RangeConstraintNodeId {
    Expression(RangeExpression),
    Attribute(Attribute),
});

/// Anonymous integer type definition (`type T is range …`).
///
/// VHDL integer types are always anonymous definitions owned by a type
/// declaration; the named object is a subtype of this anonymous base type.
///
/// ```vhdl
/// type my_int is range 0 to 255;
/// -- IntegerTypeDefinition.range_constraint = 0 to 255
/// -- The declarator actually names an IntegerSubtypeDefinition of this type.
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IntegerTypeDefinition {
    /// Inclusive discrete range that defines the type.
    ///
    /// Often absent on predefined / incomplete type nodes; the usable bounds
    /// then live on the corresponding [`IntegerSubtypeDefinition`].
    pub range_constraint: Option<RangeConstraintNodeId>,
}

/// Integer subtype indication after analysis.
///
/// Covers both the implicit first subtype of an integer type declaration and
/// explicit `subtype` / constrained subtype indications.
///
/// ```vhdl
/// type my_int is range 0 to 255;
/// subtype nibble is my_int range 0 to 15;
/// -- range_constraint = 0 to 15
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IntegerSubtypeDefinition {
    /// Constrained range of this subtype.
    pub range_constraint: RangeConstraintNodeId,
}

/// Anonymous floating type definition (`type T is range …`).
///
/// Same ownership pattern as [`IntegerTypeDefinition`]: the type declaration
/// names a floating subtype of this anonymous base type.
///
/// ```vhdl
/// type voltage is range -12.0 to 12.0;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FloatingTypeDefinition {
    /// Inclusive floating range that defines the type.
    ///
    /// Often absent on predefined type nodes; usable bounds then live on the
    /// corresponding [`FloatingSubtypeDefinition`].
    pub range_constraint: Option<RangeConstraintNodeId>,
}

/// Floating subtype indication after analysis.
///
/// ```vhdl
/// subtype small_v is voltage range -1.0 to 1.0;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FloatingSubtypeDefinition {
    /// Constrained range of this subtype.
    pub range_constraint: RangeConstraintNodeId,
}

/// Enumeration type definition (`type T is (…)`).
///
/// Literals appear in declaration order; their positions are the enumeration
/// values used by simulation and `'pos` / `'val`.
///
/// ```vhdl
/// type state_t is (IDLE, RUN, DONE);
/// type bit is ('0', '1');   -- character literals are EnumerationLiteral nodes
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumerationTypeDefinition {
    /// Enumeration literals in declaration order.
    pub enumeration_literal_list: Vec<NodeId<EnumerationLiteral>>,
}

/// Enumeration subtype indication after analysis.
///
/// May constrain the range of an enumeration type or rename it via a subtype
/// declaration. A resolution indication appears when the subtype is resolved
/// (for example a resolved `std_logic` subtype).
///
/// ```vhdl
/// type state_t is (IDLE, RUN, HALT, DONE);
/// subtype run_state is state_t range RUN to HALT;
/// -- range_constraint = RUN to HALT
/// -- parent_type      = state_t's EnumerationTypeDefinition (or its first subtype)
///
/// subtype x01 is resolved std_ulogic range 'X' to '1';
/// -- resolution_indication names the resolution function
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumerationSubtypeDefinition {
    /// Constrained enumeration range of this subtype.
    pub range_constraint: RangeConstraintNodeId,
    /// Parent enumeration type or subtype.
    pub parent_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Type mark from the subtype indication, when present in the source.
    pub subtype_type_mark: Option<NameNodeId>,
    /// Optional resolution function / record / array resolution indication.
    pub resolution_indication: Option<GenericNodeId>,
}

/// Placeholder type used by GHDL for incompletely analyzed or synthetic types.
///
/// Has no simulation-useful payload fields after GHDL-internal flags are
/// skipped. Treat as an error/placeholder if encountered in elaborated design
/// data.
#[derive(Debug, Deserialize, Serialize)]
pub struct WildcardTypeDefinition {}

/// Physical type definition (`type T is range … units … end units`).
///
/// The primary unit is the first element of [`units`](Self::units); secondary
/// units scale relative to it.
///
/// ```vhdl
/// type time is range -1e18 to 1e18
///   units
///     fs;
///     ps = 1000 fs;
///     ns = 1000 ps;
///   end units;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalTypeDefinition {
    /// Unit declarations in order (primary unit first).
    pub units: Vec<NodeId<UnitDeclaration>>,
}

/// Physical subtype indication after analysis.
///
/// ```vhdl
/// subtype delay_t is time range 0 ns to 1 us;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalSubtypeDefinition {
    /// Parent physical type or subtype.
    pub parent_type: PhysicalTypeOrSubtypeNodeId,
    /// Constrained physical range of this subtype.
    pub range_constraint: RangeConstraintNodeId,
}

/// Unconstrained or partially constrained array type definition.
///
/// [`element_subtype`](Self::element_subtype) is the analyzed element subtype.
/// [`index_subtype_list`](Self::index_subtype_list) holds the index subtypes
/// (often type marks / names for unconstrained indices).
///
/// ```vhdl
/// type word_vector is array (natural range <>) of std_logic_vector(31 downto 0);
/// type matrix is array (0 to 3, 0 to 7) of bit;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayTypeDefinition {
    /// Element subtype after analysis.
    pub element_subtype: SubtypeDefinitionNodeId,
    /// Index subtypes (one entry per dimension).
    #[serde(default)]
    pub index_subtype_list: Vec<NameNodeId>,
}

/// Array subtype indication after analysis.
///
/// Index entries are resolved subtype definitions (possibly constrained). An
/// unconstrained parent may become constrained here.
///
/// ```vhdl
/// subtype byte_vector is word_vector(0 to 3);
/// -- index_subtype_list[0] is the discrete subtype 0 to 3
///
/// subtype matrix4 is matrix;  -- may still be unconstrained in some indices
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ArraySubtypeDefinition {
    /// Element subtype after analysis (may carry element constraints).
    pub element_subtype: SubtypeDefinitionNodeId,
    /// Index subtypes after analysis (one entry per dimension).
    pub index_subtype_list: Vec<SubtypeDefinitionNodeId>,
}

/// Access type definition (`type T is access …`).
///
/// [`designated_subtype_indication`](Self::designated_subtype_indication) is
/// the subtype written after `access`. [`designated_type`](Self::designated_type)
/// is the analyzed designated type (important when the designated type was
/// incomplete at the access declaration).
///
/// ```vhdl
/// type cell;
/// type cell_ptr is access cell;   -- designated type still incomplete here
/// type cell is record
///   next_cell : cell_ptr;
///   data      : integer;
/// end record;
/// -- After completion, designated_type points at the record type.
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTypeDefinition {
    /// Designated subtype indication from the source.
    pub designated_subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed designated type (may complete an incomplete type).
    pub designated_type: Option<TypeAndSubtypeDefinitionNodeId>,
}

/// File type definition (`type T is file of …`).
///
/// [`text_file_flag`](Self::text_file_flag) is set for `std.textio.text` (and
/// equivalent text file types), which use string-oriented READ/WRITE rather
/// than typed binary I/O.
///
/// ```vhdl
/// type integer_file is file of integer;
/// -- file_type_mark = integer, text_file_flag = false
///
/// -- package std.textio:
/// type text is file of string;   -- text_file_flag = true
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FileTypeDefinition {
    /// Type mark of the file element type.
    pub file_type_mark: Option<NameNodeId>,
    /// Whether this is a text file type (`std.textio.text`).
    pub text_file_flag: bool,
}

/// Legacy / unused file-definition placeholder.
///
/// GHDL has no separate `Iir_Kind_File_Definition`; file types use
/// [`FileTypeDefinition`]. This variant exists only for enum completeness.
#[derive(Debug, Deserialize, Serialize)]
pub struct FileDefinition {}

/// Record type definition (`type T is record … end record`).
///
/// Element order in [`elements_declaration_list`](Self::elements_declaration_list)
/// is the layout order used for aggregates, selected names, and `'element`.
///
/// ```vhdl
/// type pair_t is record
///   left, right : integer;   -- two ElementDeclaration nodes (identifier list)
/// end record;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordTypeDefinition {
    /// Record elements in declaration order.
    #[serde(default)]
    pub elements_declaration_list: Vec<NodeId<ElementDeclaration>>,
}

/// Record subtype indication after analysis.
///
/// May add per-element constraints via
/// [`owned_elements`](Self::owned_elements) and/or a resolution indication for
/// resolved record subtypes (VHDL-2008).
///
/// ```vhdl
/// type rec_t is record
///   a : integer;
///   b : std_logic_vector;
/// end record;
/// subtype rec_s is rec_t (b(7 downto 0));
/// -- owned_elements contains a RecordElementConstraint for b
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordSubtypeDefinition {
    /// Element declarations (often shared with / copied from the parent type).
    #[serde(default)]
    pub elements_declaration_list: Vec<NodeId<ElementDeclaration>>,
    /// Parent record type or subtype.
    pub parent_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Type mark from the subtype indication, when present.
    pub subtype_type_mark: Option<NameNodeId>,
    /// Optional resolution indication for a resolved record subtype.
    pub resolution_indication: Option<GenericNodeId>,
    /// Element constraints owned by this subtype (chain in GHDL).
    #[serde(default)]
    pub owned_elements: Vec<NodeId<RecordElementConstraint>>,
}

/// Access subtype indication after analysis.
///
/// ```vhdl
/// type ptr is access string;
/// subtype short_ptr is ptr;   -- may refine designated subtype in some cases
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessSubtypeDefinition {
    /// Designated subtype indication of this access subtype.
    pub designated_subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed designated type.
    pub designated_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Parent access type or subtype.
    pub parent_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Type mark from the subtype indication, when present.
    pub subtype_type_mark: Option<NameNodeId>,
}

/// Incomplete type definition (`type T;`).
///
/// Used for mutually recursive access/record types. After the full type is
/// declared, [`complete_type_definition`](Self::complete_type_definition) points
/// at the completed definition and
/// [`incomplete_type_refs`](Self::incomplete_type_refs) lists names that referred
/// to the incomplete type.
///
/// ```vhdl
/// type cell;                      -- IncompleteTypeDefinition
/// type cell_ptr is access cell;
/// type cell is record             -- completes the incomplete type
///   link : cell_ptr;
/// end record;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IncompleteTypeDefinition {
    /// Names / references that pointed at this incomplete type.
    #[serde(default)]
    pub incomplete_type_refs: Vec<GenericNodeId>,
    /// Completed type definition once the full type is analyzed.
    pub complete_type_definition: Option<TypeAndSubtypeDefinitionNodeId>,
}

/// File subtype indication after analysis.
///
/// ```vhdl
/// subtype int_file is integer_file;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FileSubtypeDefinition {
    /// Parent file type or subtype.
    pub parent_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Type mark from the subtype indication, when present.
    pub subtype_type_mark: Option<NameNodeId>,
    /// Whether this subtype is a text file type.
    pub text_file_flag: bool,
}

/// Per-element constraint inside a record subtype indication.
///
/// ```vhdl
/// subtype rec_s is rec_t (b(7 downto 0));
/// -- identifier = b, subtype_indication = std_logic_vector(7 downto 0)
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordElementConstraint {
    /// Element identifier being constrained.
    pub identifier: Identifier,
    /// Zero-based element position in the parent record type.
    pub element_position: Option<i32>,
    /// Constrained subtype indication for this element.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed type of the constrained element.
    #[serde(rename = "type")]
    pub typ: Option<TypeAndSubtypeDefinitionNodeId>,
}

/// Interface type definition from a package generic (`generic (type T)`).
///
/// [`associated_type`](Self::associated_type) is filled when the generic is
/// mapped to an actual type.
///
/// ```vhdl
/// generic (
///   type element_t
/// );
/// -- InterfaceTypeDefinition.associated_type set by generic map
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct InterfaceTypeDefinition {
    /// Actual type associated by a generic map, when elaborated.
    pub associated_type: Option<TypeAndSubtypeDefinitionNodeId>,
}

/// Protected type declaration (`type T is protected … end protected`).
///
/// Contains method declarations (and shared variable-like state declarations).
/// The matching [`ProtectedTypeBody`] holds the method implementations.
///
/// ```vhdl
/// type shared_counter is protected
///   procedure increment;
///   impure function value return natural;
/// end protected;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtectedTypeDeclaration {
    /// Declarations in the protected type (methods and items).
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Corresponding protected type body, when analyzed.
    pub protected_type_body: Option<NodeId<ProtectedTypeBody>>,
}

/// Protected type body (`type T is protected body … end protected body`).
///
/// ```vhdl
/// type shared_counter is protected body
///   variable count : natural := 0;
///   procedure increment is begin count := count + 1; end;
///   impure function value return natural is begin return count; end;
/// end protected body;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ProtectedTypeBody {
    /// Identifier of the protected type being completed.
    pub identifier: Option<Identifier>,
    /// Declarations and subprogram bodies inside the protected body.
    #[serde(default)]
    pub declarations: Vec<DeclarationNodeId>,
    /// Matching protected type declaration.
    pub protected_type_declaration: Option<NodeId<ProtectedTypeDeclaration>>,
}

/// Foreign vector type used by GHDL's foreign/VPI interfaces.
///
/// Carries no simulation-useful fields once GHDL-internal declarator links are
/// skipped. Appears only in foreign-module contexts.
#[derive(Debug, Deserialize, Serialize)]
pub struct ForeignVectorTypeDefinition {}

/// Mode-view indication applied to a record object (VHDL-2019).
///
/// ```vhdl
/// port (
///   bus_if : view master_view of bus_t
/// );
/// -- name = master_view, subtype_indication / type describe bus_t
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordModeViewIndication {
    /// Name of the mode view.
    pub name: Option<NameNodeId>,
    /// Subtype indication of the viewed type.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed type of the viewed object.
    #[serde(rename = "type")]
    pub typ: Option<TypeAndSubtypeDefinitionNodeId>,
}

/// Mode-view indication applied to an array object (VHDL-2019).
///
/// ```vhdl
/// port (
///   lanes : view lane_view of lane_vector
/// );
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayModeViewIndication {
    /// Name of the mode view.
    pub name: Option<NameNodeId>,
    /// Subtype indication of the viewed type.
    pub subtype_indication: Option<SubtypeDefinitionNodeId>,
    /// Analyzed type of the viewed object.
    #[serde(rename = "type")]
    pub typ: Option<TypeAndSubtypeDefinitionNodeId>,
}

/// Scalar AMS nature definition (`nature T is … across … through …`).
///
/// Across/through types are the floating (or other) types used for branch
/// quantities of this nature.
///
/// ```vhdl
/// nature electrical is
///   voltage across
///   current through
///   ground reference;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ScalarNatureDefinition {
    /// Type mark of the across quantity type.
    pub across_type_mark: Option<NameNodeId>,
    /// Type mark of the through quantity type.
    pub through_type_mark: Option<NameNodeId>,
    /// Analyzed across type.
    pub across_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Analyzed through type.
    pub through_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Reference terminal name (`… reference`).
    pub reference: Option<NameNodeId>,
    /// Base nature (self for a root scalar nature).
    pub base_nature: Option<GenericNodeId>,
}

/// Record AMS nature definition.
///
/// ```vhdl
/// nature electrical_vector is record
///   v : electrical;
///   i : electrical;
/// end record;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordNatureDefinition {
    /// Nature elements in declaration order.
    #[serde(default)]
    pub elements_declaration_list: Vec<NodeId<NatureElementDeclaration>>,
    /// Composite across type synthesized for this nature.
    pub across_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Composite through type synthesized for this nature.
    pub through_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Base nature.
    pub base_nature: Option<GenericNodeId>,
    /// Underlying simple nature, when applicable.
    pub simple_nature: Option<GenericNodeId>,
}

/// Array AMS nature definition.
///
/// ```vhdl
/// nature electrical_bus is array (0 to 7) of electrical;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayNatureDefinition {
    /// Element subnature after analysis.
    pub element_subnature: Option<GenericNodeId>,
    /// Index subtypes (one entry per dimension).
    #[serde(default)]
    pub index_subtype_list: Vec<NameNodeId>,
    /// Composite across type synthesized for this nature.
    pub across_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Composite through type synthesized for this nature.
    pub through_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Base nature.
    pub base_nature: Option<GenericNodeId>,
}

/// Array AMS subnature indication after analysis.
///
/// ```vhdl
/// subnature nibble_bus is electrical_bus(0 to 3);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ArraySubnatureDefinition {
    /// Nature mark of the parent nature.
    pub subnature_nature_mark: Option<NameNodeId>,
    /// Element subnature after analysis.
    pub element_subnature: Option<GenericNodeId>,
    /// Index subtypes after analysis.
    #[serde(default)]
    pub index_subtype_list: Vec<SubtypeDefinitionNodeId>,
    /// Composite across type.
    pub across_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Composite through type.
    pub through_type: Option<TypeAndSubtypeDefinitionNodeId>,
    /// Base nature.
    pub base_nature: Option<GenericNodeId>,
}

/// Record resolution indication (`(elem1 resolution1, …)`).
///
/// Used in VHDL-2008 resolved composite subtypes.
///
/// ```vhdl
/// subtype resolved_rec is (a resolved, b resolved) rec_t;
/// -- record_element_resolutions lists per-element resolutions
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordResolution {
    /// Per-element resolution indications (GHDL chain).
    #[serde(default, rename = "record_element_resolutions")]
    pub record_element_resolutions: Vec<NodeId<RecordElementResolution>>,
}

/// One element of a [`RecordResolution`].
///
/// ```vhdl
/// subtype r is (data resolved_data) packet_t;
/// -- identifier = data, resolution_indication = resolved_data
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordElementResolution {
    /// Record element being resolved.
    pub identifier: Option<Identifier>,
    /// Resolution function or nested resolution indication for this element.
    pub resolution_indication: Option<GenericNodeId>,
}

/// Simple mode-view element (`a : in`, `b : out`, …).
///
/// ```vhdl
/// view master_view of bus_t is
///   req  : out;
///   ack  : in;
///   data : out;
/// end view;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleModeViewElement {
    /// Element identifier.
    pub identifier: Option<Identifier>,
    /// Mode contributed by this view element.
    pub mode: Option<Mode>,
    /// Named entity (record element) this view applies to.
    pub named_entity: Option<GenericNodeId>,
}

/// Array mode-view element that nests another mode view.
///
/// ```vhdl
/// view bus_array_view of bus_vector is
///   others : view master_view;
/// end view;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ArrayModeViewElement {
    /// Element identifier (`others` or an index name).
    pub identifier: Option<Identifier>,
    /// Named entity this element refers to.
    pub named_entity: Option<GenericNodeId>,
    /// Nested mode-view name.
    pub mode_view_name: Option<NameNodeId>,
}

/// Record mode-view element that nests another mode view.
///
/// ```vhdl
/// view top_view of top_t is
///   child : view child_view;
/// end view;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RecordModeViewElement {
    /// Element identifier.
    pub identifier: Option<Identifier>,
    /// Named entity (record element) this view applies to.
    pub named_entity: Option<GenericNodeId>,
    /// Nested mode-view name.
    pub mode_view_name: Option<NameNodeId>,
}
