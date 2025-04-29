use rand::Rng;

/// Analyzes historical prices and returns a signal and confidence.
pub fn analyze_prices(prices: &[f32]) -> (String, f32) {
    if prices.len() < 2 {
        return ("Hold".to_string(), 0.0);
    }

    let mut diffs: Vec<f32> = vec![];
    for i in 1..prices.len() {
        diffs.push(prices[i] - prices[i - 1]);
    }

    let avg_diff: f32 = diffs.iter().sum::<f32>() / diffs.len() as f32;
    let variance: f32 = diffs.iter().map(|d| (d - avg_diff).powi(2)).sum::<f32>() / diffs.len() as f32;
    let confidence = (avg_diff.abs() / (variance + 1.0)).min(1.0);

    let signal = if avg_diff > 0.0 {
        "Buy"
    } else if avg_diff < 0.0 {
        "Sell"
    } else {
        "Hold"
    };

    (signal.to_string(), confidence)
}

/// Generate a random signal for test/demo mode
pub fn generate_random_signal() -> String {
    let signals = vec!["Buy", "Sell", "Hold"];
    let idx = rand::thread_rng().gen_range(0..signals.len());
    signals[idx].to_string()
}

/// Simulated heavy analysis for benchmarking
pub fn simulate_heavy_analysis() -> f64 {
    let mut result = 0.0;
    for i in 0..10_000 {
        result += ((i as f64).sin().powi(2) + (i as f64).cos().powi(2)).sqrt();
    }
    result
}
