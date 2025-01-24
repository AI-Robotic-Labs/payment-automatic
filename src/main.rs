// Import necessary modules
mod decision_engine;
mod payments;
mod stripe_example;

use crate::decision_engine::process_payment;
use decision_engine::PaymentRequest;

use mcp_rust_schema::protocol::{Request, RequestId};
use mcp_rust_schema::types::{RequestMeta, Resource, ResourceContents};
use stripe_example::run_stripe_example;

#[tokio::main]
async fn main() {
    // Create an example payment request
    let payment_request = PaymentRequest {
        user_id: 12345, // Ensure `user_id` is a u32
        amount: 250.50,
        currency: "USD".to_string(),
        purpose: "Subscription Payment".to_string(),
    };

    // Create a Resource object
    let resource = Resource {
        uri: "12345".to_string(),
        title: "payment".to_string(),
        description: Some("Request".to_string()),
        contents: ResourceContents::Text(serde_json::to_string(&payment_request).unwrap()),
        mime_type: Some("application/json".to_string()),
        annotations: None,
    };

    // Create a Request ID
    let request_id = RequestId::Number(12345);

    // Create a RequestMeta object
    let request_meta = RequestMeta {
        progress_token: None, // Use None instead of String::new()
    };

    // Create a Request object
    let request = Request {
        meta: Some(request_meta),
        jsonrpc: "2.0".to_string(),
        method: "process_payment".to_string(),
        params: Some(serde_json::to_value(payment_request.clone()).unwrap()), // Clone because it will be used below
        id: RequestId::Number(1),
    };

    println!("Processing payment: {:?}", payment_request);

    // Process the payment request
    let response = process_payment(payment_request).await;
    match response {
        Ok(message) => println!("Payment Successful: {}", message),
        Err(error) => eprintln!("Payment Failed: {}", error),
    }

    // Optionally call the Stripe example function
    run_stripe_example();
}// Function to execute the Stripe exampleasync fn payment_stripe() {
pub async fn execute_stripe_example() {
    println!("Running Stripe example...");
    Box::pin(stripe_example::run_stripe_example()).await;
    println!("Stripe example completed.");
}