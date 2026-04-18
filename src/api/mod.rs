pub mod collection;
pub mod message;
pub mod metadata;
pub mod resource;
pub mod search;
pub mod system;
pub mod user;

use serde::Serialize;
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
/// List::from(42)               // single value
/// List::from([1, 2, 3])        // array
/// List::from(vec![1, 2, 3])    // vec
/// ```
///
/// This type exists to satisfy ResourceSpace API parameters that expect
/// comma-separated values (e.g. `"1,2,3"`), while keeping call sites
/// type-safe and free of manual string joining.
#[serde_as]
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct List<T: Display>(#[serde_as(as = "StringWithSeparator::<CommaSeparator, T>")] Vec<T>);

// u32
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

// String
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
