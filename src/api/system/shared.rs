use serde::Deserialize;

// Referenced only from the doc link below; the import keeps it resolvable.
#[allow(unused_imports)]
use super::SystemCheck;

/// Deserializes a string or a bare number alike into `Option<String>`.
///
/// `SystemCheck::info` is the only place this is needed: ResourceSpace sends
/// it as a number for `recent_user_count` and a string everywhere else, and
/// no `serde_with` combinator covers "any scalar to String".
pub(super) fn scalar_as_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(
        match Option::<serde_json::Value>::deserialize(deserializer)? {
            Some(serde_json::Value::String(s)) => Some(s),
            Some(serde_json::Value::Number(n)) => Some(n.to_string()),
            Some(serde_json::Value::Bool(b)) => Some(b.to_string()),
            _ => None,
        },
    )
}
