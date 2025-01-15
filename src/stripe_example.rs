use async_stripe::{Client, CreatePaymentIntent, Currency};

#[tokio::main]
async fn main() {
    let client = Client::new("sk_test_api_key".to_string());
    let payment_intent = CreatePaymentIntent {
        amount: 2000,
        currency: Currency::USD,
        ..Default::default()
    };

    match client.create_payment_intent(payment_intent).await {
        Ok(intent) => println!("Payment Intent Created: {:?}", intent),
        Err(err) => eprintln!("Failed to create payment intent: {}", err),
    }
}

pub async fn run_stripe_example() {
    let client = Client::new("sk_test_api_key".to_string()); // Replace with your test API key
    let payment_intent = CreatePaymentIntent {
        amount: 2000,
        currency: Currency::USD,
        ..Default::default()
    };

    match client.create_payment_intent(payment_intent).await {
        Ok(intent) => println!("Payment Intent Created: {:?}", intent),
        Err(err) => eprintln!("Failed to create payment intent: {}", err),
    }
}
