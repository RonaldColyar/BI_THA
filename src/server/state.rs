use crate::server::state_types::{Room, User};
use crate::server::transcriptor::RoomTranscript;
use std::collections::HashMap;
use tokio::sync::mpsc;
use warp::ws::Message;
pub struct ServerState {
    pub rooms: HashMap<usize, Room>,
    pub active_users: HashMap<usize, User>,
    pub peer_map: HashMap<usize, mpsc::UnboundedSender<Message>>,
    pub room_transcripts: HashMap<usize, RoomTranscript>,
}
