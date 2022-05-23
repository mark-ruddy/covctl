# covalent_class_a
Rust wrapper for the Covalent Class A Endpoints: https://www.covalenthq.com/docs/api  

## Klaytn Client Example
Set your API key as an environment variable: `export COVALENT_API_KEY=<YOUR_API_KEY>`, or you can use the `CovalentClient::new("8217", "<YOUR_API_KEY>")` function to pass the API key in as a code parameter  

In the below example a client is created which will use the Klaytn Mainnet(8217) and a unified covalent API call is made to the Get Token Balances endpoint:
```
let klaytn_client = covalent_class_a::CovalentClient::new_env_api_key("8217").unwrap();
let balances: covalent_class_a::resources::BalancesData = klaytn_client.get_token_balances("0xf4024faad5fafd0755875e3161524c9c4e1a1111", None, None).await.unwrap();
println!("Address: {}", balances.data.address);
```

For more examples see the docs: https://docs.rs/covalent_class_a/0.1.1/covalent_class_a/struct.CovalentClient.html 

## CLI Interface
See `covctl` at https://github.com/mark-ruddy/covctl for the CLI interface to this library

## API Coverage
### Implemented
- Balances Endpoints
- Transaction Endpoints
- Base Endpoints

### Not Implemented
- NFT Endpoints

## Testing
The tests will send actual API calls to the covalent API. They require an API key to be set in the environment variable COVALENT_API_KEY:
```
export COVALENT_API_KEY=<YOUR_API_KEY>
cargo test
```

The tests cover that the API calls for each library method return successfully and in some tests check for specific expected data.
