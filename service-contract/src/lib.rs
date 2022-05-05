use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, Promise, PromiseResult, Gas, AccountId};
use serde_json;

// define the methods we'll use on the other contract
#[ext_contract(ext_message)]
pub trait Receiver {
    fn get_welcome_message(&mut self) -> PromiseOrValue<String>;
}

// define methods we'll use as callbacks on our contract
#[ext_contract(ext_self)]
pub trait MyContract {
    fn my_callback(&self) -> String;
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new() -> Self {
        let this = Self {
        };
        this
    }

    pub fn say_hello(&self) -> Promise {
        let message_contract: AccountId = serde_json::from_str("\"message.manhng.testnet\"").unwrap();
        ext_message::get_welcome_message(
            message_contract,
            0,
            Gas(5_000_000_000_000)
        )
        .then(ext_self::my_callback(
            env::current_account_id(),
            0,
            Gas(5_000_000_000_000)
        ))
    }

    pub fn my_callback(&self) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "This is a callback method"
        );

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(result) => {
                let message = near_sdk::serde_json::from_slice::<String>(&result).unwrap();
                message
            },
        }
    }
}