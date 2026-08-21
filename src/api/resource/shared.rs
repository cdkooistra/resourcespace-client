use serde::Serializer;
use serde_with::SerializeAs;

use crate::api::shared::FieldValue;

pub(super) struct FieldValueAsString;

impl SerializeAs<FieldValue> for FieldValueAsString {
    fn serialize_as<S: Serializer>(val: &FieldValue, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&val.to_wire_string())
    }
}
