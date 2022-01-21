// #![deny(warnings)]
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use crate::server::state::ServerState;
use crate::server::{chat, html, rooms, transcriptor, users};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::io::AsyncWriteExt;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
pub type State = Arc<RwLock<ServerState>>;

pub async fn start() {
    pretty_env_logger::init();
    //our global state
    let state = Arc::new(RwLock::new(ServerState {
        rooms: HashMap::new(),
        active_users: HashMap::new(),
        peer_map: HashMap::new(),
        room_transcripts: HashMap::new(),
    }));

    //begin our transcriptor
    transcriptor::start_updating_task(state.clone());

    //global state filter for warp
    let state = warp::any().map(move || state.clone());

    // GET /api/chat-room
    let chat = warp::path!("api" / "chat-room" / usize)
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(state)
        .map(|room_id: usize, ws: warp::ws::Ws, state: State| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| handle_user_connection(socket, state.clone(), room_id))
        });
    
    //GET /chat/:roomId
    let base_path = warp::path!("chat" / usize)
        .map(|room_id: usize| warp::reply::html(html::format_html(room_id)));

    let routes = base_path.or(chat);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_user_connection(ws: WebSocket, state: State, room_id: usize) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    rooms::create_room_if_not_present(&room_id, &state).await;
    println!("new user id: {}", my_id);

    //user channels/websocket connection declaration
    let (user_ws_tx, mut user_ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    spawn_task_to_listen_for_outgoing_messages(rx, user_ws_tx);

    //Save the user in our server state
    users::add_user_to_state(&my_id, &state, tx).await;
    rooms::add_user_to_room(&my_id, &room_id, &state).await;

    block_and_listen_for_incoming_messages(&mut user_ws_rx, &my_id, &room_id, &state).await;
    handle_user_disconnection(&my_id, &room_id, state).await;
}

async fn handle_user_message(my_id: &usize, room_id: &usize, msg: Message, state: &State) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let user_screen_name = &state
        .read()
        .await
        .active_users
        .get(&my_id)
        .unwrap()
        .screen_name
        .clone();
    let new_msg = format!("<User#{}>: {}", user_screen_name, msg);
    //record our message locally in our
    //transcript file and broadcast it
    let room_msg_for_transcript = transcriptor::create_new_room_message(msg.to_string());
    &state
        .write()
        .await
        .room_transcripts
        .get_mut(&room_id)
        .unwrap()
        .messages
        .get_mut(&my_id)
        .unwrap()
        .push(room_msg_for_transcript);
    chat::broadcast_message_to_room(&room_id, new_msg, state, my_id).await;
}

async fn block_and_listen_for_incoming_messages(
    user_ws_rx: &mut SplitStream<WebSocket>,
    my_id: &usize,
    room_id: &usize,
    state: &State,
) {
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };
        handle_user_message(my_id, room_id, msg, state).await;
    }
}

fn spawn_task_to_listen_for_outgoing_messages(
    mut rx: UnboundedReceiverStream<Message>,
    mut user_ws_tx: SplitSink<WebSocket, Message>,
) {
    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });
}

async fn handle_user_disconnection(my_id: &usize, room_id: &usize, state: State) {
    println!("disconnecting user id :{}", my_id);
    users::remove_user_from_state(my_id, &state).await;
    rooms::remove_user_from_room_transcripts(room_id, my_id, &state).await;
    rooms::remove_user_from_room(room_id, my_id, &state).await;
    rooms::clean_up_room_if_empty(room_id, &state).await;
}
