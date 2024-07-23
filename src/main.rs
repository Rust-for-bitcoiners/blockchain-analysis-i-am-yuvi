use std::{env, time};

use bitcoin::Block;
use bitcoincore_rpc::{
    json,
    jsonrpc::{self, serde_json::error},
    Auth, Client, Error, RpcApi,
};
use chrono::Duration;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RPC_CLIENT: Client = {
        dotenv::dotenv().ok();
        let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
        let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
        let rpc_password: String =
            env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");
        Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap()
    };
}

// static client: Client = Client::new("url", Auth::UserPass("user".to_owned(), "password".to_owned())).unwrap();

// Task 1
fn time_to_mine(block_height: u64) -> Duration {
    // * is a deref operator which invokes the Deref trait of the type RPC_CLIENT which was created
    // when the lazy macro is expanded
    // if a value has a static lifetime then it means that value lives as long as the program lives
    let rpc_client: &Client = &*RPC_CLIENT;
    // todo-done
    let current_block: Block = get_block_dat(&rpc_client, block_height).unwrap();

    let current_block_time = current_block.header.time;

    let next_block: Block = get_block_dat(rpc_client, block_height + 1).unwrap();

    let next_block_time = next_block.header.time;

    Duration::new((next_block_time - current_block_time).into(), 0).unwrap()
}

fn get_block_dat(rpc_client: &Client, block_height: u64) -> Result<Block, Error> {
    let block_hash = rpc_client.get_block_hash(block_height)?;
    rpc_client.get_block(&block_hash)
}

// TODO: Task 2
fn number_of_transactions(block_height: u64) -> u16 {
    let some_value = Box::new(4 as u32);
    todo!()
}

fn main() {
    // you can use rpc_client here as if it was a global variable
    // println!("{:?}", res);
    const TIMEOUT_UTXO_SET_SCANS: time::Duration = time::Duration::from_secs(60 * 8); // 8 minutes
    dotenv::dotenv().ok();
    let rpc_url: String = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
    let rpc_user: String = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
    let rpc_password: String =
        env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");

    let custom_timeout_transport = jsonrpc::simple_http::Builder::new()
        .url(&rpc_url)
        .expect("invalid rpc url")
        .auth(rpc_user, Some(rpc_password))
        .timeout(TIMEOUT_UTXO_SET_SCANS)
        .build();
    let custom_timeout_rpc_client =
        jsonrpc::client::Client::with_transport(custom_timeout_transport);

    let rpc_client = Client::from_jsonrpc(custom_timeout_rpc_client);
    let res: json::GetTxOutSetInfoResult =
        rpc_client.get_tx_out_set_info(None, None, None).unwrap();
    println!("{:?}", res);

    //Display time to mine the block
    println!("Time to mine block {} and {}: {}", 15, 16, time_to_mine(15));
}
