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
    fn callback_register_course(&mut self, account_id: AccountId, course_id: CourseId);
}

pub trait ExtSelf {
    fn callback_promise_result(&mut self) -> u64;
    fn callback_register_course(&mut self, account_id: AccountId, course_id: CourseId);
}

#[ext_contract(ext_nft)]
pub trait NonFungibleToken {
    fn nft_level_for_owner(&self, account_id: AccountId) -> u64;
}

#[near_bindgen]
impl Contract {
    pub fn nft_level_of_user(&self, account_id: AccountId) -> Promise {
        ext_nft::nft_level_for_owner(account_id, get_nft_contract(), 0, GAS_FOR_COMMON_OPERATIONS)
            .then(ext_self::callback_promise_result(
                env::current_account_id(),
                0,
                GAS_FOR_COMMON_OPERATIONS,
            ))
    }

    #[payable]
    pub fn register_course_for_user(&mut self, course_id: CourseId) -> Promise {
        let user_id = env::predecessor_account_id();
        let courses = self.courses.to_vec().clone();
        let course = self.courses.iter().find(|x| x.course_id == course_id);

        if let Some(course) = course {
            let level = course.metadata.level;
            let price_in_near = match level {
                1 => LEVEL_1,
                2 => LEVEL_2,
                3 => LEVEL_3,
                4 => LEVEL_4,
                5 => LEVEL_5,
                6 => LEVEL_6,
                _ => 0,
            };
            let price_in_yocto = u128::from(U128(price_in_near * ONE_YOCTO));
            let amount = env::attached_deposit();
            assert!(
                amount >= price_in_yocto,
                "Not enough NEAR to register this course. Price of this course: {} NEAR",
                price_in_near
            );
            if amount > price_in_yocto {
                let refund = amount - price_in_yocto;
                Promise::new(env::predecessor_account_id()).transfer(refund);
            }
            // self.internal_add_course_to_user(&user_id, &course);
        } else {
            panic!("Course doesn't exist!");
        }

        ext_nft::nft_level_for_owner(
            env::predecessor_account_id(),
            get_nft_contract(),
            0,
            GAS_FOR_COMMON_OPERATIONS,
        )
        .then(ext_self::callback_register_course(
            user_id,
            course_id,
            env::current_account_id(),
            0,
            GAS_FOR_COMMON_OPERATIONS,
        ))
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
            }
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        }
    }

    #[private]
    fn callback_register_course(&mut self, account_id: AccountId, course_id: CourseId) {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULS");
        let nft_level = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(level) = near_sdk::serde_json::from_slice::<u64>(&val) {
                    level
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            }
            PromiseResult::Failed => env::panic(b"ERR_CALL_FAILED"),
        };

        let user_id = account_id.clone();
        let courses = self.courses.to_vec().clone();
        let course = self.courses.iter().find(|x| x.course_id == course_id);
        let course = match course {
            Some(course) => course,
            None => panic!("Course doesn't exist!"),
        };
        let course_level = course.clone().metadata.level;
        let price_in_near = match course_level {
            1 => LEVEL_1,
            2 => LEVEL_2,
            3 => LEVEL_3,
            4 => LEVEL_4,
            5 => LEVEL_5,
            6 => LEVEL_6,
            _ => 0,
        };
        let price_in_yocto = u128::from(U128(price_in_near * ONE_YOCTO));

        match nft_level {
            0 => {
                Promise::new(account_id.clone()).transfer(price_in_yocto);
                panic!(
                    "{} must have Tutor NFT to register this course",
                    account_id.clone()
                );
            }
            1 | 2 | 3 | 4 | 5 | 6 => {
                self.internal_add_course_to_user(&user_id, &course);
            }
            _ => env::panic(b"ERR_CALL_FAILED"),
        }
    }
}
