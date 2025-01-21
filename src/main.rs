mod decision_engine;
mod payments;
mod stripe_example;

use decision_engine::process_payment_request;
use payments::PaymentRequest;
use stripe_example::run_stripe_example;
use mcp_rust_schema::protocol::{Request, RequestId};
use mcp_rust_schema::types::RequestMeta;
use mcp_rust_schema::types::Resource;

#[tokio::main]
async fn main() {
    // Example payment request
    let payment_request = PaymentRequest {
        amount: 250.50,
        currency: "USD".to_string(),
        purpose: "Subscription Payment".to_string(),
        user_id: "user123".to_string(),
    };

    println!("Processing payment: {:?}", payment_request);

    let response = process_payment_request(payment_request).await;
    match response {
        Ok(message) => println!("Payment Successful: {}", message),
        Err(error) => eprintln!("Payment Failed: {}", error),
    }

    // Call the Stripe example function if needed
    run_stripe_example().await;
}async fn payment_stripe() {
    run_stripe_example().await;
}