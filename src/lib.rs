pub mod server;
pub mod client;
pub mod user_chat;

pub mod othello_rpc {
    tonic::include_proto!("othello_rpc");
}
