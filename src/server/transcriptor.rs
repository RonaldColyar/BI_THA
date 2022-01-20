/*
This module is responsible for writing logs in JSON format to the local
file system.

The current system only keeps note of messages along with the
timestamps for each individual message.
*/

use crate::server::server::State;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tokio::time::{sleep, Duration};

#[derive(Deserialize, Serialize)]
pub struct RoomMessage {
    pub message: String,
    pub sent_at: String, //datetime not serializable by serde
}

#[derive(Deserialize, Serialize)]
pub struct RoomTranscript {
    pub id: usize,
    pub started_at: String, //datetime not serializable by serde
    pub messages: HashMap<usize, Vec<RoomMessage>>,
}

pub fn create_initial_room_transcript(room_id: usize) -> RoomTranscript {
    let relative_location = format!("room{}.json", room_id);
    println!("creating initial transcript at {}", relative_location);
    let transcript = RoomTranscript {
        id: room_id,
        started_at: Utc::now().to_string(),
        messages: HashMap::new(),
    };
    let string_to_write = serde_json::to_string(&transcript).unwrap();
    fs::write(relative_location, string_to_write).expect("Unable to write file");
    return transcript;
}

pub fn start_updating_task(state: State) {
    tokio::task::spawn(async move {
        update_transcripts_every_x_seconds(30, state.clone()).await;
    });
}

pub fn create_new_room_message(msg: String) -> RoomMessage {
    return RoomMessage {
        message: msg,
        sent_at: Utc::now().to_string(),
    };
}

fn update_transcript_file(transcript: &mut RoomTranscript) {
    let relative_location = format!("room{}.json", transcript.id);
    println!("updating transcript at {}", relative_location);
    let old_transcript_str =
        fs::read_to_string(relative_location.clone()).expect("Unable to read file");
    let mut old_transcript_struct: RoomTranscript =
        serde_json::from_str(&old_transcript_str).unwrap();
    let old_users_keys = old_transcript_struct
        .messages
        .keys()
        .copied()
        .collect::<Vec<_>>();
    //add the old messages with the new messages before rewriting to the file
    for key in old_users_keys {
        if transcript.messages.contains_key(&key) {
            //adds the new messages from a user with the old messages from a user
            transcript
                .messages
                .get_mut(&key)
                .unwrap()
                .append(old_transcript_struct.messages.get_mut(&key).unwrap());
        } else {
            //adds the old messages from a user that hasn't sent a new message since the last update
            let messages = old_transcript_struct.messages.remove(&key).unwrap();
            transcript.messages.insert(key, messages);
        }
    }

    let string_to_write = serde_json::to_string(transcript).unwrap();
    fs::write(relative_location, string_to_write).expect("Unable to write file");
    clear_old_messages_without_removing_keys(transcript);
}

async fn update_transcripts_every_x_seconds(seconds_to_wait: u64, state: State) {
    loop {
        sleep(Duration::from_secs(seconds_to_wait)).await;
        let room_ids = state.read().await.rooms.keys().copied().collect::<Vec<_>>();
        for room_id in room_ids {
            update_transcript_file(
                &mut state
                    .write()
                    .await
                    .room_transcripts
                    .get_mut(&room_id)
                    .unwrap(),
            );
        }
    }
}

fn clear_old_messages_without_removing_keys(transcript: &mut RoomTranscript) {
    let keys = transcript.messages.keys().copied().collect::<Vec<_>>();

    for key in keys {
        //clear the message vec
        transcript.messages.get_mut(&key).unwrap().clear();
    }
}
