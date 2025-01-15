# Payment Automatic

AI Agent for payments automatic

## How works?

1. The `main.rs` initializes a `PaymentRequest` and sends it to the `process_payment_request` function. 

2. The `decision_engine.rs` contains the decision-making rules for payments.

3. The `payments.rs` handles the actual payment processing (using a mock API for simplicity)

## Use Cases

1. **Payment Request**: The AI agent receives a payment request from a user.
2. **Decision Engine**: The AI agent uses the decision engine to determine if the payment should be approved or denied.
3. **Payment Processing**: The AI agent processes the payment using the payment processing module.
4. **Response**: The AI agent sends a response to the user.

## How to Run
1. Clone the repository: `git clone https://github.com/AI-Robotic-Labs/payment-automatic.git`

2. Navigate to the project directory: `cd payment-automatic`

3. Build the project: `cargo build`

4. Run the project: `cargo run`


