use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, log, near_bindgen, AccountId, Balance, BlockHeight, EpochHeight, Gas,
    PanicOnDefault, Promise, PromiseOrValue, PromiseResult, Timestamp, promise_result_as_success
};

use crate::core::*;
use crate::course::*;
use crate::enumeration::*;
use crate::internal::*;
use crate::utils::*;
use crate::ft_callback::*;
use crate::nft_callback::*;

mod core;
mod course;
mod enumeration;
mod internal;
mod utils;
mod ft_callback;
mod nft_callback;

pub type CourseId = u128;

pub const ONE_YOCTO: u128 = 1_000_000_000_000_000_000_000_000;
pub const LEVEL_1: u128 = 5;
pub const LEVEL_2: u128 = 10;
pub const LEVEL_3: u128 = 15;
pub const LEVEL_4: u128 = 20;
pub const LEVEL_5: u128 = 25;
pub const LEVEL_6: u128 = 30;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub lng_fungible_contract_id: AccountId,
    pub lne_fungible_contract_id: AccountId,
    pub courses_by_user: LookupMap<AccountId, UnorderedSet<Course>>,
    pub courses_by_contributor: LookupMap<AccountId, UnorderedSet<Course>>,
    pub course_metadata_by_id: UnorderedMap<CourseId, CourseMetadata>,
    pub courses: UnorderedSet<Course>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        lng_fungible_contract_id: AccountId,
        lne_fungible_contract_id: AccountId,
    ) -> Self {
        Self {
            owner_id,
            lng_fungible_contract_id,
            lne_fungible_contract_id,
            courses_by_user: LookupMap::new(b'a'),
            courses_by_contributor: LookupMap::new(b'b'),
            course_metadata_by_id: UnorderedMap::new(b'c'),
            courses: UnorderedSet::new(b'd'),
        }
    }

    #[init]
    pub fn new_default(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            "ft-lng.l2e.testnet".to_string().try_into().unwrap(),
            "ft-lne.l2e.testnet".to_string().try_into().unwrap(),
        )
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
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    const CREATE_COURSE_DEPOSIT: u128 = 12420000000000000000000;
    const LEVEL1_PRICE: u128 = LEVEL_1 * ONE_YOCTO;
    const LEVEL2_PRICE: u128 = LEVEL_2 * ONE_YOCTO;

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    fn get_default_box() -> CustomBox {
        let card = Card {
            id: 1,
            question: "What is Rust?".to_string(),
            answer: "Programming Language".to_string(),
        };
        let cards = vec![card.clone(), card.clone()];
        CustomBox { cards }
    }

    fn get_default_metadata() -> CourseMetadata {
        CourseMetadata {
            name: "Rust".to_string(),
            level: 2,
            luck: 1,
            start_time: 100000,
            end_time: 120000,
            current_date: 1,
            course_type_id: 1,
            boxes: vec![get_default_box()],
        }
    }

    #[test]
    fn test_init_contract() {
        let context = get_context(false);
        testing_env!(context.build());

        let contract = Contract::new(accounts(1), accounts(2), accounts(3));

        assert_eq!(contract.owner_id, accounts(1));
        assert_eq!(contract.lng_fungible_contract_id, accounts(2));
        assert_eq!(contract.lne_fungible_contract_id, accounts(3));
        assert_eq!(contract.total_courses_count(), U128(0));
    }

    #[test]
    fn test_default_init_contract() {
        let context = get_context(false);
        testing_env!(context.build());

        let contract = Contract::new_default("main.l2e.testnet".to_string().try_into().unwrap());

        assert_eq!(
            contract.owner_id,
            "main.l2e.testnet".to_string().try_into().unwrap()
        );
        assert_eq!(
            contract.lng_fungible_contract_id,
            "ft-lng.l2e.testnet".to_string().try_into().unwrap()
        );
        assert_eq!(
            contract.lne_fungible_contract_id,
            "ft-lne.l2e.testnet".to_string().try_into().unwrap()
        )
    }

    #[test]
    fn test_create_course() {
        let mut context = get_context(false);
        testing_env!(context.build());

        let mut contract =
            Contract::new_default("main.l2e.testnet".to_string().try_into().unwrap());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(CREATE_COURSE_DEPOSIT)
            .build());

        let course = contract.create_course(get_default_metadata());
        assert_eq!(contract.total_courses_count(), U128(1));
        assert_eq!(contract.total_courses_for_contributor(accounts(0)), U128(1));
        assert_eq!(contract.total_courses_for_user(accounts(0)), U128(0));
        assert_eq!(contract.total_courses_for_user(accounts(1)), U128(0));
    }

    #[test]
    fn test_register_course() {
        let mut context = get_context(false);
        testing_env!(context.build());

        let mut contract =
            Contract::new_default("main.l2e.testnet".to_string().try_into().unwrap());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(LEVEL2_PRICE + CREATE_COURSE_DEPOSIT)
            .predecessor_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .build());

        contract.create_course(get_default_metadata());
        assert_eq!(contract.total_courses_count(), U128(1));
        assert_eq!(contract.total_courses_for_contributor(accounts(0)), U128(1));
        contract.register_course(accounts(0), u128::from(U128(1)));
        assert_eq!(contract.total_courses_for_user(accounts(0)), U128(1));
        assert_eq!(contract.total_courses_for_user(accounts(1)), U128(0));
        // print!("{:?}", contract.courses_for_user(accounts(0), Some(U128(0)), Some(10)));
    }
}
