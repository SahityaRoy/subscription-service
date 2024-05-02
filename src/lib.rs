use soroban_sdk::{contractimpl, contracttype, Bytes, Env, IntoVal, Symbol, Vec};

// Structure to store a single subscription
#[contracttype]
pub struct Subscription {
    user: Bytes,
    service_id: Symbol,
    start_time: u64,
    end_time: u64,
}

// The contract type
#[contracttype]
pub enum DataKey {
    Subscription(Bytes), // Store subscriptions by user address
}

pub trait SubscriptionContract {
    #[contractfn]
    fn create_subscription(
        env: Env,
        user: Bytes,
        service_id: Symbol,
        duration: u64, // Duration in seconds
    );

    #[contractfn]
    fn is_subscription_active(env: Env, user: Bytes, service_id: Symbol) -> bool;

    // ... Add more functions like:
    //  - renew_subscription(env: Env, ...)
    //  - cancel_subscription(env: Env, ...)
}

// Implementation of the Contract trait
contractimpl! {
    fn create_subscription(env: Env, user: Bytes, service_id: Symbol, duration: u64) {
        let start_time = env.ledger().timestamp();
        let end_time = start_time + duration;

        let subscription = Subscription {
            user: user.clone(),
            service_id,
            start_time,
            end_time,
        };

        env.data().set(DataKey::Subscription(user), subscription); // Store subscription
    }

    fn is_subscription_active(env: Env, user: Bytes, service_id: Symbol) -> bool {
        let subscription: Option<Subscription> = 
            env.data().get(DataKey::Subscription(user.clone()))?;

        match subscription {
            Some(sub) => {
                let current_time = env.ledger().timestamp();
                sub.service_id == service_id && sub.start_time <= current_time && current_time <= sub.end_time
            },
            None => false,
        }
    }
}
