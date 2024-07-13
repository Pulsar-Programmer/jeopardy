/** @type {WebSocket | null} */
var socket = null;

var round = 0;

function connect(is_host, userData) {
    disconnect()

    const { location } = window

    const protocol = location.protocol.startsWith('https') ? 'wss' : 'ws'
    const wsUri = `${protocol}://${location.host}/${is_host ? "host" : "play"}`

    console.log('Connecting...')
    socket = new WebSocket(wsUri)

    socket.onopen = () => {
        console.log('Connected');
        socket.send(JSON.stringify(userData));
        console.log(`Sent data: ${userData}`);
    }

    const handle_message = is_host ? handle_message_host : handle_message_player;

    socket.onmessage = (ev) => {
        console.log('Received: ' + ev.data)
        handle_message(ev.data);
    }

    socket.onclose = () => {
        console.log('Disconnected')
        socket = null
    }

    socket.onerror = (ev) => {
        console.log(ev.data);
    }
}

function disconnect() {
    if (socket) {
        console.log('Disconnecting...')
        socket.close()
        socket = null
        redirect("/");
    }
}

function handle_message_host(text){
    let object = JSON.parse(text);
    if(object.BuzzCompleted){

    } else if(object.AddUser){
        add_user(object.AddUser.client_name, object.AddUser.client_id);
    } else if(object.RemoveUser){
        remove_user(object.RemoveUser.client_id);
    }
}

function handle_message_player(text){
    let object = JSON.parse(text);
    if(object.LockBuzzer){
        
    } else if(object.Kicked){
        alert("Kicked from the lobby!");
        disconnect();
    } else if(object.StartTimer){
        round = object.StartTimer.round;

    } else if(object.PauseTimer){

    } else if(object.CodeNotFound){
        alert("Code not found!");
        disconnect();
    // } else if(object.AddUser){
    //     add_user(object.AddUser.client_name, object.AddUser.client_id);
    // } else if(object.RemoveUser){
    //     remove_user(object.RemoveUser.client_id);
    }
}

function text_socket(text){
    socket.send(text);
}

function start_timer(){

}

function pause_timer(){

}