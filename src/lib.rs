use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, Timestamp, EpochHeight, BlockHeight, AccountId, Balance, ext_contract, log, Gas, PromiseResult, PromiseOrValue, PanicOnDefault};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::collections::{Vector, LookupMap};

use crate::course::*;

mod course;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
    CourseUserKey,
    CourseContributorKey
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub lng_fungible_contract_id: AccountId,
    pub lne_fungible_contract_id: AccountId,
    pub courses_by_user: LookupMap<AccountId, Vector<Course>>,
    pub courses_by_contributor: LookupMap<AccountId, Vector<Course>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new (owner_id: AccountId, lng_fungible_contract_id: AccountId, lne_fungible_contract_id: AccountId) -> Self {
        Self {
            owner_id,
            lng_fungible_contract_id,
            lne_fungible_contract_id,
            courses_by_user: LookupMap::new(StorageKey::CourseUserKey.try_to_vec().unwrap()),
            courses_by_contributor: LookupMap::new(StorageKey::CourseContributorKey.try_to_vec().unwrap())
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::{testing_env, MockedBlockchain};
    use near_sdk::test_utils::{VMContextBuilder, accounts};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id(accounts(0))
        .signer_account_id(accounts(0))
        .predecessor_account_id(accounts(0))
        .is_view(is_view);

        builder
    }

    #[test]
    fn test_init_contract() {
        let context = get_context(false);
        testing_env!(context.build());

        let contract = Contract::new(accounts(1), accounts(2), accounts(3));

        assert_eq!(contract.owner_id, accounts(1));
        assert_eq!(contract.lng_fungible_contract_id, accounts(2));
        assert_eq!(contract.lne_fungible_contract_id, accounts(3));
    }
}
