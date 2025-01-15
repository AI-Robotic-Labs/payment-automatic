# Payment Automatic

AI Agent for payments automatic

## How works?

1. The `main.rs` initializes a `PaymentRequest` and sends it to the `process_payment_request` function. 

2. The `decision_engine.rs` contains the decision-making rules for payments.

3. The `payments.rs` handles the actual payment processing (using a mock API for simplicity)

## Example

```json
Processing payment: PaymentRequest { user_id: 42, amount: 250.5, currency: "USD", purpose: "Subscription Payment" }
```