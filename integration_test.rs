use actix_web::{test, App};
use axonix_ai::routes::register_routes;

#[actix_web::test]
async fn test_analyze_get() {
    let app = test::init_service(App::new().configure(register_routes)).await;
    let req = test::TestRequest::get().uri("/analyze").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_analyze_post() {
    let app = test::init_service(App::new().configure(register_routes)).await;
    let req = test::TestRequest::post()
        .uri("/analyze")
        .set_json(&serde_json::json!({
            "symbol": "ETH/USD",
            "historical_prices": [1500.0, 1510.0, 1505.0, 1515.0]
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_signal_endpoint() {
    let app = test::init_service(App::new().configure(register_routes)).await;
    let req = test::TestRequest::get().uri("/signal").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
