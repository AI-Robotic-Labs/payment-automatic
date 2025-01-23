// Import necessary modules
mod decision_engine;
mod payments;
mod stripe_example;

use decision_engine::process_payment_request; // Import the `process_payment_request` function
use crate::payments::PaymentRequest; // Import `PaymentRequest`
use stripe_example::run_stripe_example; // Import the `run_stripe_example` function

use mcp_rust_schema::protocol::{Request, RequestId};
use mcp_rust_schema::types::{RequestMeta, Resource};

#[tokio::main]
async fn main() {
    // Create an example payment request
    let payment_request = PaymentRequest {
        user_id: 12345, // Ensure `user_id` is a u32
        amount: 250.50,
        currency: "USD".to_string(),
        purpose: "Subscription Payment".to_string(),
    };

    println!("Processing payment: {:?}", payment_request);

    // Process the payment request
    let response = process_payment_request(payment_request).await; // Call the function from the decision_engine module
    match response {
        Ok(message) => println!("Payment Successful: {}", message),
        Err(error) => eprintln!("Payment Failed: {}", error),
    }

    // Optionally call the Stripe example function
    payment_stripe().await;
}

// Function to execute the Stripe example
async fn payment_stripe() {
    println!("Running Stripe example...");
    run_stripe_example().await;
}
    fn payment_stripe() {
        println!("Running Stripe example...");
        run_stripe_example();
    }  async fn payment_stripe() {
    println!("Running Stripe example...");
    run_stripe_example().await; // Add `.await` for asynchronous execution
  }

