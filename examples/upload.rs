use resourcespace_client::api::resource::UploadMultipartRequest;
use resourcespace_client::client::Client;

use std::path::Path;

#[tokio::main]
async fn main() {
    dotenvy::from_path("examples/.env").ok();

    let base_url = std::env::var("RS_BASE_URL").expect("RS_BASE_URL not set");
    let user = std::env::var("RS_USER").expect("RS_USER not set");
    let password = std::env::var("RS_PASS").expect("RS_PASS not set");
    let key = std::env::var("RS_KEY").expect("RS_KEY not set");

    let client = Client::builder()
        .base_url(&base_url)
        .expect("Error when setting base_url")
        .user_key(&user, &key)
        // .session_key(&user, &password)
        .build()
        .await
        .expect("Error when building client");

    let result = client
        .resource()
        .upload_multipart(
            UploadMultipartRequest::new(91287, false, false),
            Path::new("pexels.jpg"),
        )
        .await;

    match result {
        Ok(response) => println!("{:#?}", response),
        Err(e) => println!("Error: {}", e),
    }
}
