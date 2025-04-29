use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Analysis {
    pub symbol: String,
    pub signal: String,
    pub confidence: f32,
    pub timestamp: String,
}

#[derive(Deserialize)]
pub struct AnalysisRequest {
    pub symbol: String,
    pub historical_prices: Vec<f32>,
}

#[derive(Serialize)]
pub struct SignalStat {
    pub signal_type: String,
    pub total_count: u32,
    pub avg_confidence: f32,
    pub max_confidence: f32,
}
