# Klaytn-Covalent Unified Hackathon Report for covctl

## Links

- Video Demo - https://www.youtube.com/watch?v=JpHNQ9KZcjA
- Github Repo - https://github.com/mark-ruddy/covctl
- `covctl` CLI tool source code and `README.md` - https://github.com/mark-ruddy/covctl/tree/main/covctl
- `covalent_class_a` library source code and `README.md` - https://github.com/mark-ruddy/covctl/tree/main/covalent_class_a
- Crates.io for `covctl` - https://crates.io/crates/covctl
- Crates.io for `covalent_class_a` - https://crates.io/crates/covalent_class_a
- Developer documentation `covalent_class_a` - https://docs.rs/covalent_class_a/0.1.1/covalent_class_a/struct.CovalentClient.html

## Project Description
My initial plan for this hackathon project was very different to the hackathon project I ended up producing. The legacy of the original project remains in the `deprecated/` directory in the code repo.  

The original project would have been a webapp search tool for DeFi data within the Klaytn Mainnet - but I abandoned this project after realising I would not be able to produce a good-looking and dynamic frontend for it within the time-frame.  

I transitioned my project to something I knew might be really useful to both developers and users who interact with the Covalent APIs: An easy to use CLI tool to query the Covalent Class A endpoints called `covctl`, and an underlying Rust library which exposes all of the Covlanet Class A Endpoints(except NFTs currently) to Rust developers called `covalent_class_a`.  

- This project isn't necessarily related to DeFi - but it can be used for DeFi data
- This projects equivalent of a webapp frontend is the CLI tool `covctl` - I understand that this only appeals to advanced users or developers

## How to try out
Hosting a CLI app like this is different to a website - I was going to provide an EC2 server and give the SSH private key file here, but I don't want an unauthorised user accessing an EC2 under my control and possibly breaking it or running up the CPU for a long period of time.

if you want to try out the project its as easy as:
```
cargo install covctl
covctl --help
```

## What this project is useful for
### What covctl provides to developers
`covctl` as a solid debugging tool to easily make Covalent API calls, and `covalent_class_a` for Rust developers who want to hook into the Covalent Class A Endpoints.  

Having a client library available to interact with an API is a lot more approachable than implementing the direct HTTP API calls - more projects like `covalent_class_a` could drive uptake of the Covalent Unified APIs in general.  

### What covctl provides to users
`covctl` may be used by tech-savvy users who wish to make custom Covalent API calls without having to use `curl` and building the API URLs themselves. Admittedly this project likely will appeal more to developers than users since it lacks a webapp frontend.  

The usability of `covctl` lies in how easy it is to make Covalent API calls without having to worry about structuring the API URL manually, the HTTP header, formatting the returned JSON etc.  

For example to get formatted JSON transactions data for a wallet address on the Klaytn Mainnet(chain_id flag defaults to 8217): `covctl transactions-for-address --addr 0xf4024faad5fafd0755875e3161524c9c4e1a1111`  

## Engineering
One of my major goals during the development of `covctl` and `covalent_class_a` was to apply good software engineering practices to the codebases. This was done with a focus on:

- Automated Testing - `covalent_class_a` has a test suite which covers every API-calling function
- Documentation - `covalent_class_a` has auto-documentation comments written for every function and some example code for developers. Both projects have a descriptive and clear `README.md`
- Usability - Installation of `covctl` is as simple as `cargo install covctl`, and by specifying `covctl --help` all the flags are viewable. The `covalent_class_a` library is auto-documented and straightforward for any Rust project to hook into using it

I believe overall I accomplished my goals for the engineering side of this project - and that this project is now "extensible". The implementation of certain Covalent Class B and the NFT section of Class As would be a straightforward but still time-consuming development task.
