//! Expressions and literals (LRM clause 9).
//!
//! Expression nodes cover operators, calls, aggregates, allocators, qualified
//! expressions, type conversions, and the various literal forms. Names that may
//! appear in expression contexts (indexed, selected, attributes, …) are also
//! members of [`Expression`].

use std::borrow::Cow;
use std::fmt;
use std::ops;
use std::ptr;

use smallvec::SmallVec;

use super::*;

subset_declaration!(Expression ExpressionNodeId {
    CharacterLiteral(CharacterLiteral),
    IntegerLiteral(IntegerLiteral),
    FloatingPointLiteral(FloatingPointLiteral),
    PhysicalIntLiteral(PhysicalIntLiteral),
    PhysicalFpLiteral(PhysicalFpLiteral),
    NullLiteral(NullLiteral),
    OverflowLiteral(OverflowLiteral),
    StringLiteral(StringLiteral),
    SimpleAggregate(SimpleAggregate),

    Unary(UnaryOperator),
    Binary(BinaryOperator),
    FunctionCall(FunctionCall),
    ParenthesisExpression(ParenthesisExpression),
    QualifiedExpression(QualifiedExpression),
    TypeConversion(TypeConversion),

    Aggregate(Aggregate),
    AllocatorByExpression(AllocatorByExpression),
    AllocatorBySubtype(AllocatorBySubtype),

    IndexedName(IndexedName),
    SimpleName(SimpleName),
    SliceName(SliceName),
    SelectedName(SelectedName),
    SelectedElement(SelectedElement),
    AttributeName(AttributeName),
    Attribute(Attribute),
    Dereference(Dereference),
    ImplicitDereference(ImplicitDereference),
});

subset_declaration!(Literal LiteralNodeId {
    Integer(IntegerLiteral),
    FloatingPoint(FloatingPointLiteral),
    Null(NullLiteral),
    String(StringLiteral),
    PhysicalInt(PhysicalIntLiteral),
    PhysicalFp(PhysicalFpLiteral),
});

subset_declaration!(Allocator AllocatorNodeId {
    ByExpression(AllocatorByExpression),
    BySubtype(AllocatorBySubtype),
});

/// A unary operator application.
///
/// ```vhdl
/// -a;  not en;  abs x;  ?? cond;  and vec;   -- reduction (VHDL-2008)
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct UnaryOperator {
    /// Operator kind (identity, negation, `abs`, `not`, condition, reductions).
    pub kind: UnaryOperatorKind,
    /// Operand expression.
    pub operand: ExpressionNodeId,
    /// Resolved operator function implementation.
    pub implementation: FunctionImplementationNodeId,
}

/// Kind of a unary operator.
#[derive(Debug, Deserialize, Serialize)]
pub enum UnaryOperatorKind {
    /// Unary `+` (identity).
    #[serde(rename = "+")]
    Identity,
    /// Unary `-` (negation).
    #[serde(rename = "-")]
    Negation,
    /// Absolute value (`abs`).
    #[serde(rename = "abs")]
    Absolute,
    /// Logical / bitwise not (`not`).
    #[serde(rename = "not")]
    Not,
    /// Condition operator (`??`, VHDL-2008).
    #[serde(rename = "??")]
    Condition,
    /// Reduction `and` (VHDL-2008).
    #[serde(rename = "and")]
    ReductionAnd,
    /// Reduction `or` (VHDL-2008).
    #[serde(rename = "or")]
    ReductionOr,
    /// Reduction `nand` (VHDL-2008).
    #[serde(rename = "nand")]
    ReductionNand,
    /// Reduction `nor` (VHDL-2008).
    #[serde(rename = "nor")]
    ReductionNor,
    /// Reduction `xor` (VHDL-2008).
    #[serde(rename = "xor")]
    ReductionXor,
    /// Reduction `xnor` (VHDL-2008).
    #[serde(rename = "xnor")]
    ReductionXnor,
}

/// A binary operator application.
///
/// ```vhdl
/// a and b;  x + y;  srl 2;  left & right;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct BinaryOperator {
    /// Operator kind (logical, relational, shifting, arithmetic, …).
    pub kind: BinaryOperatorKind,
    /// Left operand.
    pub left: ExpressionNodeId,
    /// Right operand.
    pub right: ExpressionNodeId,
    /// Resolved operator function implementation.
    pub implementation: FunctionImplementationNodeId,
}

/// Kind of a binary operator.
#[derive(Debug, Deserialize, Serialize)]
pub enum BinaryOperatorKind {
    /// Logical / bitwise `and`.
    #[serde(rename = "and")]
    And,
    /// Logical / bitwise `or`.
    #[serde(rename = "or")]
    Or,
    /// Logical / bitwise `nand`.
    #[serde(rename = "nand")]
    Nand,
    /// Logical / bitwise `nor`.
    #[serde(rename = "nor")]
    Nor,
    /// Logical / bitwise `xor`.
    #[serde(rename = "xor")]
    Xor,
    /// Logical / bitwise `xnor`.
    #[serde(rename = "xnor")]
    Xnor,
    /// Equality (`=`).
    #[serde(rename = "=")]
    Equality,
    /// Inequality (`/=`).
    #[serde(rename = "/=")]
    Inequality,
    /// Less than (`<`).
    #[serde(rename = "<")]
    LessThan,
    /// Less than or equal (`<=`).
    #[serde(rename = "<=")]
    LessThanOrEqual,
    /// Greater than (`>`).
    #[serde(rename = ">")]
    GreaterThan,
    /// Greater than or equal (`>=`).
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    /// Matching equality (`?=`, VHDL-2008).
    #[serde(rename = "?=")]
    MatchEquality,
    /// Matching inequality (`?/=`, VHDL-2008).
    #[serde(rename = "?/=")]
    MatchInequality,
    /// Matching less than (`?<`, VHDL-2008).
    #[serde(rename = "?<")]
    MatchLessThan,
    /// Matching less than or equal (`?<=`, VHDL-2008).
    #[serde(rename = "?<=")]
    MatchLessThanOrEqual,
    /// Matching greater than (`?>`, VHDL-2008).
    #[serde(rename = "?>")]
    MatchGreaterThan,
    /// Matching greater than or equal (`?>=`, VHDL-2008).
    #[serde(rename = "?>=")]
    MatchGreaterThanOrEqual,
    /// Shift left logical (`sll`).
    #[serde(rename = "sll")]
    Sll,
    /// Shift left arithmetic (`sla`).
    #[serde(rename = "sla")]
    Sla,
    /// Shift right logical (`srl`).
    #[serde(rename = "srl")]
    Srl,
    /// Shift right arithmetic (`sra`).
    #[serde(rename = "sra")]
    Sra,
    /// Rotate left (`rol`).
    #[serde(rename = "rol")]
    Rol,
    /// Rotate right (`ror`).
    #[serde(rename = "ror")]
    Ror,
    /// Addition (`+`).
    #[serde(rename = "+")]
    Addition,
    /// Subtraction (`-`).
    #[serde(rename = "-")]
    Substraction,
    /// Concatenation (`&`).
    #[serde(rename = "&")]
    Concatenation,
    /// Multiplication (`*`).
    #[serde(rename = "*")]
    Multiplication,
    /// Division (`/`).
    #[serde(rename = "/")]
    Division,
    /// Modulus (`mod`).
    #[serde(rename = "mod")]
    Modulus,
    /// Remainder (`rem`).
    #[serde(rename = "rem")]
    Remainder,
    /// Exponentiation (`**`).
    #[serde(rename = "**")]
    Exponentiation,
}

/// A function call expression.
///
/// ```vhdl
/// rising_edge(clk);
/// ieee.numeric_std.to_integer(unsigned(a));
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionCall {
    /// Prefix naming the called function (possibly a selected name).
    pub prefix: PrefixNodeId,
    /// Resolved function declaration or interface function.
    pub implementation: FunctionImplementationNodeId,
    /// Parameter associations of the call.
    #[serde(default)]
    pub parameter_associations: Vec<AssociationElementNodeId>,
    /// Return type of the call.
    #[serde(rename = "type")]
    pub return_type: SubtypeDefinitionNodeId,
}

/// An integer literal.
///
/// ```vhdl
/// 42;  1_024;  16#FF#;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct IntegerLiteral {
    /// Integer value after analysis.
    pub value: i64,
}

/// A floating-point literal.
///
/// ```vhdl
/// 3.14;  1.0e-3;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FloatingPointLiteral {
    /// Floating-point value (JSON encodes raw bits as `#` + hex).
    #[serde(rename = "fp_value", deserialize_with = "deserialize_f64")]
    pub value: f64,
}

/// A physical literal with an integer abstract literal.
///
/// ```vhdl
/// 10 ns;  1 sec;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalIntLiteral {
    /// Integer abstract literal.
    pub value: i64,
    /// Unit name (`ns`, `sec`, …).
    pub unit_name: NameNodeId,
}

/// A physical literal with a floating-point abstract literal.
///
/// ```vhdl
/// 0.5 ns;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalFpLiteral {
    /// Floating-point abstract literal.
    #[serde(rename = "fp_value", deserialize_with = "deserialize_f64")]
    pub value: f64,
    /// Unit name (`ns`, `sec`, …).
    pub unit_name: NameNodeId,
}

subset_declaration!(PhysicalLiteral PhysicalLiteralNodeId {
    PhysicalInt(PhysicalIntLiteral),
    PhysicalFp(PhysicalFpLiteral),
});

/// Wrapper for a literal known to overflow its target type.
///
/// GHDL keeps the original literal under `literal_origin` so diagnostics and
/// tools can still inspect the written value.
#[derive(Debug, Deserialize, Serialize)]
pub struct OverflowLiteral {
    /// Original literal expression that overflowed.
    pub literal_origin: ExpressionNodeId,
}

/// A range expression (`A to B` / `A downto B`).
///
/// ```vhdl
/// 0 to 7;  7 downto 0;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct RangeExpression {
    /// Range direction (`to` or `downto`).
    pub direction: Direction,
    /// Left bound of the range.
    pub left_limit: ExpressionNodeId,
    /// Right bound of the range.
    pub right_limit: ExpressionNodeId,
}

/// An aggregate expression.
///
/// Choices and associated expressions are the simulation-relevant content;
/// optional GHDL [`AggregateInfo`] analysis metadata may hang off the node in
/// the export but is not required to evaluate the aggregate.
///
/// ```vhdl
/// (0 => '0', others => '1');
/// (addr => x"00", data => x"FF");
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct Aggregate {
    /// Association choices of the aggregate.
    #[serde(rename = "association_choices")]
    pub associations: Vec<ChoiceNodeId>,
    /// Type of the aggregate after analysis, when determined.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// A string literal (including bit-string literals after analysis).
///
/// ```vhdl
/// "hello";  x"FF";  b"1010";
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct StringLiteral {
    /// Latin-1 contents of the string.
    #[serde(rename = "string8_id")]
    pub value: Latin1String,
    /// Optional origin expression when this literal was derived (e.g. expanded).
    pub literal_origin: Option<ExpressionNodeId>,
}

/// An enumeration literal (including character enumeration values of a type).
///
/// ```vhdl
/// true;  red;  '1';   -- '1' may also appear as CharacterLiteral in name contexts
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct EnumerationLiteral {
    /// Position number of the literal in its enumeration type.
    pub enum_pos: u32,
    /// Identifier of the enumeration literal (may be a character literal name).
    pub identifier: Identifier,
}

/// Owned ISO-8859-1 (Latin-1) string as exported by GHDL string literals.
///
/// JSON stores the string as UTF-8; characters above U+007F are decoded back to
/// single Latin-1 bytes on deserialize.
#[derive(Deserialize)]
#[serde(try_from = "CompactString")]
pub struct Latin1String {
    characters: SmallVec<[u8; 24]>,
}

impl Serialize for Latin1String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let utf8 = if let Ok(string) = str::from_utf8(&self.characters) {
            Cow::Borrowed(string)
        } else {
            Cow::Owned(
                self.characters
                    .iter()
                    .map(|&byte| char::from(byte))
                    .collect(),
            )
        };
        serializer.serialize_str(&utf8)
    }
}

impl Latin1String {
    /// Returns this string as a borrowed Latin-1 view.
    #[must_use]
    pub fn as_str(&self) -> &Latin1Str {
        self
    }
}

impl ops::Deref for Latin1String {
    type Target = Latin1Str;

    fn deref(&self) -> &Self::Target {
        Latin1Str::new(self.characters.as_slice())
    }
}

impl fmt::Display for Latin1String {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(formatter)
    }
}

impl fmt::Debug for Latin1String {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(formatter)
    }
}

impl TryFrom<CompactString> for Latin1String {
    type Error = LatinStringError;

    #[expect(clippy::indexing_slicing, reason = "// TODO explain why this is safe")]
    fn try_from(value: CompactString) -> Result<Self, Self::Error> {
        let mut bytes = value.into_bytes();
        if let Some(first) = bytes.iter().position(|&byte| byte >= 0x80) {
            let mut read_pos = first;
            let mut write_pos = first;
            while read_pos < bytes.len() {
                let byte_1 = bytes[read_pos];
                let latin_1_char = if byte_1 >= 0x80 {
                    if byte_1 > 0b_1100_0011 {
                        // Either a 3- or 4-byte character (always too large for latin-1),
                        // or a 2-byte character > 0xff (not a valid latin-1 character)
                        return Err(LatinStringError {
                            position: write_pos,
                        });
                    }
                    let byte_2 = bytes[read_pos + 1];
                    read_pos += 2;
                    (byte_1 << 6) + (byte_2 & 0b_0011_1111)
                } else {
                    read_pos += 1;
                    byte_1
                };
                bytes[write_pos] = latin_1_char;
                write_pos += 1;
            }
            bytes.truncate(write_pos);
        }
        Ok(Self { characters: bytes })
    }
}

/// Error returned when a string contains a non–ISO-8859-1 character.
#[derive(Clone, Copy, Debug)]
pub struct LatinStringError {
    /// Byte index (0-based) of the first invalid character.
    pub position: usize,
}

impl fmt::Display for LatinStringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "character at position {} is not an ISO-8859-1 character",
            self.position + 1,
        )
    }
}

/// Borrowed ISO-8859-1 string slice.
#[repr(transparent)]
pub struct Latin1Str([u8]);

impl Latin1Str {
    /// Creates a Latin-1 string view from raw bytes.
    pub fn new<S: AsRef<[u8]> + ?Sized>(string: &S) -> &Self {
        let bytes = string.as_ref();
        let latin1_str = ptr::from_ref(bytes) as *const Latin1Str;
        // SAFETY: Self is a repr(transparent) of `[u8]`, therefore `&Self` has
        // the same memory layout as `&[u8]`.
        #[expect(unsafe_code, reason = "must use unsafe to do this")]
        unsafe {
            &*latin1_str
        }
    }

    /// Returns the underlying Latin-1 bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the length in bytes (equal to the number of Latin-1 characters).
    #[must_use]
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

    /// Returns whether the string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.as_bytes().is_empty()
    }
}

impl fmt::Display for Latin1Str {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &byte in self.as_bytes() {
            write!(formatter, "{}", byte as char)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Latin1Str {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &byte in self.as_bytes() {
            write!(formatter, "{}", (byte as char).escape_debug())?;
        }
        Ok(())
    }
}

/// A character literal used as a name or expression (`'X'`).
///
/// The identifier holds the character name (including the quotes in GHDL's
/// identifier encoding). The named entity usually resolves to the corresponding
/// enumeration literal of the character / enumeration type.
///
/// ```vhdl
/// '1';  'Z';  'X';
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterLiteral {
    /// Character name as written (e.g. `'1'`).
    pub identifier: Identifier,
    /// Resolved enumeration literal or other named entity.
    pub named_entity: NamedEntityNodeId,
    /// Type of the character literal.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
}

/// A qualified expression (`type_mark'(expression)`).
///
/// Qualification states the type (or subtype) of an expression without
/// converting its value. Contrast with [`TypeConversion`], which changes type.
///
/// ```vhdl
/// integer'(1 + 2)          -- qualified: type is integer, value unchanged
/// std_logic_vector'("01")  -- qualifies a bit-string / aggregate
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct QualifiedExpression {
    /// Type mark naming the type or subtype.
    pub type_mark: NameNodeId,
    /// Qualified operand expression.
    pub expression: ExpressionNodeId,
    /// Type of the qualified expression after analysis.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
}

/// The null access-value literal (`null`).
///
/// ```vhdl
/// ptr := null;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct NullLiteral {
    /// Access type of the null literal.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
}

/// An allocator that initializes from a qualified expression (`new type_mark'(…)`).
///
/// ```vhdl
/// ptr := new string'("hello");
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AllocatorByExpression {
    /// Initial-value expression (typically a qualified expression).
    pub expression: ExpressionNodeId,
    /// Access type of the allocator result.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
    /// Designated (element) type of the allocated object.
    pub allocator_designated_type: SubtypeDefinitionNodeId,
}

/// An allocator that creates an uninitialized object of a given subtype (`new subtype`).
///
/// ```vhdl
/// ptr := new string(1 to 10);
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AllocatorBySubtype {
    /// Subtype indication of the allocated object.
    pub subtype_indication: SubtypeDefinitionNodeId,
    /// Analyzed allocator subtype (often the same as `subtype_indication`).
    pub allocator_subtype: SubtypeDefinitionNodeId,
    /// Access type of the allocator result.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
    /// Designated (element) type of the allocated object.
    pub allocator_designated_type: SubtypeDefinitionNodeId,
}

/// GHDL aggregate analysis helper attached to an [`Aggregate`].
///
/// Records bounds and choice-shape flags computed during analysis. Simulation
/// should evaluate the aggregate from [`Aggregate::associations`]; these fields
/// are optional analysis aids and may be incomplete depending on the export.
#[derive(Debug, Deserialize, Serialize)]
pub struct AggregateInfo {
    /// Minimum length implied by positional / named choices.
    #[serde(default)]
    pub aggr_min_length: i32,
    /// Whether an `others` choice is present.
    #[serde(default)]
    pub aggr_others_flag: bool,
    /// Whether any choice bound is non-static (dynamic).
    #[serde(default)]
    pub aggr_dynamic_flag: bool,
    /// Whether any named (choice => value) associations are present.
    #[serde(default)]
    pub aggr_named_flag: bool,
}

/// A parenthesized expression that preserves source parentheses in the AST.
///
/// ```vhdl
/// (a + b) * c;
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct ParenthesisExpression {
    /// Expression inside the parentheses.
    pub expression: ExpressionNodeId,
    /// Type of the parenthesized expression.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
}

/// A type conversion (`type_mark(expression)`).
///
/// Converts a value from one closely related type to another. This is distinct
/// from a [`QualifiedExpression`], which only asserts a type without converting.
///
/// ```vhdl
/// integer(1.5)                 -- type conversion: real → integer
/// integer'(1 + 2)              -- qualified expression (not a conversion)
/// std_logic_vector(unsigned_a) -- conversion between closely related array types
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct TypeConversion {
    /// Target type mark of the conversion.
    pub type_mark: NameNodeId,
    /// Operand expression being converted.
    pub expression: ExpressionNodeId,
    /// Result type of the conversion.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
    /// Optional subtype produced for the conversion result.
    pub type_conversion_subtype: Option<SubtypeDefinitionNodeId>,
}

/// A simple aggregate used by GHDL for expanded string / bit-string literals.
///
/// The list holds element literals (typically [`EnumerationLiteral`] nodes for
/// character values). Prefer [`StringLiteral`] / [`Aggregate`] for source-level
/// forms; this node appears when GHDL expands a literal into element-wise form.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleAggregate {
    /// Element literals of the expanded aggregate.
    #[serde(default)]
    pub simple_aggregate_list: Vec<NodeId<EnumerationLiteral>>,
    /// Type of the simple aggregate.
    #[serde(rename = "type")]
    pub typ: SubtypeDefinitionNodeId,
    /// Optional origin expression (e.g. the original string literal).
    pub literal_origin: Option<ExpressionNodeId>,
    /// Optional literal subtype computed during analysis.
    pub literal_subtype: Option<SubtypeDefinitionNodeId>,
}

#[cfg(test)]
mod tests {
    #![expect(clippy::unwrap_used, reason = "ok for tests")]
    use super::*;

    fn check_ok(str: &str, expected: &[u8]) {
        let latin_string = Latin1String::try_from(CompactString::new(str)).unwrap();
        assert_eq!(latin_string.characters.as_slice(), expected);
    }

    fn check_err(str: &str, expected_pos: usize) {
        let latin_string = Latin1String::try_from(CompactString::new(str));
        assert_eq!(latin_string.err().unwrap().position, expected_pos);
    }

    #[test]
    fn test_latin_string_ok() {
        check_ok("", &[]);

        check_ok("\u{0000}", &[0x00]);
        check_ok("\u{0001}", &[0x01]);
        check_ok("\u{007f}", &[0x7f]);
        check_ok("\u{0080}", &[0x80]);
        check_ok("\u{0081}", &[0x81]);
        check_ok("\u{00ff}", &[0xff]);

        check_ok("abc\u{0000}", &[b'a', b'b', b'c', 0x00]);
        check_ok("abc\u{0001}", &[b'a', b'b', b'c', 0x01]);
        check_ok("abc\u{007f}", &[b'a', b'b', b'c', 0x7f]);
        check_ok("abc\u{0080}", &[b'a', b'b', b'c', 0x80]);
        check_ok("abc\u{0081}", &[b'a', b'b', b'c', 0x81]);
        check_ok("abc\u{00ff}", &[b'a', b'b', b'c', 0xff]);

        check_ok("\u{0000}xyz", &[0x00, b'x', b'y', b'z']);
        check_ok("\u{0001}xyz", &[0x01, b'x', b'y', b'z']);
        check_ok("\u{007f}xyz", &[0x7f, b'x', b'y', b'z']);
        check_ok("\u{0080}xyz", &[0x80, b'x', b'y', b'z']);
        check_ok("\u{0081}xyz", &[0x81, b'x', b'y', b'z']);
        check_ok("\u{00ff}xyz", &[0xff, b'x', b'y', b'z']);

        check_ok(
            "abc\u{007f}\u{0080}\u{0081}xyz\u{00c0}\u{00ff} ",
            &[
                b'a', b'b', b'c', 0x7f, 0x80, 0x81, b'x', b'y', b'z', 0xc0, 0xff, b' ',
            ],
        );
    }

    #[test]
    fn test_latin_string_err() {
        // 2-byte characters
        check_err("\u{0100}", 0);
        check_err("\u{0101}", 0);
        check_err("\u{07ff}", 0);
        // 3-byte characters
        check_err("\u{0800}", 0);
        check_err("\u{ffff}", 0);
        // 4-byte characters
        check_err("\u{10000}", 0);
        check_err("\u{10fff}", 0);

        check_err("\u{0100}xyz", 0);
        check_err("\u{0101}xyz", 0);
        check_err("\u{07ff}xyz", 0);
        check_err("\u{0800}xyz", 0);
        check_err("\u{ffff}xyz", 0);
        check_err("\u{10000}xyz", 0);
        check_err("\u{10fff}xyz", 0);

        check_err("abc\u{0100}", 3);
        check_err("abc\u{0101}", 3);
        check_err("abc\u{07ff}", 3);
        check_err("abc\u{0800}", 3);
        check_err("abc\u{ffff}", 3);
        check_err("abc\u{10000}", 3);
        check_err("abc\u{10fff}", 3);

        check_err("abc\u{0100}\u{0101}xyz\u{10fff}", 3);
    }
}
