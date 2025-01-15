use stripe::{Client, CreatePaymentIntent, Currency};

#[tokio::main]
pub async fn run_stripe_example() {
    let _client = Client::new("sk_test_api_key".to_string());
    let _payment_intent = CreatePaymentIntent::new(2000, Currency::USD);

}
#[allow(dead_code)]
async fn run_example() {
    let _client = Client::new("sk_test_api_key".to_string()); // Replace with your test API key
    let _payment_intent = CreatePaymentIntent::new(2000, Currency::USD);

}