use resourcespace_client::Client;
use resourcespace_client::api::SortOrder;
use resourcespace_client::api::search::DoSearchRequest;
use resourcespace_client::api::system::GetDailyStatSummaryRequest;
use resourcespace_client::api::message::GetUserMessageRequest;

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

    let search_result = client.search()
        .do_search(DoSearchRequest::new("909").sort(SortOrder::Asc))
        .await;

    match search_result {
        Ok(response) => println!("{:#?}", response),
        Err(e) => println!("Error: {}", e),
    }

    let system_result = client.system()
        .get_daily_stat_summary(GetDailyStatSummaryRequest::new().days(31))
        .await;

    match system_result {
        Ok(response) => println!("{:#?}", response),
        Err(e) => println!("Error: {}", e),
    }

    // let message_result = client.message()
    //     .get_user_message(GetUserMessageRequest::new(12))
    //     .await;

    // match message_result {
    //     Ok(response) => println!("{:#?}", response),
    //     Err(e) => println!("Error: {}", e),
    // }

}
