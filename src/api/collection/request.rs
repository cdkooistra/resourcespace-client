use serde::Serialize;
use serde_with::json::JsonString;
use serde_with::{serde_as, skip_serializing_none};
use validator::Validate;

use crate::api::shared::{List, SortOrder, bool_as_u8, opt_bool_as_u8};

// Referenced only from doc links below; the import keeps it resolvable.
#[allow(unused_imports)]
use super::CollectionApi;

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AddResourceToCollectionRequest {
    /// The ID of the resource to add.
    pub resource: u32,
    /// The ID of the collection to add the resource to.
    pub collection: u32,
    /// Adds every resource matching this search query, rather than just
    /// `resource`.
    pub search: Option<String>,
}

impl AddResourceToCollectionRequest {
    pub fn new(resource: u32, collection: u32) -> Self {
        Self {
            resource,
            collection,
            search: None,
        }
    }

    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemoveResourceFromCollectionRequest {
    /// The ID of the resource to remove.
    pub resource: u32,
    /// The ID of the collection to remove the resource from.
    pub collection: u32,
}

impl RemoveResourceFromCollectionRequest {
    pub fn new(resource: u32, collection: u32) -> Self {
        Self {
            resource,
            collection,
        }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateCollectionRequest {
    /// The name of the new collection.
    pub name: String,
    /// If true, marks this collection as an upload collection.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub forupload: Option<bool>,
}

impl CreateCollectionRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            forupload: None,
        }
    }

    pub fn forupload(mut self, forupload: bool) -> Self {
        self.forupload = Some(forupload);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeleteCollectionRequest {
    /// The ID of the collection to delete.
    ///
    /// Sent as `ref`, not `collection`. The knowledge base documents this
    /// parameter as `$collection`, but RS 11.0 only accepts `ref` — sending
    /// `collection` leaves the parameter unbound and the call returns `false`.
    #[serde(rename = "ref")]
    pub collection: u32,
}

impl DeleteCollectionRequest {
    pub fn new(collection: u32) -> Self {
        Self { collection }
    }
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct SearchPublicCollectionsRequest {
    /// Optional search string to filter collections by name.
    pub search: Option<String>,
    /// Field name to order results by.
    pub order_by: Option<String>,
    /// Sort direction for the results.
    pub sort: Option<SortOrder>,
    /// If true, excludes theme/featured collections from results.
    /// Defaults to `true` server-side when omitted.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_themes: Option<bool>,
    /// If true, excludes public collections from results.
    /// Defaults to `false` server-side when omitted.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_public: Option<bool>,
}

impl SearchPublicCollectionsRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.search = Some(search.into());
        self
    }

    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn sort(mut self, sort: SortOrder) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn exclude_themes(mut self, exclude_themes: bool) -> Self {
        self.exclude_themes = Some(exclude_themes);
        self
    }

    pub fn exclude_public(mut self, exclude_public: bool) -> Self {
        self.exclude_public = Some(exclude_public);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetCollectionRequest {
    /// The ID of the collection to retrieve.
    #[serde(rename = "ref")]
    pub collection_id: u32,
}

impl GetCollectionRequest {
    pub fn new(collection_id: u32) -> Self {
        Self { collection_id }
    }
}

#[non_exhaustive]
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SaveCollectionRequest {
    /// The ID of the collection to save.
    #[serde(rename = "ref")]
    pub collection_id: u32,
    /// JSON object containing the collection fields to update (e.g. name, description, public).
    #[serde_as(as = "JsonString")]
    pub coldata: SaveCollectionColdata,
}

impl SaveCollectionRequest {
    pub fn new(collection_id: u32, coldata: SaveCollectionColdata) -> Self {
        Self {
            collection_id,
            coldata,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Validate)]
pub struct SaveCollectionColdata {
    /// Comma-separated value of keywords to be associated with this collection.
    pub keywords: Option<List<String>>,
    /// If true, other users are allowed to add/remove resources when collection is shared or is public.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_changes: Option<bool>,
    /// Comma-separated value of users to attach to the collection.
    pub users: Option<List<String>>,
    /// Collection name.
    pub name: Option<String>,
    /// If true, public. Otherwise private (legacy).
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub public: Option<bool>,
    /// 0 = standard, 3 = Featured collection, 4 = public. If 3 or 4 then public should be set to 1.
    #[serde(rename = "type")]
    pub r#type: Option<u8>,
    /// ID of parent featured collection. Set to 0 to create a new root level collection (see below). Applies to Featured collections only.
    pub parent: Option<u32>,
    /// If true, creates a root level featured collection (parent=0). Applies to Featured collections only.
    #[serde(
        serialize_with = "opt_bool_as_u8",
        skip_serializing_if = "Option::is_none"
    )]
    pub force_featured_collection_type: Option<bool>,
    /// 0 = no image, 1 = most popular image, 10 - most popular images, 100 - manually select image. Applies to Featured collections only.
    pub thumbnail_selection_method: Option<u32>,
    /// Resource ID to use as thumbnail. Only if thumbnail_selection_method =100. Applies to Featured collections only.
    pub bg_img_resource_ref: Option<u32>,
}

impl SaveCollectionColdata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keywords(mut self, keywords: impl Into<List<String>>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    pub fn allow_changes(mut self, allow_changes: bool) -> Self {
        self.allow_changes = Some(allow_changes);
        self
    }

    pub fn users(mut self, users: impl Into<List<String>>) -> Self {
        self.users = Some(users.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn public(mut self, public: bool) -> Self {
        self.public = Some(public);
        self
    }

    pub fn r#type(mut self, r#type: u8) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn parent(mut self, parent: u32) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn force_featured_collection_type(mut self, force: bool) -> Self {
        self.force_featured_collection_type = Some(force);
        self
    }

    pub fn thumbnail_selection_method(mut self, method: u32) -> Self {
        self.thumbnail_selection_method = Some(method);
        self
    }

    pub fn bg_img_resource_ref(mut self, resource_ref: u32) -> Self {
        self.bg_img_resource_ref = Some(resource_ref);
        self
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ShowHideCollectionRequest {
    /// The ID of the collection to show or hide.
    pub collection: u32,
    /// If true, shows the collection in the drop-down list. Otherwise, hides it.
    #[serde(serialize_with = "bool_as_u8")]
    pub show: bool,
    /// The ID of the user whose drop-down list is being updated.
    pub user: u32,
}

impl ShowHideCollectionRequest {
    pub fn new(collection: u32, show: bool, user: u32) -> Self {
        Self {
            collection,
            show,
            user,
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SendCollectionToAdminRequest {
    /// The ID of the collection to send to the administrator for review.
    pub collection: u32,
}

impl SendCollectionToAdminRequest {
    pub fn new(collection: u32) -> Self {
        Self { collection }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetFeaturedCollectionsRequest {
    /// The ID of the parent featured collection (category) to retrieve children for. Use 0 for top-level.
    pub parent: u32,
}

impl GetFeaturedCollectionsRequest {
    pub fn new(parent: u32) -> Self {
        Self { parent }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeleteResourcesInCollectionRequest {
    /// The ID of the collection whose resources should all be deleted.
    pub collection: u32,
}

impl DeleteResourcesInCollectionRequest {
    pub fn new(collection: u32) -> Self {
        Self { collection }
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetCollectionsResourceCountRequest {
    /// Comma-separated list of collection IDs to retrieve resource counts for.
    #[serde(rename = "refs")]
    pub collection_ids: List<u32>,
}

impl GetCollectionsResourceCountRequest {
    pub fn new(collection_ids: impl Into<List<u32>>) -> Self {
        Self {
            collection_ids: collection_ids.into(),
        }
    }
}
