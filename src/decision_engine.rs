use crate::payments::{process_payment, PaymentRequest};

/// Simulates a decision engine that processes payment requests.
pub async fn process_payment_request(payment: PaymentRequest) -> Result<String, String> {
    // Simulate decision-making by analyzing the payment purpose
    let decision = if payment.purpose.contains("Subscription") {
        "APPROVE"
    } else if payment.amount > 1000.0 {
        "REQUIRE_APPROVAL"
    } else {
        "APPROVE"
    };

    match decision {
        "APPROVE" => {
            process_payment(payment).await
        }
        "REQUIRE_APPROVAL" => Err("Payment requires manual approval".to_string()),
        _ => Err("Payment rejected due to policy".to_string()),
    }
}
