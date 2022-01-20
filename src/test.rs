use crate::server::rooms;
use crate::server::server::State;
use crate::server::state::ServerState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/*
Since this is a test exercise, I
am not aiming for high test coverage rather
to show typical style.
*/

#[tokio::test]
async fn test_room_logic() {
    //our mock global state
    let state: State = Arc::new(RwLock::new(ServerState {
        rooms: HashMap::new(),
        active_users: HashMap::new(),
        peer_map: HashMap::new(),
        room_transcripts: HashMap::new(),
    }));

    //1: adding a room
    //room creation triggers transcript creation, which allows us to
    //test the transcript lightly
    let mock_room_id: usize = 31;
    assert_eq!(state.read().await.rooms.len(), 0);
    assert_eq!(state.read().await.room_transcripts.len(), 0);
    rooms::create_room_if_not_present(&mock_room_id, &state).await;
    assert_eq!(state.read().await.rooms.len(), 1);
    assert_eq!(state.read().await.room_transcripts.len(), 1);

    //---duplicate detection test
    rooms::create_room_if_not_present(&mock_room_id, &state).await;
    assert_eq!(state.read().await.rooms.len(), 1);
    assert_eq!(state.read().await.room_transcripts.len(), 1);

    //2: adding a user to a room
    let mock_user_id: usize = 32;
    assert_eq!(
        state
            .read()
            .await
            .rooms
            .get(&mock_room_id)
            .unwrap()
            .user_ids
            .contains(&mock_user_id),
        false
    );
    rooms::add_user_to_room(&mock_user_id, &mock_room_id, &state).await;
    assert_eq!(
        state
            .read()
            .await
            .rooms
            .get(&mock_room_id)
            .unwrap()
            .user_ids
            .contains(&mock_user_id),
        true
    );

    //3.make sure rooms get cleaned up
    state
        .write()
        .await
        .rooms
        .get_mut(&mock_room_id)
        .unwrap()
        .user_ids
        .remove(&mock_user_id);
    rooms::clean_up_room_if_empty(&mock_room_id, &state).await;
    assert_eq!(state.read().await.rooms.len(), 0);
}
