# BI_THA

## Note: The instructions for starting this system are near the bottom of this document.

This is a simple websocket chat server/static frontend that has basic room based functionality. 
The server uses multi producer single consumer based communication channels to achieve nice chat speeds
with one piece of global state.

There are a few different core modules that help complete this system:

- [rooms](https://github.com/RonaldColyar/BI_THA/blob/main/src/server/rooms.rs) which handles the room based logic.
- [chat](https://github.com/RonaldColyar/BI_THA/blob/main/src/server/chat.rs) which handles chat broadcasting across rooms.
- [html](https://github.com/RonaldColyar/BI_THA/blob/main/src/server/html.rs) which is responsible for generating a static html page with minor functionality
- [state](https://github.com/RonaldColyar/BI_THA/blob/main/src/server/state.rs) which is the main server state type that is used throughout the system as a 
  singleton.
- [transcriptor](https://github.com/RonaldColyar/BI_THA/blob/main/src/server/transcriptor.rs) which is responsible for keeping track of chat messaging in the local file system


## A chat room from start to finish

In order to create a room on the server, the user would need to visit a specific url which will automatically spawn a room in the server.
Visiting `http://localhost:3030/chat/:room_number` will take you to the room with the identity `room_number`. Users will chat in this room using
the frontend client(raw html with basic functionality) and when all users leave this room, it will be removed. 

## The transcriptor?

Transcripts are created and updated on a 30 second interval by the server, a transcript is composed of the below items:

- room_id -> int
- created at -> datetime string
- messages -> dynamic array of message objects

Each room owns ONE transcript file and each transcript file is JSON serialized in local storage.
Data is transfered from memory to local storage, to keep the host server resources free.

## Why the 30 second interval for the transcriptor?
I wanted to have a periodic update on messages to avoid updating too often and not so much. Scalability is very important in a system such as this one, where the number of users are unknown.

## Test Coverage
This server has very low test coverage due to its nature.

# Setup And Run

This system is built using the [cargo](https://doc.rust-lang.org/cargo/) build system. 

With cargo installed, run the following commands:

`Cargo install`

`Cargo run`

`Cargo test` is the only command needed to run the single test written.

# Manual Testing ideas
1. Start the server
2. Navigate to `http://localhost:3030/chat/1` in two different browser tabs and exchange messages. Then open a third browser, navigate to `http://localhost:3030/chat/2`, send a message and confirm the message was only broadcasted to the users connected to url that represents room 2.
