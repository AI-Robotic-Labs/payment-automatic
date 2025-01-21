use crate::decision_engine::{process_payment, PaymentRequest};

/// Simulates a decision engine that processes payment requests.
///
/// # Arguments
/// 
/// * `payment` - A `PaymentRequest` object containing payment details.
///
/// # Returns
/// 
/// * `Ok(String)` - If the payment is approved and processed successfully.
/// * `Err(String)` - If the payment requires manual approval or is rejected.
pub async fn process_payment_request(payment: PaymentRequest) -> Result<String, String> {
    // Simulate decision-making based on the payment details
    let decision = if payment.purpose.contains("Subscription") {
        "APPROVE"
    } else if payment.amount > 1000.0 {
        "REQUIRE_APPROVAL"
    } else {
        "APPROVE"
    };

    // Decision-based flow
    match decision {
        "APPROVE" => {
            // Proceed with payment processing
            process_payment(payment).await
        }
        "REQUIRE_APPROVAL" => {
            // Return an error for manual approval
            Err("Payment requires manual approval".to_string())
        }
        _ => {
            // Return a policy rejection error
            Err("Payment rejected due to policy".to_string())
        }
    }
}


