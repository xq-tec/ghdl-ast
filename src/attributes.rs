//! Predefined and user attribute expressions (LRM clause 16 / 6.7).
//!
//! [`Attribute`] is a predefined attribute application (`prefix'ATTR`).
//! [`AttributeValue`] is the value produced by an attribute specification for a
//! particular designated entity.

use super::*;

/// Value of an attribute for a designated entity after attribute specification.
///
/// Links a designated named entity to the expression from an
/// [`AttributeSpecification`](crate::AttributeSpecification) and the resulting
/// analyzed type.
///
/// ```vhdl
/// attribute keep : boolean;
/// attribute keep of clk : signal is true;
/// -- AttributeValue.designated_entity = clk, expression comes from the spec
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct AttributeValue {
    /// Analyzed type of the attribute value.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Entity (signal, variable, …) this attribute value applies to.
    pub designated_entity: Option<NamedEntityNodeId>,
    /// Attribute specification that created this value.
    pub attribute_specification: Option<NodeId<AttributeSpecification>>,
}

/// Predefined attribute application (`prefix'kind`).
///
/// Distinguishes type attributes (`'left`, `'image`), array attributes
/// (`'length`, `'range`), signal attributes (`'event`, `'last_value`), and AMS
/// attributes. The [`kind`](Self::kind) enum encodes which predefined attribute
/// was applied. Parameters such as `'image(x)` or `'val(n)` are stored on this
/// node ([`parameter`](Self::parameter) / [`parameter_2`](Self::parameter_2)…).
///
/// ```vhdl
/// integer'high
/// vec'length
/// clk'event
/// real'image(3.14)
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct Attribute {
    /// Prefix the attribute is applied to.
    pub prefix: PrefixNodeId,
    /// Which predefined attribute was selected.
    pub kind: AttributeKind,
    /// First attribute parameter (`ATTR(param)`), when present.
    pub parameter: Option<ExpressionNodeId>,
    /// Second attribute parameter (AMS attributes).
    pub parameter_2: Option<ExpressionNodeId>,
    /// Third attribute parameter (AMS attributes such as `'ztf`).
    pub parameter_3: Option<ExpressionNodeId>,
    /// Fourth attribute parameter (AMS attributes such as `'ztf`).
    pub parameter_4: Option<ExpressionNodeId>,
    /// Analyzed result type of the attribute application.
    #[serde(rename = "type")]
    pub typ: Option<SubtypeDefinitionNodeId>,
    /// Index subtype for array attributes that yield a discrete range / type.
    pub index_subtype: Option<SubtypeDefinitionNodeId>,
}

/// Discriminant for a predefined VHDL attribute kind (LRM clause 16).
///
/// Covers type attributes (`'left`, `'image`), array attributes (`'length`,
/// `'range`), signal attributes (`'event`, `'last_value`), AMS attributes, and
/// naming attributes (`'simple_name`, `'path_name`). Variant names follow GHDL's
/// exported `kind` strings (snake case). Several attributes share a source
/// spelling (for example `'left` / `'right`) but are split into type vs array
/// variants after analysis.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeKind {
    /// `TYPE'base`
    Base,
    /// `OBJECT'subtype`
    Subtype,
    /// `ARRAY'element`
    Element,
    /// `NATURE'across`
    Across,
    /// `NATURE'through`
    Through,
    /// `NATURE'reference`
    NatureReference,
    /// `TYPE'left`
    LeftType,
    /// `TYPE'right`
    RightType,
    /// `TYPE'high`
    HighType,
    /// `TYPE'low`
    LowType,
    /// `TYPE'ascending`
    AscendingType,
    /// `TYPE'image`
    Image,
    /// `TYPE'value`
    Value,
    /// `TYPE'pos`
    Pos,
    /// `TYPE'val`
    Val,
    /// `TYPE'succ`
    Succ,
    /// `TYPE'pred`
    Pred,
    /// `TYPE'leftof`
    Leftof,
    /// `TYPE'rightof`
    Rightof,
    /// `SIGNAL'slew`
    SignalSlew,
    /// `QUANTITY'slew`
    QuantitySlew,
    /// `SIGNAL'ramp`
    Ramp,
    /// `QUANTITY'zoh`
    Zoh,
    /// `QUANTITY'ltf`
    Ltf,
    /// `QUANTITY'ztf`
    Ztf,
    /// `QUANTITY'dot`
    Dot,
    /// `QUANTITY'integ`
    Integ,
    /// `QUANTITY'delayed`
    QuantityDelayed,
    /// `QUANTITY'above`
    Above,
    /// `SIGNAL'delayed`
    Delayed,
    /// `SIGNAL'stable`
    Stable,
    /// `SIGNAL'quiet`
    Quiet,
    /// `SIGNAL'transaction`
    Transaction,
    /// `SIGNAL'event`
    Event,
    /// `SIGNAL'active`
    Active,
    /// `SIGNAL'last_event`
    LastEvent,
    /// `SIGNAL'last_active`
    LastActive,
    /// `SIGNAL'last_value`
    LastValue,
    /// `SIGNAL'driving`
    Driving,
    /// `SIGNAL'driving_value`
    DrivingValue,
    /// `ARCHITECTURE'behavior` (VHDL-87)
    Behavior,
    /// `ARCHITECTURE'structure` (VHDL-87)
    Structure,
    /// `NAMED_ENTITY'simple_name`
    SimpleName,
    /// `NAMED_ENTITY'instance_name`
    InstanceName,
    /// `NAMED_ENTITY'path_name`
    PathName,
    /// `MODE_VIEW'converse`
    Converse,
    /// `ARRAY'left`
    LeftArray,
    /// `ARRAY'right`
    RightArray,
    /// `ARRAY'high`
    HighArray,
    /// `ARRAY'low`
    LowArray,
    /// `ARRAY'length`
    LengthArray,
    /// `ARRAY'ascending`
    AscendingArray,
    /// `ARRAY'range`
    RangeArray,
    /// `ARRAY'reverse_range`
    ReverseRangeArray,
}

impl fmt::Display for AttributeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AttributeKind::*;
        #[expect(clippy::match_same_arms, reason = "clearer this way")]
        let attribute_name = match self {
            Base => "BASE",
            Subtype => "SUBTYPE",
            Element => "ELEMENT",
            Across => "ACROSS",
            Through => "THROUGH",
            NatureReference => "REFERENCE",
            LeftType => "LEFT",
            RightType => "RIGHT",
            HighType => "HIGH",
            LowType => "LOW",
            AscendingType => "ASCENDING",
            Image => "IMAGE",
            Value => "VALUE",
            Pos => "POS",
            Val => "VAL",
            Succ => "SUCC",
            Pred => "PRED",
            Leftof => "LEFTOF",
            Rightof => "RIGHTOF",
            SignalSlew => "SLEW",
            QuantitySlew => "SLEW",
            Ramp => "RAMP",
            Zoh => "ZOH",
            Ltf => "LTF",
            Ztf => "ZTF",
            Dot => "DOT",
            Integ => "INTEG",
            QuantityDelayed => "DELAYED",
            Above => "ABOVE",
            Delayed => "DELAYED",
            Stable => "STABLE",
            Quiet => "QUIET",
            Transaction => "TRANSACTION",
            Event => "EVENT",
            Active => "ACTIVE",
            LastEvent => "LAST_EVENT",
            LastActive => "LAST_ACTIVE",
            LastValue => "LAST_VALUE",
            Driving => "DRIVING",
            DrivingValue => "DRIVING_VALUE",
            Behavior => "BEHAVIOR",
            Structure => "STRUCTURE",
            SimpleName => "SIMPLE_NAME",
            InstanceName => "INSTANCE_NAME",
            PathName => "PATH_NAME",
            Converse => "CONVERSE",
            LeftArray => "LEFT",
            RightArray => "RIGHT",
            HighArray => "HIGH",
            LowArray => "LOW",
            LengthArray => "LENGTH",
            AscendingArray => "ASCENDING",
            RangeArray => "RANGE",
            ReverseRangeArray => "REVERSE_RANGE",
        };
        fmt::Display::fmt(attribute_name, f)
    }
}
