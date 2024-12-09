use std::sync::{Mutex, Arc};

use tonic::{Request, Response, transport::Server};

use crate::user_chat::UserChat;
use crate::othello_rpc::chat_server::{Chat, ChatServer};
use crate::othello_rpc::{ChatRequest, Empty};

pub mod othello_rpc {
    tonic::include_proto!("othello_rpc");
}

struct RpcServer {
    player_data: Arc<Mutex<UserChat>>
}

#[tonic::async_trait]
impl Chat for RpcServer {
    async fn send_message(
        &self,
        request: Request<ChatRequest>
    ) -> Result<Response<Empty>, tonic::Status> {
        let msg = request.into_inner().text;
        self.player_data.lock().unwrap().add_message(msg);

        let msgs = &self.player_data.lock().unwrap().messages;
        println!("{:?}", msgs);
        
        Ok(Response::new(Empty{}))
    }
}

pub async fn start_rpc_server(player_data: Arc<Mutex<UserChat>>, ip_addr: &str)
    -> Result<(), Box<dyn std::error::Error>> {
    let addr = ip_addr.parse()?;
    let server = RpcServer { player_data };

    Server::builder()
        .add_service(ChatServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
