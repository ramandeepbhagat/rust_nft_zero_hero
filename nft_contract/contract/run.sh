npm run build

export NFT_CONTRACT_ID="npwa.testnet"
export RECIEVER_NFT_CONTRACT_ID="npwb.testnet"

echo $NFT_CONTRACT_ID
echo $RECIEVER_NFT_CONTRACT_ID

near deploy --wasmFile contract/target/wasm32-unknown-unknown/release/nft_contract.wasm --accountId $NFT_CONTRACT_ID

near call $NFT_CONTRACT_ID new_default_meta '{"owner_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID

near call $NFT_CONTRACT_ID update_metadata '{"metadata": {"spec": "nft-1.0.0", "name": "NFT Tutorial Contract", "symbol": "NFTTC"}}' --accountId $NFT_CONTRACT_ID

# near call $NFT_CONTRACT_ID nft_metadata '{}' --accountId $NFT_CONTRACT_ID

near call $NFT_CONTRACT_ID nft_mint '{"token_id": "tk-1", "metadata": {"title": "Token-1", "description": "Testing the transfer call function", "media": "https://gateway.pinata.cloud/ipfs/QmekQYu4pgFWhk43E1bQowABCPUWhvZQ3qdjK9H7FeDUpJ"}, "receiver_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID --amount 0.1

# near view $NFT_CONTRACT_ID nft_tokens '{}'

near view $NFT_CONTRACT_ID nft_token '{"token_id": "tk-1"}'

# near view $NFT_CONTRACT_ID nft_tokens_for_owner '{"account_id": "'$NFT_CONTRACT_ID'"}'

# near view $NFT_CONTRACT_ID nft_supply_for_owner '{"account_id": "'$NFT_CONTRACT_ID'"}'

# near call $NFT_CONTRACT_ID nft_transfer '{"receiver_id": "'$RECIEVER_NFT_CONTRACT_ID'", "token_id": "tk-1", "msg": "foo"}' --accountId $NFT_CONTRACT_ID --depositYocto 1 --gas 200000000000000

# Failure [npwa.testnet]: Error: Cannot find contract code for account npwb.testnet

near call $NFT_CONTRACT_ID nft_transfer_call '{"receiver_id": "'$RECIEVER_NFT_CONTRACT_ID'", "token_id": "tk-1", "msg": "keep-it-now"}' --accountId $NFT_CONTRACT_ID --depositYocto 1 --gas 200000000000000
