use crate::server::server::State;
use std::collections::HashSet;
use warp::ws::Message;

pub async fn broadcast_message_to_room(
    room_id: &usize,
    message: String,
    state: &State,
    my_id: &usize,
) {
    let read_lock = state.read().await;
    println!(
        "broadcasting message to room({}) from user({})",
        room_id, my_id
    );
    let users_in_room: &HashSet<usize> = &read_lock.rooms.get(room_id).unwrap().user_ids;
    for (&uid, tx) in read_lock.peer_map.iter() {
        if my_id.clone() != uid && users_in_room.contains(&uid) {
            if let Err(_disconnected) = tx.send(Message::text(message.clone())) {
                //user disconnection is handled in a different task.
            }
        }
    }
}
