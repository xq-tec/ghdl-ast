//! Shared enums and helpers used across AST node types.
//!
//! These types mirror GHDL-exported scalar attributes (modes, directions, delay
//! mechanisms, and special list encodings such as `"all"` / `"others"`).

use std::marker::PhantomData;

use super::*;

/// Range direction of a discrete or floating range (`to` / `downto`).
///
/// Corresponds to the direction in a VHDL range expression:
///
/// ```vhdl
/// subtype T is integer range 0 to 7;       -- Direction::To
/// subtype U is integer range 7 downto 0;   -- Direction::Downto
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Direction {
    /// Ascending range (`A to B`).
    #[serde(rename = "to")]
    To,
    /// Descending range (`A downto B`).
    #[serde(rename = "downto")]
    Downto,
}

impl Direction {
    /// Returns whether this direction is ascending (`to`).
    #[must_use]
    pub fn is_ascending(&self) -> bool {
        match self {
            Direction::To => true,
            Direction::Downto => false,
        }
    }
}

/// Object or port mode (`in`, `out`, `inout`, `buffer`, `linkage`).
///
/// Used on interface objects (ports, parameters, generics that carry a mode) and
/// on file declarations that declare an explicit mode.
///
/// ```vhdl
/// port (
///   clk : in  std_logic;           -- Mode::In
///   q   : out std_logic;           -- Mode::Out
///   dq  : inout std_logic;         -- Mode::InOut
///   ...
/// );
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Input mode.
    In,
    /// Output mode.
    Out,
    /// Bidirectional mode.
    InOut,
    /// Buffer mode (readable output with restricted connection rules).
    Buffer,
    /// Linkage mode (foreign / non-digital connection).
    Linkage,
    /// Mode was not determined (GHDL exports this as `"???"`).
    ///
    /// Can appear on interface constants when no mode was written and analysis
    /// left the mode unspecified.
    #[serde(rename = "???")]
    Unknown,
}

/// Delay mechanism of a signal assignment (`inertial` or `transport`).
///
/// ```vhdl
/// s <= transport '1' after 5 ns;   -- DelayMechanism::Transport
/// s <= inertial  '0' after 2 ns;   -- DelayMechanism::Inertial (also the default)
/// s <= reject 1 ns inertial '1' after 5 ns;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DelayMechanism {
    /// Inertial delay (default): pulses shorter than the delay may be rejected.
    Inertial,
    /// Transport delay: all scheduled transactions are kept.
    Transport,
}

/// Force mode of a VHDL-2008 force/release assignment (`in` / `out`).
///
/// ```vhdl
/// s <= force in  '1';   -- ForceMode::In  (effective value)
/// s <= force out '0';   -- ForceMode::Out (driving value)
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ForceMode {
    /// Force/release the effective value (`force in` / `release in`).
    In,
    /// Force/release the driving value (`force out` / `release out`).
    Out,
}

/// Optional `register` / `bus` signal kind on a guarded signal.
///
/// Only meaningful when [`SignalDeclaration::guarded_signal_flag`] (or the
/// corresponding interface flag) is true. When the signal is not guarded, GHDL
/// still stores one of these values as a default; ignore it unless the signal
/// is guarded.
///
/// ```vhdl
/// signal s : std_logic bus;       -- SignalKind::Bus
/// signal t : std_logic register;  -- SignalKind::Register
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SignalKind {
    /// `register` kind: holds the last driven value when all drivers disconnect.
    Register,
    /// `bus` kind: becomes the resolution function's disconnect value when all
    /// drivers disconnect.
    Bus,
}

/// A list of index expressions, or the special value `others`.
///
/// Used for indexed names where GHDL may encode an `others` choice as the string
/// `"others"` instead of a node-ID array.
#[derive(Debug)]
pub enum IndexList {
    /// Explicit list of index expressions.
    Items(Vec<ExpressionNodeId>),
    /// The `others` choice.
    Others,
}

impl IndexList {
    /// Returns the list of indices.
    ///
    /// # Panics
    ///
    /// Panics if the index list is [`Others`](IndexList::Others).
    #[must_use]
    pub fn items(&self) -> &[ExpressionNodeId] {
        #[expect(clippy::panic, reason = "panic is intentional")]
        match self {
            IndexList::Items(items) => items,
            IndexList::Others => panic!("expected list of indices, got 'others'"),
        }
    }
}

impl Serialize for IndexList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            IndexList::Items(items) => items.serialize(serializer),
            IndexList::Others => serializer.serialize_str("others"),
        }
    }
}

impl<'de> Deserialize<'de> for IndexList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde::de::SeqAccess;
        use serde::de::Visitor;
        use serde::de::value::SeqAccessDeserializer;

        struct IndexListVisitor;

        impl<'de> Visitor<'de> for IndexListVisitor {
            type Value = IndexList;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an array of node IDs or the string \"others\"")
            }

            fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let items = Vec::deserialize(SeqAccessDeserializer::new(visitor))?;
                Ok(IndexList::Items(items))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if value == "others" {
                    Ok(IndexList::Others)
                } else {
                    Err(Error::custom(format!("expected \"others\", got '{value}'")))
                }
            }
        }

        deserializer.deserialize_any(IndexListVisitor)
    }
}

/// A sensitivity list: either an explicit list of signals/names, or `all`.
///
/// ```vhdl
/// process (clk, rst)   -- SensitivityList::Signals([...])
/// process (all)        -- SensitivityList::All  (VHDL-2008)
/// wait on clk, rst;    -- SensitivityList::Signals([...])
/// wait on all;         -- SensitivityList::All
/// ```
#[derive(Debug)]
pub enum SensitivityList {
    /// Explicit sensitivity names (usually signal names).
    Signals(Vec<ExpressionNodeId>),
    /// VHDL-2008 `all` — every signal read in the process body.
    All,
}

impl Serialize for SensitivityList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SensitivityList::Signals(signals) => signals.serialize(serializer),
            SensitivityList::All => serializer.serialize_str("all"),
        }
    }
}

impl<'de> Deserialize<'de> for SensitivityList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde::de::SeqAccess;
        use serde::de::Visitor;
        use serde::de::value::SeqAccessDeserializer;

        struct SensitivityListVisitor;

        impl<'de> Visitor<'de> for SensitivityListVisitor {
            type Value = SensitivityList;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an array of node IDs or the string \"all\"")
            }

            fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let signals = Vec::deserialize(SeqAccessDeserializer::new(visitor))?;
                Ok(SensitivityList::Signals(signals))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if value == "all" {
                    Ok(SensitivityList::All)
                } else {
                    Err(Error::custom(format!("expected \"all\", got '{value}'")))
                }
            }
        }

        deserializer.deserialize_any(SensitivityListVisitor)
    }
}

/// A list of instantiation labels, or the special values `all` / `others`.
///
/// Used in configuration specifications and component configurations:
///
/// ```vhdl
/// for all : nand2 use entity work.nand2(rtl);     -- InstantiationList::All
/// for others : nand2 use ...;                     -- InstantiationList::Others
/// for u1, u2 : nand2 use ...;                     -- InstantiationList::Items([...])
/// ```
#[derive(Debug)]
pub enum InstantiationList {
    /// Explicit instantiation labels.
    Items(Vec<NodeId<SimpleName>>),
    /// Every matching instantiation (`all`).
    All,
    /// Remaining matching instantiations (`others`).
    Others,
}

impl Serialize for InstantiationList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InstantiationList::Items(items) => items.serialize(serializer),
            InstantiationList::All => serializer.serialize_str("all"),
            InstantiationList::Others => serializer.serialize_str("others"),
        }
    }
}

impl<'de> Deserialize<'de> for InstantiationList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde::de::SeqAccess;
        use serde::de::Visitor;
        use serde::de::value::SeqAccessDeserializer;

        struct InstantiationListVisitor;

        impl<'de> Visitor<'de> for InstantiationListVisitor {
            type Value = InstantiationList;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an array of node IDs or the string \"all\"/\"others\"")
            }

            fn visit_seq<V>(self, visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let items = Vec::deserialize(SeqAccessDeserializer::new(visitor))?;
                Ok(InstantiationList::Items(items))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                match value {
                    "all" => Ok(InstantiationList::All),
                    "others" => Ok(InstantiationList::Others),
                    _ => Err(Error::custom(format!(
                        "expected \"all\" or \"others\", got '{value}'"
                    ))),
                }
            }
        }

        deserializer.deserialize_any(InstantiationListVisitor)
    }
}

/// Sentinel node used when GHDL could not form a valid AST node.
///
/// A single global error node exists at [`Error::GLOBAL_ID`] and is also used as
/// the named entity of unresolved names (see [`crate::NamedEntity::Unresolved`]).
#[derive(Debug, Deserialize, Serialize)]
pub struct Error {}

impl Error {
    /// The ID of the global error node.
    pub const GLOBAL_ID: NodeId<Self> = NodeId(IdPrimitive::new(2).unwrap(), PhantomData);
}

/// Overload-resolution candidate list built during name analysis.
///
/// This is an analysis artifact; it is not needed for simulation of an already
/// analyzed design.
#[derive(Debug, Deserialize, Serialize)]
pub struct OverloadList {}
