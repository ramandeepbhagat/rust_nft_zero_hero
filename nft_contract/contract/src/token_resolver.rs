use crate::*;

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    /*
        resolves the promise of the cross contract call to the receiver contract
        this is stored on THIS contract and is meant to analyze what happened in the cross contract call when nft_on_transfer was called
        as part of the nft_transfer_call method
    */
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
    ) -> bool;
}

#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    // resolves the cross contract call when calling nft_on_transfer in the nft_transfer_call method
    // returns true if the token was successfully transferred to the receiver_id
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
    ) -> bool {
        // Whether receiver wants to return token back to the sender, based on `nft_on_transfer`
        // call result.
        if let PromiseResult::Successful(value) = env::promise_result(0) {
            // As per the standard, the nft_on_transfer should return whether we should return the token to it's owner or not
            if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
                // if we need don't need to return the token, we simply return true meaning everything went fine
                if !return_token {
                    /*
                        since we've already transferred the token and nft_on_transfer returned false, we don't have to
                        revert the original transfer and thus we can just return true since nothing went wrong.
                    */
                    return true;
                }
            }
        }

        // get the token object if there is some token object
        let mut token = if let Some(token) = self.tokens_by_id.get(&token_id) {
            if token.owner_id != receiver_id {
                // The token is not owned by the receiver anymore. Can't return it.
                return true;
            }
            token
        // if there isn't a token object, it was burned and so we return true
        } else {
            return true;
        };


        // if at the end, we haven't returned true, that means that we should return the token to it's original owner
        log!("Return {} from @{} to @{}", token_id, receiver_id, owner_id);

        // we remove the token from the receiver
        self.internal_remove_token_from_owner(&receiver_id, &token_id);

        // we add the token to the original owner
        self.internal_add_token_to_owner(&owner_id, &token_id);

        // we change the token struct's owner to be the original owner
        token.owner_id = owner_id;

        // we inset the token back into the tokens_by_id collection
        self.tokens_by_id.insert(&token_id, &token);

        // return false
        false
    }
}
