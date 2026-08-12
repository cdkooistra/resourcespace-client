//! Shared helpers for the integration tests.
#![allow(dead_code)]

use std::sync::OnceLock;

use resourcespace_client::Client;
use resourcespace_client::api::FieldValue;
use resourcespace_client::api::collection::{
    AddResourceToCollectionRequest, CreateCollectionRequest, SaveCollectionColdata,
    SaveCollectionRequest,
};
use resourcespace_client::api::metadata::UpdateFieldRequest;
use resourcespace_client::api::resource::{
    CreateResourceRequest, UpdateRelatedResourceRequest, UploadMultipartRequest, UploadSource,
};

/// Resource type used for seeded fixtures.
pub const FIXTURE_RESOURCE_TYPE: u32 = 1;

/// `dotenvy` mutates process environment, so load it once rather than
/// racing on it from every `#[tokio::test]`.
fn load_env() {
    static ENV: OnceLock<()> = OnceLock::new();
    ENV.get_or_init(|| {
        dotenvy::from_path("tests/.env").ok();
    });
}

/// Builds a client against the live instance, or returns `None` when
/// `RS_BASE_URL` isn't set.
///
/// Tests skip themselves on `None` so `cargo test` still passes on a
/// machine without credentials; see `tests/example.env` for the full set.
pub async fn live_client() -> Option<Client> {
    load_env();

    let base_url = std::env::var("RS_BASE_URL").ok()?;
    let user = std::env::var("RS_USER").expect("RS_USER");
    // `example.env` ships `RS_KEY=`, so an empty value means "no key".
    let key = std::env::var("RS_KEY").ok().filter(|k| !k.is_empty());

    let client = match key {
        Some(key) => Client::builder()
            .base_url(&base_url)
            .user_key(&user, &key)
            .build()
            .await
            .expect("build client"),
        None => {
            let pass = std::env::var("RS_PASS").expect("RS_PASS");
            Client::builder()
                .base_url(&base_url)
                .session_key(&user, &pass)
                .build()
                .await
                .expect("build client")
        }
    };

    Some(client)
}

/// IDs of the records created by [`seed`].
#[derive(Debug)]
pub struct Seed {
    pub resource: u32,
    pub related: u32,
    pub collection: u32,
    /// A public collection, so `search_public_collections` has something to
    /// return.
    pub public_collection: u32,
    /// A root-level featured collection, so `get_featured_collections` has
    /// something to return.
    pub featured_collection: u32,
}

/// URL used for the arbitrary image attached to the seeded resource,
/// picsum.photos is a handy API for generating placeholder images.
const FIXTURE_ASSET_URL: &str = "https://picsum.photos/640/320.jpg";
const FIXTURE_ASSET_FILENAME: &str = "rs-client-seed.jpg";

async fn fixture_asset() -> reqwest::Body {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("build image client")
        .get(FIXTURE_ASSET_URL)
        .send()
        .await
        .expect("fetch fixture image")
        .error_for_status()
        .expect("fixture image response ok")
        .bytes()
        .await
        .expect("read fixture image")
        .to_vec()
        .into()
}

/// Creates a resource with a title and an attached file, a second resource
/// related to it, and three collections: one holding the resource, one
/// public, one featured.
pub async fn seed(client: &Client) -> Seed {
    let resource = client
        .resource()
        .create_resource(CreateResourceRequest::new(FIXTURE_RESOURCE_TYPE))
        .await
        .expect("create resource");
    let related = client
        .resource()
        .create_resource(CreateResourceRequest::new(FIXTURE_RESOURCE_TYPE))
        .await
        .expect("create related resource");

    client
        .metadata()
        .update_field(UpdateFieldRequest::new(
            resource,
            "title",
            FieldValue::from("rs-client seed resource"),
        ))
        .await
        .expect("set title");

    client
        .resource()
        .upload_multipart(
            UploadMultipartRequest::new(resource, false, false),
            UploadSource::from_stream(fixture_asset().await, FIXTURE_ASSET_FILENAME),
        )
        .await
        .expect("upload file to seed resource");

    client
        .resource()
        .update_related_resource(UpdateRelatedResourceRequest::new(resource, [related]).add(true))
        .await
        .expect("relate resources");

    let collection = client
        .collection()
        .create_collection(CreateCollectionRequest::new("rs-client seed collection"))
        .await
        .expect("create collection");

    client
        .collection()
        .add_resource_to_collection(AddResourceToCollectionRequest::new(resource, collection))
        .await
        .expect("add resource to collection");

    // Collection `type` 3 is featured and 4 is public; both require `public`
    // to be set as well.
    let public_collection = client
        .collection()
        .create_collection(CreateCollectionRequest::new("rs-client seed public"))
        .await
        .expect("create public collection");
    client
        .collection()
        .save_collection(SaveCollectionRequest::new(
            public_collection,
            // `save_collection` replaces rather than patches: any field left
            // unset is cleared, so the name has to be repeated here.
            SaveCollectionColdata::new()
                .name("rs-client seed public")
                .public(true)
                .r#type(4),
        ))
        .await
        .expect("make collection public");

    let featured_collection = client
        .collection()
        .create_collection(CreateCollectionRequest::new("rs-client seed featured"))
        .await
        .expect("create featured collection");
    client
        .collection()
        .save_collection(SaveCollectionRequest::new(
            featured_collection,
            SaveCollectionColdata::new()
                .name("rs-client seed featured")
                .public(true)
                .r#type(3)
                .force_featured_collection_type(true),
        ))
        .await
        .expect("make collection featured");

    Seed {
        resource,
        related,
        collection,
        public_collection,
        featured_collection,
    }
}

#[macro_export]
macro_rules! live_client_or_skip {
    () => {
        match $crate::common::live_client().await {
            Some(client) => client,
            None => {
                eprintln!("skipping {}: RS_BASE_URL not set", module_path!());
                return Ok(());
            }
        }
    };
}
