BASICS OF W3 IN RUST WITH ETH
======
W3 interaction with Eth in Rust to retrieve info from Etherscan.
This example includes SOLANA token address, which can be easily changed into another one in ``main.rs`` file.


Setup & run
======

The .env file should contain the following:

```sh
# .env vars
GORLI=wss://goerli.infura.io/ws/v3/xxxxxxxxxxx
ACCOUNT_ADDRESS=xxxxxxxxxx
```
The ``ACCOUNT_ADDRESS`` is the address of a Ethereum wallet without the "0x" prefix from https://www.infura.io/.

The ``GORLI`` value is an endpoint address for Görli testnet from https://www.infura.io/.

```sh
# Start
cargo run
```
