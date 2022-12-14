/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet, LazyOption, UnorderedMap};
use near_sdk::{env, log, require, near_bindgen, PanicOnDefault, AccountId, CryptoHash, Balance, Promise, ext_contract, Gas,  PromiseResult, PromiseOrValue};
use near_sdk::{serde::{self, Serialize, Deserialize}, json_types::{Base64VecU8, U128}};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::token_reciever::*;
pub use crate::token_resolver::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;

mod internal;
mod approval;
mod enumeration;
mod metadata;
mod mint;
mod token_reciever;
mod token_resolver;
mod nft_core;
mod royalty;

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id.
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized.
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id.
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),

            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            /* tokens_per_owner: LookupMap::new(b"a"),
            tokens_by_id: LookupMap::new(b"c"),
            token_metadata_by_id: UnorderedMap::new(
                b"d"
            ),
            //set the owner_id field equal to the passed in owner_id.
            owner_id,
            metadata: LazyOption::new(
                b"e",
                Some(&metadata),
            ), */
        };

        //return the Contract object
        this
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "NFT Tutorial Contract".to_string(),
                symbol: "NFTTC".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    // pub fn update_metadata(&mut self, metadata: NFTContractMetadata) {
    //     self.update_nft_metadata(metadata);
    // }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

}
