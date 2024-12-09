use tonic::transport::Channel;

use crate::othello_rpc::chat_client::ChatClient;
use crate::othello_rpc::ChatRequest;

pub struct RpcClient {
    chat_client: ChatClient<Channel>
}

impl RpcClient {
    pub async fn new(ip_addr: String) -> Result<Self, tonic::transport::Error> {
        let url = format!("http://{}", ip_addr);
        let chat = ChatClient::connect(url).await?;

        Ok(RpcClient { chat_client: chat })
    }

    pub async fn send_chat_message(&mut self, msg: String) -> Result<(), String> {
        let request = tonic::Request::new(ChatRequest {
            text: msg
        });

        self.chat_client.send_message(request)
            .await
            .map_err(|err| {
                format!("Error code {} while calling an RPC function: {}", err.code(), err.message())
            })?;
        Ok(())
    }
}
