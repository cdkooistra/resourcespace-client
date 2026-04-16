pub mod collection;
pub mod message;
pub mod metadata;
pub mod resource;
pub mod search;
pub mod system;
pub mod user;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc
}
