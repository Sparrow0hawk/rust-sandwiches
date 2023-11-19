mod fixtures;

#[actix_web::test]
async fn test_index() {
    let app = fixtures::spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    let page_str = &resp.text().await.unwrap();

    let index_page = fixtures::IndexPage::new(page_str);

    assert_eq!(index_page.get_header(), "Rust Sandwiches")
}
