use serde::Serializer;

/// Serializes a node ID as the literal `NULL` when absent.
///
/// `api_set_node` matches on the uppercase string `NULL` to decide between
/// creating and updating, so an omitted or numeric value will not do.
pub(super) fn node_id_or_null<S: Serializer>(id: &Option<u32>, s: S) -> Result<S::Ok, S::Error> {
    match id {
        Some(id) => s.serialize_str(&id.to_string()),
        None => s.serialize_str("NULL"),
    }
}
