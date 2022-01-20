mod server {
    pub mod chat;
    pub mod html;
    pub mod rooms;
    pub mod server;
    pub mod state;
    pub mod state_types;
    pub mod transcriptor;
    pub mod users;
}
mod test;

#[tokio::main]
async fn main() {
    server::server::start().await;
}
