mod decision_engine;
mod payments;
mod stripe_example;

use decision_engine::process_payment_request;
use payments::PaymentRequest;
use stripe_example::run_stripe_example;

#[tokio::main]
async fn main() {
    // Example payment request
    let payment_request = PaymentRequest {
        user_id: 42,
        amount: 250.50,
        currency: "USD".to_string(),
        purpose: "Subscription Payment".to_string(),
    };

    println!("Processing payment: {:?}", payment_request);

    let response = process_payment_request(payment_request).await;
    match response {
        Ok(message) => println!("Payment Successful: {}", message),
        Err(error) => eprintln!("Payment Failed: {}", error),
    }
}


#[tokio::main]
#[allow(dead_code)]
async fn payment_stripe() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        run_stripe_example();
    });
}