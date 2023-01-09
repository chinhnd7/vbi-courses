use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTContract {
    owner_id: String,
    prefix: String,
    tokens: UnorderedMap<usize, TokenInfo>,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenInfo {
    pub token_id: usize,
    pub token_owner_id: String,
    pub name: String,
    pub description: String,
    pub media_uri: String,
    pub level: u128,
}

impl Default for NFTContract {
    fn default() -> Self {
        Self {
            owner_id: "junkyu.testnet".to_string(),
            prefix: "owner".to_string(),
            tokens: UnorderedMap::new(b"tokens".to_vec()),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl NFTContract {
    pub fn mint_nft(
        &mut self,
        token_owner_id: String,
        name: String,
        description: String,
        media_uri: String,
        level: u128,
    ) -> TokenInfo {
        let tokenInfo = TokenInfo {
            token_id: self.tokens.len() as usize,
            token_owner_id,
            name,
            description,
            media_uri,
            level,
        };

        self.tokens.insert(&tokenInfo.token_id, &tokenInfo);
        tokenInfo
    }

    pub fn get_token_by_id(&self, token_id: usize) -> TokenInfo {
        self.tokens.get(&token_id).unwrap().clone()
    }

    pub fn get_total_token(&self) -> usize {
        self.tokens.len() as usize
    }
}
