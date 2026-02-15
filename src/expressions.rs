use std::fmt;
use std::ops;
use std::ptr;

use smallvec::SmallVec;

use super::*;

subset_declaration!(Expression ExpressionNodeId {
    CharacterLiteral(CharacterLiteral),
    IntegerLiteral(IntegerLiteral),
    PhysicalIntLiteral(PhysicalIntLiteral),
    OverflowLiteral(OverflowLiteral),
    StringLiteral(StringLiteral),

    Unary(UnaryOperator),
    Binary(BinaryOperator),
    FunctionCall(SubprogramCall),

    Aggregate(Aggregate),
    IndexedName(IndexedName),
    SimpleName(SimpleName),
    SliceName(SliceName),
});

#[derive(Debug, Deserialize)]
pub struct UnaryOperator {
    pub kind: UnaryOperatorKind,
    pub operand: ExpressionNodeId,
    pub implementation: NodeId<SubprogramDeclaration>,
}

#[derive(Debug, Deserialize)]
pub enum UnaryOperatorKind {
    #[serde(rename = "+")]
    Identity,
    #[serde(rename = "-")]
    Negation,
    #[serde(rename = "abs")]
    Absolute,
    #[serde(rename = "not")]
    Not,
    #[serde(rename = "??")]
    Condition,
    #[serde(rename = "and")]
    ReductionAnd,
    #[serde(rename = "or")]
    ReductionOr,
    #[serde(rename = "nand")]
    ReductionNand,
    #[serde(rename = "nor")]
    ReductionNor,
    #[serde(rename = "xor")]
    ReductionXor,
    #[serde(rename = "xnor")]
    ReductionXnor,
}

#[derive(Debug, Deserialize)]
pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub left: ExpressionNodeId,
    pub right: ExpressionNodeId,
    pub implementation: NodeId<SubprogramDeclaration>,
}

#[derive(Debug, Deserialize)]
pub enum BinaryOperatorKind {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "nand")]
    Nand,
    #[serde(rename = "nor")]
    Nor,
    #[serde(rename = "xor")]
    Xor,
    #[serde(rename = "xnor")]
    Xnor,
    #[serde(rename = "=")]
    Equality,
    #[serde(rename = "/=")]
    Inequality,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "<=")]
    LessThanOrEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    #[serde(rename = "?=")]
    MatchEquality,
    #[serde(rename = "?/=")]
    MatchInequality,
    #[serde(rename = "?<")]
    MatchLessThan,
    #[serde(rename = "?<=")]
    MatchLessThanOrEqual,
    #[serde(rename = "?>")]
    MatchGreaterThan,
    #[serde(rename = "?>=")]
    MatchGreaterThanOrEqual,
    #[serde(rename = "sll")]
    Sll,
    #[serde(rename = "sla")]
    Sla,
    #[serde(rename = "srl")]
    Srl,
    #[serde(rename = "sra")]
    Sra,
    #[serde(rename = "rol")]
    Rol,
    #[serde(rename = "ror")]
    Ror,
    #[serde(rename = "+")]
    Addition,
    #[serde(rename = "-")]
    Substraction,
    #[serde(rename = "&")]
    Concatenation,
    #[serde(rename = "*")]
    Multiplication,
    #[serde(rename = "/")]
    Division,
    #[serde(rename = "mod")]
    Modulus,
    #[serde(rename = "rem")]
    Remainder,
    #[serde(rename = "**")]
    Exponentiation,
}

/// ```text
/// procedure_call:
///
/// implementation: &procedure_declaration
/// parameter_associations: &[association_element_by_expression] | &[association_element_open] | &[association_element_by_individual]
/// prefix: &simple_name | &selected_name
/// ```
///
/// ```text
/// function_call:
///
/// prefix: &selected_name | &simple_name | &operator_symbol
/// type: &floating_subtype_definition | &array_subtype_definition | &record_type_definition | &enumeration_subtype_definition | &enumeration_type_definition | &record_subtype_definition | &physical_subtype_definition | &integer_subtype_definition | &array_type_definition
/// parameter_associations: &[association_element_by_expression]
/// base_name: &function_call
/// implementation: &function_declaration
/// ```
#[derive(Debug, Deserialize)]
pub struct SubprogramCall {
    pub prefix: PrefixNodeId,
    pub implementation: NodeId<SubprogramDeclaration>,
    #[serde(default)]
    pub parameter_associations: Vec<AssociationElementNodeId>,
    #[serde(rename = "type")]
    pub return_type: Option<SubtypeDefinitionNodeId>,
}

/// ```text
/// type: &physical_type_definition | &integer_type_definition | &physical_subtype_definition | &integer_subtype_definition
/// value: int
/// literal_origin: &division_operator | &pos_attribute | &multiplication_operator | &succ_attribute | &exponentiation_operator | &pred_attribute | &identity_operator | &leftof_attribute | &low_type_attribute | &physical_int_literal | &low_array_attribute | &type_conversion | &left_array_attribute | &substraction_operator | &addition_operator | &qualified_expression | &left_type_attribute | &rightof_attribute | &right_array_attribute | &high_array_attribute | &modulus_operator | &negation_operator | &attribute_name | &simple_name | &length_array_attribute | &high_type_attribute | &right_type_attribute | &absolute_operator | &remainder_operator | &val_attribute
/// literal_length: int
#[derive(Debug, Deserialize)]
pub struct IntegerLiteral {
    pub value: i64,
}

/// ```text
/// literal_origin: &division_operator | &multiplication_operator | &negation_operator | &simple_name | &attribute_name | &exponentiation_operator | &identity_operator | &high_type_attribute | &low_type_attribute | &right_type_attribute | &type_conversion | &substraction_operator | &absolute_operator | &addition_operator | &left_type_attribute | &qualified_expression
/// literal_length: int
/// type: &floating_subtype_definition | &floating_type_definition
/// fp_value: "1.234…"
/// ```
#[derive(Debug, Deserialize)]
pub struct FloatingPointLiteral {
    #[serde(rename = "fp_value")]
    pub value: f64,
}

/// ```text
/// unit_name: &simple_name
/// value: int
/// type: &physical_type_definition
/// literal_length: int
/// ```
#[derive(Debug, Deserialize)]
pub struct PhysicalIntLiteral {
    pub value: i64,
    pub unit_name: NameNodeId,
}

subset_declaration!(PhysicalLiteral PhysicalLiteralNodeId {
    Integer(IntegerLiteral),
    Float(FloatingPointLiteral),
});

/// Wrapper expression for literals that are known to overflow their target type.
#[derive(Debug, Deserialize)]
pub struct OverflowLiteral {
    pub literal_origin: ExpressionNodeId,
}

/// ```text
/// left_limit_expr: &character_literal | &function_call | &length_array_attribute | &integer_literal | &physical_int_literal | &left_array_attribute | &enumeration_literal | &floating_point_literal | &addition_operator | &simple_name | &low_array_attribute
/// left_limit: &character_literal | &function_call | &length_array_attribute | &integer_literal | &physical_int_literal | &enumeration_literal | &left_array_attribute | &floating_point_literal | &addition_operator | &simple_name | &low_array_attribute
/// direction: "downto" | "to"
/// right_limit_expr: &character_literal | &function_call | &length_array_attribute | &multiplication_operator | &integer_literal | &physical_int_literal | &right_array_attribute | &high_array_attribute | &floating_point_literal | &addition_operator | &simple_name | &substraction_operator | &enumeration_literal
/// range_origin: &range_array_attribute | &simple_name | &reverse_range_array_attribute
/// type: &physical_type_definition | &floating_subtype_definition | &enumeration_subtype_definition | &floating_type_definition | &enumeration_type_definition | &integer_type_definition | &physical_subtype_definition | &integer_subtype_definition
/// right_limit: &character_literal | &function_call | &length_array_attribute | &multiplication_operator | &integer_literal | &physical_int_literal | &enumeration_literal | &high_array_attribute | &floating_point_literal | &addition_operator | &simple_name | &substraction_operator | &right_array_attribute
/// ```
#[derive(Debug, Deserialize)]
pub struct RangeExpression {
    pub direction: Direction,
    pub left_limit: ExpressionNodeId,
    pub right_limit: ExpressionNodeId,
}

/// ```text
/// literal_subtype: &array_subtype_definition
/// aggregate_expand_flag: bool
/// association_choices: &[choice_by_range] | &[choice_by_others] | &[choice_by_expression] | &[choice_by_name] | &[choice_by_none]
/// type: &array_subtype_definition | &record_subtype_definition | &record_type_definition
/// aggregate_info: &aggregate_info
/// determined_aggregate_flag: bool
/// ```
#[derive(Debug, Deserialize)]
pub struct Aggregate {
    #[serde(rename = "association_choices")]
    pub associations: Vec<AssociationElementNodeId>,
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

/// ```text
/// literal_subtype: &array_subtype_definition
/// bit_string_base: "BASE_2" | "BASE_16" | "BASE_NONE" | "BASE_8"
/// has_length: bool
/// literal_length: int
/// has_sign: bool
/// has_signed: bool
/// string8_id: "…"
/// string_length: int
/// type: &array_subtype_definition
/// ```
#[derive(Debug, Deserialize)]
pub struct StringLiteral {
    #[serde(rename = "string8_id")]
    pub value: Latin1String,
    pub literal_origin: Option<ExpressionNodeId>,
}

/// ```text
/// identifier: "…"
/// type: &enumeration_type_definition
/// literal_origin: &and_operator | &not_operator | &equality_operator | &right_array_attribute | &greater_than_operator | &simple_name | &attribute_name | &succ_attribute | &less_than_operator | &pred_attribute | &less_than_or_equal_operator | &leftof_attribute | &high_type_attribute | &low_type_attribute | &inequality_operator | &greater_than_or_equal_operator | &left_array_attribute | &right_type_attribute | &type_conversion | &or_operator | &qualified_expression | &left_type_attribute | &val_attribute
/// enum_pos: int
/// is_within_flag: bool
/// parent: int
/// seen_flag: bool
/// subprogram_hash: int
/// visible_flag: bool
/// ```
#[derive(Debug, Deserialize)]
pub struct EnumerationLiteral {
    pub enum_pos: u32,
    pub identifier: Identifier,
}

#[derive(Deserialize)]
#[serde(try_from = "CompactString")]
pub struct Latin1String {
    characters: SmallVec<[u8; 24]>,
}

impl Latin1String {
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

#[derive(Clone, Copy, Debug)]
pub struct LatinStringError {
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

#[repr(transparent)]
pub struct Latin1Str([u8]);

impl Latin1Str {
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

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

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
