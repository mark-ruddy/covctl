# covctl
CLI tool to interact with the Covalent Unified API: https://www.covalenthq.com/docs/api  

`covctl` will return JSON for each API call. It provides a convenient CLI interface to easily make Unified Covalent API calls without having to build the API URLs manually

## Install
### Install with Cargo
```
cargo install covctl
```

### Build from source
```
git clone https://github.com/mark-ruddy/covctl.git
cd covctl/covctl/
cargo build --release
```

## Usage
First register and get an API key from: https://www.covalenthq.com/  

Then you can pass this key into each call e.g. `covctl --api-key=<YOUR_API_KEY>`  
Or you can omit the `--api-key` flag and set this environment variable: `export COVALENT_API_KEY=<YOUR_API_KEY>`  

See available API endpoints to query(`covctl` covers all Class A Endpoints except for NFTs): `covctl --help`  

## Examples

- `covctl transaction --help` - see the available flags for the transaction info API call, `--help` can be passed like this to any subcommand API call e.g. `covctl changes-in-token-holders --help`

- `covctl transaction --tx-hash 0x269fad968de5baf8d324b64d0a19df72ccfc762b33e1760729633f4946e0c863` - get back JSON on thistransaction

- `covctl token-balances --addr 0xf4024faad5fafd0755875e3161524c9c4e1a1111 | jq` - get back JSON for token balances of this address and pipe it through `jq`

- `covctl log-events-by-topic-hashes --topic-hash 0x804c9b842b2748a22bb64b345453a3de7ca54a6ca45ce00d415894979e22897a --sender-addr 0x7d2768dE32b0b80b7a3454c06BdAc94A69DDc7A9 --starting-block 12500000 --ending-block 12500100` - in this example the returned items JSON array is empty because there is no information for the parameters given
