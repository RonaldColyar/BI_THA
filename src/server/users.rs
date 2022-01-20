use super::state_types::User;
use crate::server::server::State;
use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

pub async fn add_user_to_state(user_id: &usize, state: &State, tx: UnboundedSender<Message>) {
    let mut write_lock = state.write().await;
    write_lock.active_users.insert(
        user_id.clone(),
        User {
            screen_name: user_id.clone().to_string(),
        },
    );
    write_lock.peer_map.insert(user_id.clone(), tx);
}

pub async fn remove_user_from_state(user_id: &usize, state: &State) {
    let mut write_lock = state.write().await;
    write_lock.active_users.remove(user_id);
    write_lock.peer_map.remove(user_id);
}
