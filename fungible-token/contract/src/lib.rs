use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, log, near_bindgen, AccountId};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FungibleToken {
    user_accounts: LookupMap<AccountId, u128>,
    total_supply: u128,
}

// Define the default, which automatically initializes the contract
impl Default for FungibleToken {
    fn default() -> Self {
        let mut contract = FungibleToken {
            user_accounts: LookupMap::new(b'm'),
            total_supply: 100000,
        };
        let account_id = env::signer_account_id();
        contract
            .user_accounts
            .insert(&account_id, &contract.total_supply);
        return contract;
    }
}

// Implement the contract structure
#[near_bindgen]
impl FungibleToken {
    pub fn get_total_token(&self) -> u128 {
        self.total_supply.clone()
    }

    pub fn get_token_account(&self, account_id: AccountId) -> Option<u128> {
        self.user_accounts.get(&account_id)
    }
}
