use std::env;
use std::str::FromStr;

use web3::{ 
    transports::WebSocket,
    {types::{Address, H160, U256}},
    {contract::{Contract, Options}}
};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    
    // instance to conn to eth network
    let websocket = WebSocket::new(&env::var("GORLI").unwrap()).await?;
    let w3sock = web3::Web3::new(websocket);

    // get all accounts & push our one
    let mut accounts = w3sock.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    // get balance
    for account in accounts {
        let balance = w3sock.eth().balance(account, None).await?;
        println!(
            "ETH balance of {:?} is: {}",
            account,
            wei_to_eth(balance)
        )
    }

    // token info
    let token_addr = Address::from_str("0x1287C6DFd1e3859847EA810313d9487887bF6e28").unwrap();
    let token_contract =
        Contract::from_json(w3sock.eth(), token_addr, include_bytes!("erc20_abi.json")).unwrap();

    let token_name: String = token_contract
                            .query("name", (), None, Options::default(), None)
                            .await
                            .unwrap();

    let total_sup: U256 = token_contract
                            .query("totalSupply", (), None, Options::default(), None)
                            .await
                            .unwrap();

    println!("Token name: {} , Total supply: {}", token_name, total_sup);

    Ok(())
}

// we divide val 
// since 1 Eth is 1^18 Wei.
fn wei_to_eth(wei_val: U256) -> f64 {
    let result = wei_val.as_u128() as f64;
    result / 1_000_000_000_000_000_000.0
}