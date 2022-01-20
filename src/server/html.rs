pub static INDEX_HTML_PART_ONE: &str = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Warp Chat</title>
    </head>
    <body>
        <h1>Warp chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">Send</button>
        <script type="text/javascript">
        const chat = document.getElementById('chat');
        const text = document.getElementById('text');
"#;

pub static INDEX_HTML_PART_TWO: &str = r#"

const ws = new WebSocket(uri);
function message(data) {
    const line = document.createElement('p');
    line.innerText = data;
    chat.appendChild(line);
}
ws.onopen = function() {
    chat.innerHTML = '<p><em>Connected!</em></p>';
};
ws.onmessage = function(msg) {
    message(msg.data);
};
ws.onclose = function() {
    chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
};
send.onclick = function() {
    const msg = text.value;
    ws.send(msg);
    text.value = '';
    message('<You>: ' + msg);
};
</script>
</body>
</html>
"#;

pub fn format_html(room_id: usize) -> String {
    let url = format!(
        " const uri = 'ws://' + location.host + '/api/chat-room/' + '{}'",
        room_id
    );
    return format!("{}{}{}", INDEX_HTML_PART_ONE, url, INDEX_HTML_PART_TWO);
}
