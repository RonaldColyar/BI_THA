use crate::server::server::State;
use crate::server::state_types::Room;
use crate::server::transcriptor;
use std::collections::HashSet;

pub async fn add_user_to_room(user_id: &usize, room_id: &usize, state: &State) {
    println!("adding user to room: {}", room_id);
    state
        .write()
        .await
        .rooms
        .get_mut(room_id)
        .unwrap()
        .user_ids
        .insert(user_id.clone());
    //make sure our user has a messages section
    //in our transcript
    state
        .write()
        .await
        .room_transcripts
        .get_mut(&room_id)
        .unwrap()
        .messages
        .insert(user_id.clone(), Vec::new());
}

pub async fn clean_up_room_if_empty(room_id: &usize, state: &State) {
    let amount_of_users = state
        .read()
        .await
        .rooms
        .get(&room_id)
        .unwrap()
        .user_ids
        .len();

    if amount_of_users == 0 {
        //should never require the checking of the returned option
        println!("Ending Room {}", room_id);
        state.write().await.rooms.remove(&room_id);
        state.write().await.room_transcripts.remove(&room_id);
    }
}

pub async fn remove_user_from_room_transcripts(room_id: &usize, user_id: &usize, state: &State) {
    println!("removing user from room transcripts: {}", room_id);

    //helps with the case of lingering users
    state
        .write()
        .await
        .room_transcripts
        .get_mut(room_id)
        .unwrap()
        .messages
        .remove(user_id);
}

pub async fn remove_user_from_room(room_id: &usize, user_id: &usize, state: &State) {
    println!("removing user from room {}", room_id);
    state
        .write()
        .await
        .rooms
        .get_mut(&room_id)
        .unwrap()
        .user_ids
        .remove(&user_id);
}

pub async fn create_room_if_not_present(room_id: &usize, state: &State) {
    if state.read().await.rooms.get(room_id).is_none() {
        println!("Starting Room {}", room_id);
        state.write().await.rooms.insert(
            room_id.clone(),
            Room {
                user_ids: HashSet::new(),
            },
        );
        //make our room transcript exist
        let room_transcript = transcriptor::create_initial_room_transcript(room_id.clone());
        state
            .write()
            .await
            .room_transcripts
            .insert(room_id.clone(), room_transcript);
    }
}
