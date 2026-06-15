use resourcespace_client::Client;

async fn new_rs_client() -> Client {
    let base_url = std::env::var("RS_BASE_URL").expect("RS_BASE_URL not set");
    let user = std::env::var("RS_USER").expect("RS_USER not set");
    let key = std::env::var("RS_KEY").expect("RS_KEY not set");
    Client::builder()
        .base_url(&base_url)
        .user_key(&user, &key)
        .build()
        .await
        .expect("failed to build client")
}

#[tokio::test]
#[ignore = "requires live RS API with env vars set"]
async fn test_update_field_text_with_comma() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_path("tests/.env").ok();
    let client = new_rs_client().await;

    let resource_id = 1;
    let field = "title";

    let result = client
        .metadata()
        .update_field(
            resourcespace_client::api::metadata::UpdateFieldRequest::new(
                resource_id,
                field,
                resourcespace_client::api::FieldValue::from("Testing, comma in text field"),
            ),
        )
        .await;

    dbg!(result?);
    Ok(())
}

#[tokio::test]
#[ignore = "requires live RS API with env vars set"]
async fn test_update_field_keyword_with_comma() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_path("tests/.env").ok();
    let client = new_rs_client().await;

    let resource_id = 1226;
    let field = "tags";

    let result = client
        .metadata()
        .update_field(
            resourcespace_client::api::metadata::UpdateFieldRequest::new(
                resource_id,
                field,
                resourcespace_client::api::FieldValue::from(["Doe, John", "Smith, Jane"]),
            ),
        )
        .await;

    dbg!(result?);
    Ok(())
}

#[tokio::test]
#[ignore = "requires live RS API with env vars set"]
async fn test_create_resource_with_keyword_metadata() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_path("tests/.env").ok();
    let client = new_rs_client().await;

    let resource_type = 1;
    let text_field_id = 8;
    let keyword_field_id = 97;

    let result = client
        .resource()
        .create_resource(
            resourcespace_client::api::resource::CreateResourceRequest::new(resource_type)
                .metadata(std::collections::HashMap::from([
                    (
                        text_field_id,
                        resourcespace_client::api::FieldValue::from("Comma, testing"),
                    ),
                    (
                        keyword_field_id,
                        resourcespace_client::api::FieldValue::from(["Doe, John", "Smith, Jane"]),
                    ),
                ])),
        )
        .await;

    dbg!(result?);
    Ok(())
}
