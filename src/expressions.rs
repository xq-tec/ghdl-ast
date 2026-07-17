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

#[derive(Debug, Deserialize, Serialize)]
pub struct UnaryOperator {
    pub kind: UnaryOperatorKind,
    pub operand: ExpressionNodeId,
    pub implementation: FunctionImplementationNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub left: ExpressionNodeId,
    pub right: ExpressionNodeId,
    pub implementation: FunctionImplementationNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionCall {
    pub prefix: PrefixNodeId,
    pub implementation: FunctionImplementationNodeId,
    #[serde(default)]
    pub parameter_associations: Vec<AssociationElementNodeId>,
    #[serde(rename = "type")]
    pub return_type: SubtypeDefinitionNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntegerLiteral {
    pub value: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FloatingPointLiteral {
    #[serde(rename = "fp_value", deserialize_with = "deserialize_f64")]
    pub value: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalIntLiteral {
    pub value: i64,
    pub unit_name: NameNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalFpLiteral {
    #[serde(rename = "fp_value", deserialize_with = "deserialize_f64")]
    pub value: f64,
    pub unit_name: NameNodeId,
}

subset_declaration!(PhysicalLiteral PhysicalLiteralNodeId {
    PhysicalInt(PhysicalIntLiteral),
    PhysicalFp(PhysicalFpLiteral),
});

/// Wrapper expression for literals that are known to overflow their target type.
#[derive(Debug, Deserialize, Serialize)]
pub struct OverflowLiteral {
    pub literal_origin: ExpressionNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RangeExpression {
    pub direction: Direction,
    pub left_limit: ExpressionNodeId,
    pub right_limit: ExpressionNodeId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Aggregate {
    #[serde(rename = "association_choices")]
    pub associations: Vec<ChoiceNodeId>,
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StringLiteral {
    #[serde(rename = "string8_id")]
    pub value: Latin1String,
    pub literal_origin: Option<ExpressionNodeId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnumerationLiteral {
    pub enum_pos: u32,
    pub identifier: Identifier,
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterLiteral {}

#[derive(Debug, Deserialize, Serialize)]
pub struct QualifiedExpression {}

#[derive(Debug, Deserialize, Serialize)]
pub struct NullLiteral {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AllocatorByExpression {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AllocatorBySubtype {}

#[derive(Debug, Deserialize, Serialize)]
pub struct AggregateInfo {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParenthesisExpression {}

#[derive(Debug, Deserialize, Serialize)]
pub struct TypeConversion {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleAggregate {}

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
