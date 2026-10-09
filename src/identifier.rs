use std::borrow::Borrow;
use std::fmt;
use std::hash;
use std::ops::Deref;

use compact_str::CompactString;
use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

/// Case-folded VHDL identifier used for comparisons and hashing.
///
/// Regular identifiers are stored in lowercase Latin-1 form; extended identifiers
/// and character literals keep their original spelling.
#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct NormalizedIdentifier(
    /// Normalized identifier text.
    pub CompactString,
);

impl NormalizedIdentifier {
    /// Creates a normalized identifier from a string.
    #[must_use]
    pub fn new(identifier: &str) -> Self {
        if is_regular(identifier) {
            let mut normalized = CompactString::with_capacity(identifier.len());
            normalized.extend(identifier.chars().map(to_normalized));
            Self(normalized)
        } else {
            Self(CompactString::new(identifier))
        }
    }

    /// Creates an identifier from a normalized, static string.
    #[must_use]
    pub fn static_normalized(identifier: &'static str) -> Self {
        debug_assert!(is_normalized(identifier), "identifier is not normalized");
        Self(CompactString::const_new(identifier))
    }

    /// Returns the normalized identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for NormalizedIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl fmt::Display for NormalizedIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for NormalizedIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl PartialEq<str> for NormalizedIdentifier {
    fn eq(&self, other: &str) -> bool {
        debug_assert!(
            is_normalized(other),
            "tried comparing normalized identifier with non-normalized string",
        );
        self.0 == other
    }
}

/// An identifier with a normalized and an original representation.
///
/// The normalized representation is used for comparisons and hashing.
///
/// The original representation is restored on a best-effort basis from the original source code.
#[derive(Clone, Eq)]
pub struct Identifier {
    /// Case-folded form used for equality and hashing.
    pub normalized: NormalizedIdentifier,
    /// Original spelling from the source, when available.
    pub original: Option<CompactString>,
}

impl Identifier {
    /// Creates an identifier from an original source spelling.
    #[must_use]
    pub fn new(original: &str) -> Self {
        let normalized = NormalizedIdentifier::new(original);
        let original = if is_regular(original) {
            Some(CompactString::new(original))
        } else {
            None
        };
        Self {
            normalized,
            original,
        }
    }

    /// Creates an identifier from a normalized, static string.
    #[must_use]
    pub fn static_normalized(identifier: &'static str) -> Self {
        debug_assert!(is_normalized(identifier), "identifier is not normalized");
        let normalized = NormalizedIdentifier::static_normalized(identifier);
        let original = None;
        Self {
            normalized,
            original,
        }
    }

    /// Returns the original spelling, or the normalized form if none was stored.
    #[must_use]
    pub fn original(&self) -> &CompactString {
        self.original.as_ref().unwrap_or(&self.normalized.0)
    }

    /// Consumes the identifier and returns the original spelling when present.
    #[must_use]
    pub fn into_original(self) -> CompactString {
        self.original.unwrap_or(self.normalized.0)
    }

    /// Returns whether GHDL generated this identifier, such as the label `P0` that elaboration
    /// gives an unlabeled process.
    ///
    /// The normalized form of an identifier from the source is lowercase, or an extended
    /// identifier or character literal. A generated identifier contains uppercase letters, so it
    /// never collides with one from the source.
    #[must_use]
    pub fn is_generated(&self) -> bool {
        !is_normalized(&self.normalized)
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper(NormalizedIdentifier, Option<CompactString>);

        let Helper(normalized, original) = Helper::deserialize(deserializer)?;
        Ok(Self {
            normalized,
            original,
        })
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&self.normalized)?;
        tuple.serialize_element(&self.original)?;
        tuple.end()
    }
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.original(), f)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.original(), f)
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.normalized == other.normalized
    }
}

impl hash::Hash for Identifier {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.normalized.hash(state);
    }
}

impl Borrow<str> for Identifier {
    fn borrow(&self) -> &str {
        &self.normalized
    }
}

#[must_use]
fn to_normalized(ch: char) -> char {
    if matches!(ch, 'A'..='Z' | 'À'..='Ö' | 'Ø'..='Þ') {
        char::from_u32(ch as u32 + 0x20).unwrap_or_else(|| unreachable!())
    } else {
        ch
    }
}

#[must_use]
fn is_regular(identifier: &str) -> bool {
    !matches!(identifier.as_bytes().first(), Some(b'\\' | b'\''))
}

#[must_use]
fn is_normalized(identifier: &str) -> bool {
    if is_regular(identifier) {
        identifier
            .chars()
            .all(|char| !matches!(char, 'A'..='Z' | 'À'..='Ö' | 'Ø'..='Þ'))
    } else {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_normalized() {
        // Check all latin-1 characters
        for ch in '\u{00}'..='\u{ff}' {
            assert_eq!(to_normalized(ch).to_string(), ch.to_lowercase().to_string());
        }
    }

    #[test]
    fn test_identifier_new() {
        #[track_caller]
        fn check(original: &str, normalized: &str) {
            assert_eq!(&*NormalizedIdentifier::new(original), normalized);
        }

        // Character literals
        check("'z'", "'z'");
        check("'A'", "'A'");
        check("'Ö'", "'Ö'");

        // Extended identifiers
        check("\\ABCdefÄÖÜäöü\\", "\\ABCdefÄÖÜäöü\\");

        // Regular identifiers
        check("abc", "abc");
        check("aBc", "abc");
        check("abcÜ", "abcü");
    }

    #[test]
    fn test_identifier_is_generated() {
        #[track_caller]
        fn check(normalized: &str, generated: bool) {
            let identifier = Identifier {
                normalized: NormalizedIdentifier(CompactString::new(normalized)),
                original: None,
            };
            assert_eq!(identifier.is_generated(), generated);
        }

        check("P0", true);
        check("p0", false);
        check("\\P0\\", false);
        check("'A'", false);
        check("Ölabel", true);
    }
}
