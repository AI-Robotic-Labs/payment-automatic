use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub user_id: u32,
    pub amount: f64,
    pub currency: String,
    pub purpose: String,
}

/// Simulates processing of payment via external API or logic.
pub async fn process_payment(payment: PaymentRequest) -> Result<String, String> {
    // Simulate an API call
    let payment_api_url = "https://api.example.com/process_payment";

    let client = reqwest::Client::new();
    let response = client
        .post(payment_api_url)
        .json(&payment)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if response.status().is_success() {
        let resp_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(resp_json["message"].as_str().unwrap_or("Success").to_string())
    } else {
        Err(format!(
            "Payment API responded with status {}",
            response.status()
        ))
    }
}
