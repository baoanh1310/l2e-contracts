use crate::*;

pub const XCC_GAS: Gas = Gas(50_000_000_000_000 as u64);
pub const GAS_FOR_COMMON_OPERATIONS: Gas = Gas(3_000_000_000_000);
pub const GAS_RESERVED_FOR_CURRENT_CALL: Gas = Gas(20_000_000_000_000);

fn get_nft_contract() -> AccountId {
    AccountId::try_from("nft.l2e.testnet".to_string()).unwrap()
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn callback_promise_result(&mut self) -> u64;
}

pub trait ExtSelf {
    fn callback_promise_result(&mut self) -> u64;
}

#[ext_contract(ext_nft)]
pub trait NonFungibleToken {
    fn nft_level_for_owner(&self, account_id: AccountId) -> u64;
}

#[near_bindgen]
impl Contract {
    pub fn nft_level_of_user(&self, account_id: AccountId) -> Promise {
        ext_nft::nft_level_for_owner(account_id, get_nft_contract(), 0, GAS_FOR_COMMON_OPERATIONS).then(
            ext_self::callback_promise_result(env::current_account_id(), 0, GAS_FOR_COMMON_OPERATIONS),
        )

        // let call = ext_nft::nft_level_for_owner(env::predecessor_account_id(), get_nft_contract(), 0, GAS_FOR_COMMON_OPERATIONS);
        // let REMAINING_GAS: Gas = env::prepaid_gas() - env::used_gas() - GAS_FOR_COMMON_OPERATIONS - GAS_RESERVED_FOR_CURRENT_CALL;
        // let callback = ext_self::callback_promise_result(env::current_account_id(), 0, REMAINING_GAS);
        // call.then(callback)
    }
}

#[near_bindgen]
impl ExtSelf for Contract {
    #[private]
    fn callback_promise_result(&mut self) -> u64 {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULS");
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(level) = near_sdk::serde_json::from_slice::<u64>(&val) {
                    level
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            },
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }
}
