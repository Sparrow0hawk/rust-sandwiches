mod fixtures;

#[actix_web::test]
async fn test_page_not_found() {
    let app = fixtures::spawn_app().await;

    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/foo", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    let status = resp.status().clone();

    let page_str = &resp.text().await.unwrap();

    let index_page = fixtures::IndexPage::new(page_str);

    assert_eq!(index_page.get_header(), "Page not found");
    assert_eq!(status, 404)
}
