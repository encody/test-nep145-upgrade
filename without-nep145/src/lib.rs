use near_contract_standards::{fungible_token::FungibleToken, impl_fungible_token_core};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    near_bindgen, AccountId, PanicOnDefault, PromiseOrValue,
};

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
#[near_bindgen]
pub struct Contract {
    token: FungibleToken,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            token: FungibleToken::new(b"t"),
        }
    }

    pub fn mint(&mut self, amount: U128) {
        let account = env::predecessor_account_id();
        if !self.token.accounts.contains_key(&account) {
            self.token.internal_register_account(&account);
        }
        self.token.internal_deposit(&account, amount.into());
    }
}

impl_fungible_token_core!(Contract, token);
