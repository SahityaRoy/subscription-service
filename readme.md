**Subscription Services Smart Contract**

**Description**

This Soroban smart contract enables the management of subscription-based services on the Stellar network, allowing users to pay in Stellar Lumens (XLM) or a custom token to access digital content or services for specified durations.

**Features**

* `create_subscription(user, service_id, duration)`
* `is_subscription_active(user, service_id)`
* (Add more features if you've implemented them)

**Prerequisites**

* Rust and Cargo
* Soroban CLI ([https://soroban.stellar.org/install](https://soroban.stellar.org/install))
* A Stellar account with funds for contract deployment and testing.
* Understanding of basic smart contract concepts.

**Setup**

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/](https://github.com/)<your-username>/subscription_contract.git
