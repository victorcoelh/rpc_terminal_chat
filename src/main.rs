use std::sync::{Arc, Mutex};
use std::io::{Write, stdout};

use text_io::read;
use tokio::runtime::Runtime;

use rpc_terminal_chat::{user_chat::UserChat,
    server::start_rpc_server,
    client::RpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let player_data = Arc::new(Mutex::new(UserChat::new()));
    let player_data_pointer = player_data.clone();
    let rt = Runtime::new().unwrap();

    println!("Insert your IP Address:");
    let ip_addr: String = read!();

    rt.spawn(async move {
        start_rpc_server(player_data_pointer, &ip_addr).await.unwrap();
    });

    println!("Insert client IP Address:");
    let ip_addr = read!();
    let mut client = RpcClient::new(ip_addr).await.expect("a");

    stdout().flush().unwrap();
    loop {
        let msg: String = read!("{}\n");

        player_data.lock().unwrap().add_message(msg.clone());
        client.send_chat_message(msg).await?;
    }
}
