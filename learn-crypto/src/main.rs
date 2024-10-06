use dotenv::dotenv;

pub mod inverse;
pub mod eth_wallet;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let wallet = eth_wallet::Wallet::from_file("wallet1.json").unwrap();
    println!("loaded wallet: {:?}", wallet);
    let endpoint = std::env::var("INFURA_WS").unwrap();
    let web3_conn = eth_wallet::establish_web3_connection(&endpoint).await.unwrap();

    let block_number = web3_conn.eth().block_number().await.unwrap();
    println!("block number: {:?}", block_number);

    let balance = wallet.get_balance_in_eth(&web3_conn).await.unwrap();
    println!("balance: {:?}", balance);
}