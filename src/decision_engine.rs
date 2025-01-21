use serde::{Deserialize, Serialize};
use reqwest;

/// Represents a payment request.
///
/// This structure is serializable and deserializable with `Serde` for easy conversion to/from JSON.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub user_id: u32,
    pub amount: f64,
    pub currency: String,
    pub purpose: String,
}

/// Simulates processing of a payment via an external API or logic.
///
/// # Arguments
/// 
/// * `payment` - A `PaymentRequest` object that contains the payment details.
///
/// # Returns
/// 
/// * `Ok(String)` - If the payment is processed successfully.
/// * `Err(String)` - If the payment fails due to network issues or API errors.
pub async fn process_payment(payment: PaymentRequest) -> Result<String, String> {
    // Endpoint for the external payment processing API
    let payment_api_url = "https://api.example.com/process_payment";

    // Create an HTTP client
    let client = reqwest::Client::new();

    // Make a POST request with the payment data
    let response = client
        .post(payment_api_url)
        .json(&payment) // Serialize the `PaymentRequest` to JSON
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    // Handle the API response
    if response.status().is_success() {
        let resp_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        // Extract the "message" field or default to "Success"
        Ok(resp_json["message"].as_str().unwrap_or("Success").to_string())
    } else {
        Err(format!(
            "Payment API responded with status {}",
            response.status()
        ))
    }
}
