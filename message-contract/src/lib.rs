use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PromiseOrValue};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new() -> Self {
        let this = Self {
        };
        this
    }

    pub fn get_welcome_message(&self) -> PromiseOrValue<String> {
        let processor = env::predecessor_account_id();
        let account_id = env::signer_account_id();
        assert_ne!(processor, account_id.clone());
        let welcome_message = format!("Hello {}!", account_id);
        PromiseOrValue::Value(welcome_message)
    }
}