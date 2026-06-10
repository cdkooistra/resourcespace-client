use resourcespace_client::api::resource::UploadFileByUrlRequest;
use resourcespace_client::api::resource::UploadMultipartRequest;
use resourcespace_client::api::resource::UploadSource;
use resourcespace_client::client::Client;

#[tokio::main]
async fn main() {
    dotenvy::from_path("examples/.env").ok();

    let base_url = std::env::var("RS_BASE_URL").expect("RS_BASE_URL not set");
    let user = std::env::var("RS_USER").expect("RS_USER not set");
    let _password = std::env::var("RS_PASS").expect("RS_PASS not set");
    let key = std::env::var("RS_KEY").expect("RS_KEY not set");

    let client = Client::builder()
        .base_url(&base_url)
        .user_key(&user, &key)
        // .session_key(&user, &password)
        .build()
        .await
        .expect("Error when building client");

    // upload_multipart using a file path

    let result = client
        .resource()
        .upload_multipart(
            UploadMultipartRequest::new(91287, false, false),
            "pexels.jpg", // automatically becomes PathBuf
        )
        .await;

    match result {
        Ok(response) => println!("{:#?}", response),
        Err(e) => println!("Error: {}", e),
    }

    // upload_multipart by piping a stream from a URL directly (no buffering)

    let download =
        reqwest::get("https://images.pexels.com/photos/36851963/pexels-photo-36851963.jpeg")
            .await
            .expect("Failed to fetch URL");

    let source = UploadSource::from_stream(
        reqwest::Body::wrap_stream(download.bytes_stream()),
        "some-image-from-pexels.jpg",
    );

    let result = client
        .resource()
        .upload_multipart(UploadMultipartRequest::new(1228, false, false), source)
        .await;

    match result {
        Ok(response) => println!("{:#?}", response),
        Err(e) => println!("Error: {}", e),
    }

    // or just use upload_file_by_url

    let result = client
        .resource()
        .upload_file_by_url(
            UploadFileByUrlRequest::new(1228)
                .url("https://images.pexels.com/photos/20063016/pexels-photo-20063016.jpeg"),
        )
        .await;

    match result {
        Ok(response) => println!("{:#?}", response),
        Err(e) => println!("Error: {}", e),
    }
}
