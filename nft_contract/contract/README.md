# Hello NEAR Contract

The smart contract exposes two methods to enable storing and retrieving a greeting in the NEAR network.

```rust
const DEFAULT_GREETING: &str = "Hello";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    greeting: String,
}

impl Default for Contract {
    fn default() -> Self {
        Self{greeting: DEFAULT_GREETING.to_string()}
    }
}

#[near_bindgen]
impl Contract {
    // Public: Returns the stored greeting, defaulting to 'Hello'
    pub fn get_greeting(&self) -> String {
        return self.greeting.clone();
    }

    // Public: Takes a greeting, such as 'howdy', and records it
    pub fn set_greeting(&mut self, greeting: String) {
        // Record a log permanently to the blockchain!
        log!("Saving greeting {}", greeting);
        self.greeting = greeting;
    }
}
```

<br />

# Quickstart

1. Make sure you have installed [rust](https://rust.org/).
2. Install the [`NEAR CLI`](https://github.com/near/near-cli#setup)

<br />

## 1. Build and Deploy the Contract

You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
./deploy.sh
```

Once finished, check the `neardev/dev-account` file to find the address in which the contract was deployed:

```bash
cat ./neardev/dev-account
# e.g. dev-1659899566943-21539992274727
```

<br />
<!-- 1st -->
export NFT_CONTRACT_ID="aaa.testnet"
export OTHER_NFT_CONTRACT_ID="bbb.testnet"

echo $NFT_CONTRACT_ID

near deploy --wasmFile contract/target/wasm32-unknown-unknown/release/nft_contract.wasm --accountId $NFT_CONTRACT_ID

near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID

near call $NFT_CONTRACT_ID update_metadata '{"metadata": {"spec": "nft-1.0.0", "name": "NFT Tutorial Contract", "symbol": "NFTTC"}}' --accountId $NFT_CONTRACT_ID

near call $NFT_CONTRACT_ID nft_metadata '{}' --accountId $NFT_CONTRACT_ID

near call $NFT_CONTRACT_ID nft_mint '{"token_id": "tk-1", "metadata": {"title": "Dreamy Nights", "description": "Testing the transfer call function", "media": "https://gateway.pinata.cloud/ipfs/QmekQYu4pgFWhk43E1bQowABCPUWhvZQ3qdjK9H7FeDUpJ"}, "receiver_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID --amount 0.1

near view $NFT_CONTRACT_ID nft_tokens '{}'

near view $NFT_CONTRACT_ID nft_token '{"token_id": "tk-1"}'

near view $NFT_CONTRACT_ID nft_tokens_for_owner '{"account_id": "'$NFT_CONTRACT_ID'"}'

near view $NFT_CONTRACT_ID nft_supply_for_owner '{"account_id": "'$NFT_CONTRACT_ID'"}'

near call $NFT_CONTRACT_ID nft_transfer '{"receiver_id": "'$OTHER_NFT_CONTRACT_ID'", "token_id": "tk-1", "msg": "foo"}' --accountId $NFT_CONTRACT_ID --depositYocto 1 --gas 200000000000000

near call $NFT_CONTRACT_ID nft_transfer_call '{"receiver_id": "'$OTHER_NFT_CONTRACT_ID'", "token_id": "tk-1", "msg": "foo"}' --accountId $NFT_CONTRACT_ID --depositYocto 1 --gas 200000000000000

## 2. Retrieve the Greeting

`get_greeting` is a read-only method (aka `view` method).

`View` methods can be called for **free** by anyone, even people **without a NEAR account**!

```bash
# Use near-cli to get the greeting
near view <dev-account> get_greeting
```

<br />

## 3. Store a New Greeting

`set_greeting` changes the contract's state, for which it is a `change` method.

`Change` methods can only be invoked using a NEAR account, since the account needs to pay GAS for the transaction.

```bash
# Use near-cli to set a new greeting
near call <dev-account> set_greeting '{"greeting":"howdy"}' --accountId <dev-account>
```

**Tip:** If you would like to call `set_greeting` using your own account, first login into NEAR using:

```bash
# Use near-cli to login your NEAR account
near login
```

and then use the logged account to sign the transaction: `--accountId <your-account>`.
