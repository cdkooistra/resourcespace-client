use resourcespace_client::client::RsClient;

#[tokio::main]
async fn main() {
    dotenvy::from_path("examples/.env").ok();

    let base_url = std::env::var("RS_BASE_URL").expect("RS_BASE_URL not set");
    let user = std::env::var("RS_USER").expect("RS_USER not set");
    let password = std::env::var("RS_PASS").expect("RS_PASS not set");
    let key = std::env::var("RS_KEY").expect("RS_KEY not set");

    let client = RsClient::builder()
        .base_url(&base_url)
        .unwrap()
        // .user_key(&user, &key)
        .session_key(&user, &password)
        .build()
        .await
        .unwrap();

    let result = client.send_request("do_search", &[("param1", "909")]).await;

    match result {
        Ok(response) => println!("{:#?}", response),
        Err(e) => println!("Error: {}", e),
    }
}
