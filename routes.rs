use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::{Analysis, AnalysisRequest};
use crate::utils::{analyze_prices, generate_random_signal};

#[get("/analyze")]
async fn analyze() -> impl Responder {
    let prices = vec![23000.0, 23200.0, 23150.0, 23300.0, 23400.0];
    let (signal, confidence) = analyze_prices(&prices);

    let result = Analysis {
        symbol: "BTC/USD".to_string(),
        signal,
        confidence,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    HttpResponse::Ok().json(result)
}

#[post("/analyze")]
async fn analyze_post(req: web::Json<AnalysisRequest>) -> impl Responder {
    let (signal, confidence) = analyze_prices(&req.historical_prices);

    HttpResponse::Ok().json(Analysis {
        symbol: req.symbol.clone(),
        signal,
        confidence,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[get("/signal")]
async fn random_signal() -> impl Responder {
    let result = generate_random_signal();
    HttpResponse::Ok().body(result)
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(analyze);
    cfg.service(analyze_post);
    cfg.service(random_signal);
}
