use crate::*;

#[ext_contract(ext_nft_receiver)]
trait NonFungibleTokenReceiver {
    //Method stored on the receiver contract that is called via cross contract call when nft_transfer_call is called
    /// Returns `true` if the token should be returned back to the sender.
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool>;

}

#[near_bindgen]
impl NonFungibleTokenReceiver for Contract {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        log!(
            "inside nft_on_transfer; predecessor_account_id={}, sender_id={}, previous_owner_id={}, token_id={}, msg={}",
            &env::predecessor_account_id(),
            &sender_id,
            &previous_owner_id,
            &token_id,
            &msg
        );

        match msg.as_str() {
            "return-it-now" => PromiseOrValue::Value(true),
            "return-it-later" => {
                // Call ok_go with no attached deposit and all unspent GAS (weight of 1)
                Self::ext(env::current_account_id())
                    .ok_go(true).into()
            }
            "keep-it-now" => PromiseOrValue::Value(false),
            "keep-it-later" => {
                // Call ok_go with no attached deposit and all unspent GAS (weight of 1)
                Self::ext(env::current_account_id())
                    .ok_go(false).into()
            }
            _ => env::panic_str("unsupported msg"),
        }
    }
}
