pub mod collection;
pub mod message;
pub mod metadata;
pub mod resource;
pub mod search;
pub mod system;
pub mod user;

use serde::{Serialize, Serializer};
use serde_with::{StringWithSeparator, formats::CommaSeparator, serde_as};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

/// A list of values that serializes to a comma-separated string.
///
/// Accepts a single value, an array, or a [`Vec`] via [`Into`] conversions,
/// making it ergonomic to pass one or many values at call sites:
///
/// ```no_run
/// # use resourcespace_client::api::List;
/// let _ = List::from(42);               // single value
/// let _ = List::from([1, 2, 3]);        // array
/// let _ = List::from(vec![1, 2, 3]);    // vec
/// ```
///
/// This type exists to satisfy ResourceSpace API parameters that expect
/// comma-separated values (e.g. `"1,2,3"`), while keeping call sites
/// type-safe and free of manual string joining.
#[serde_as]
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct List<T: Display>(#[serde_as(as = "StringWithSeparator::<CommaSeparator, T>")] Vec<T>);

impl<T: Display> List<T> {
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    pub fn as_slice(&self) -> &[T] {
        &self.0
    }
}

// List<u32>
impl From<u32> for List<u32> {
    fn from(val: u32) -> Self {
        Self(vec![val])
    }
}
impl From<Vec<u32>> for List<u32> {
    fn from(vals: Vec<u32>) -> Self {
        Self(vals)
    }
}
impl<const N: usize> From<[u32; N]> for List<u32> {
    fn from(arr: [u32; N]) -> Self {
        Self(arr.into_iter().collect())
    }
}

// List<String>
impl From<String> for List<String> {
    fn from(val: String) -> Self {
        Self(vec![val])
    }
}
impl From<&str> for List<String> {
    fn from(val: &str) -> Self {
        Self(vec![val.to_string()])
    }
}
impl From<Vec<String>> for List<String> {
    fn from(vals: Vec<String>) -> Self {
        Self(vals)
    }
}
impl From<Vec<&str>> for List<String> {
    fn from(vals: Vec<&str>) -> Self {
        Self(vals.into_iter().map(|s| s.to_string()).collect())
    }
}
impl<const N: usize> From<[String; N]> for List<String> {
    fn from(arr: [String; N]) -> Self {
        Self(arr.into_iter().collect())
    }
}
impl<const N: usize> From<[&str; N]> for List<String> {
    fn from(arr: [&str; N]) -> Self {
        Self(arr.into_iter().map(|s| s.to_string()).collect())
    }
}

// extend From and FromIterator for List
impl<T: Display> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T: Display + Clone> From<&[T]> for List<T> {
    fn from(arr: &[T]) -> Self {
        Self(arr.to_vec())
    }
}

/// Serializes a `bool` as an integer (`1` for `true`, `0` for `false`).
///
/// ResourceSpace expects boolean values to be serialized as integers.
fn bool_as_u8<S: Serializer>(b: &bool, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u8(if *b { 1 } else { 0 })
}

/// Serializes an `Option<bool>` as an integer (`1` for `Some(true)`, `0` for `Some(false)` or `None`).
///
/// ResourceSpace expects boolean values to be serialized as integers.
fn opt_bool_as_u8<S: Serializer>(b: &Option<bool>, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u8(
        // In case a Request struct has an `Option<bool>` field that is `None`,
        // `skip_serializing_if` will omit it from the request body.
        // If that does not happen, we should panic to notify a bad Request struct.
        if b.expect("opt_bool_as_u8 called on None; pair with skip_serializing_if") {
            1
        } else {
            0
        },
    )
}

/// The value to set for a metadata field.
///
/// Accepts plain text, keywords or a list of node IDs via named constructors:
///
/// ```no_run
/// # use resourcespace_client::api::FieldValue;
/// let _ = FieldValue::from("hello");                      // plain text
/// let _ = FieldValue::from(["red"]);                      // single keyword
/// let _ = FieldValue::from(["red", "blue"]);              // multiple keywords
/// let _ = FieldValue::from(["Doe, John", "Smith, Jane"]); // multiple quoted keywords
/// let _ = FieldValue::from(42u32);                        // single node ID
/// let _ = FieldValue::from([1u32, 2, 3]);                 // multiple node IDs
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    /// A text value e.g. "hello", used for single text values.
    Text(String),
    /// (Multiple) keyword values; each value is quoted if it contains a comma.
    Keywords(List<String>),
    /// A list of node IDs, sets nodevalues = true automatically.
    Nodes(List<u32>),
}

impl FieldValue {
    pub(crate) fn to_wire_string(&self) -> String {
        match self {
            Self::Text(s) => s.to_owned(),
            Self::Keywords(vs) => vs
                .as_slice()
                .iter()
                .map(|s| {
                    if s.contains(',') {
                        format!("\"{s}\"")
                    } else {
                        s.to_owned()
                    }
                })
                .collect::<Vec<_>>()
                .join(","),
            Self::Nodes(ids) => ids
                .as_slice()
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(","),
        }
    }
}

// Text
impl From<&str> for FieldValue {
    fn from(val: &str) -> Self {
        Self::Text(val.to_owned())
    }
}

impl From<String> for FieldValue {
    fn from(val: String) -> Self {
        Self::Text(val)
    }
}

// Nodes
impl From<u32> for FieldValue {
    fn from(val: u32) -> Self {
        Self::Nodes(List::from(val))
    }
}

impl From<Vec<u32>> for FieldValue {
    fn from(val: Vec<u32>) -> Self {
        Self::Nodes(List::from(val))
    }
}

impl<const N: usize> From<[u32; N]> for FieldValue {
    fn from(val: [u32; N]) -> Self {
        Self::Nodes(List::from(val))
    }
}

// Keywords
impl From<Vec<String>> for FieldValue {
    fn from(val: Vec<String>) -> Self {
        Self::Keywords(List::from(val))
    }
}

impl From<Vec<&str>> for FieldValue {
    fn from(val: Vec<&str>) -> Self {
        Self::Keywords(List::from(val))
    }
}

impl<const N: usize> From<[String; N]> for FieldValue {
    fn from(val: [String; N]) -> Self {
        Self::Keywords(List::from(val))
    }
}

impl<const N: usize> From<[&str; N]> for FieldValue {
    fn from(val: [&str; N]) -> Self {
        Self::Keywords(List::from(val))
    }
}
