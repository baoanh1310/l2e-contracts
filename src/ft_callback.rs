use crate::*;

const GAS_FOR_FT_TRANSFER: Gas = Gas(15_000_000_000_000);

fn get_ft_contract() -> AccountId {
    AccountId::try_from("ft-lng.l2e.testnet".to_string()).unwrap()
}

#[ext_contract(ext_ft)]
pub trait FungibleTokenCore {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn callback_pay_reward_user(&mut self, account_id: AccountId, amount: U128);
}

pub trait ExtSelf {
    fn callback_pay_reward_user(&mut self, account_id: AccountId, amount: U128);
}

#[near_bindgen]
impl Contract {
    pub(crate) fn pay_reward_user(&self, account_id: AccountId, amount: U128) {
        ext_ft::ft_transfer(
            account_id.clone(),
            amount,
            None,
            get_ft_contract(),
            1,
            GAS_FOR_FT_TRANSFER,
        );
    }
}
