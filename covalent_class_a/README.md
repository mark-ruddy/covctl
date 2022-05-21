# covalent_class_a
Rust wrapper for the Covalent Class A Endpoints: https://www.covalenthq.com/docs/api
TODO: add example project covctl

## Klaytn Client Example
```
let klaytn_client = covalent_class_a::CovalentClient::new("8127", "<YOUR_API_KEY>").unwrap();
let balances: covalent_class_a::resources::BalanceData = klaytn_client.get_token_balances("<KLAYTN_WALLET_ADDRESS>")
println!("Address: {}", balances.data.address)
```
