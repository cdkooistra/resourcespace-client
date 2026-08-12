mod common;

#[tokio::test]
async fn test_update_field_text_with_comma() -> Result<(), Box<dyn std::error::Error>> {
    let client = live_client_or_skip!();

    let resource_id = common::seed(&client).await.resource;
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
async fn test_update_field_keyword_with_comma() -> Result<(), Box<dyn std::error::Error>> {
    let client = live_client_or_skip!();

    let resource_id = common::seed(&client).await.resource;

    // Type 9 is a dynamic keywords list. Field refs and shortnames differ
    // between installs, so ask the instance instead of hardcoding one.
    let fields = client
        .metadata()
        .get_resource_type_fields(
            resourcespace_client::api::metadata::GetResourceTypeFieldsRequest::new()
                .field_type_ids([9]),
        )
        .await?;
    let field = fields.first().expect("a keywords field").name.clone();

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
async fn test_create_resource_with_keyword_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let client = live_client_or_skip!();

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
