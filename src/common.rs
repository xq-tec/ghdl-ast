use std::marker::PhantomData;

use super::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Direction {
    #[serde(rename = "to")]
    To,
    #[serde(rename = "downto")]
    Downto,
}

impl Direction {
    #[must_use]
    pub fn is_ascending(&self) -> bool {
        match self {
            Direction::To => true,
            Direction::Downto => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    In,
    Out,
    InOut,
    Buffer,
    Linkage,
}

#[derive(Debug)]
pub enum IndexList {
    Items(Vec<ExpressionNodeId>),
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {}

impl Error {
    /// The ID of the global error node.
    pub const GLOBAL_ID: NodeId<Self> = NodeId(IdPrimitive::new(2).unwrap(), PhantomData);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OverloadList {}
