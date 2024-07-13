/** @type {WebSocket | null} */
var socket = null;

function connect(is_host) {
    disconnect()

    const { location } = window

    const protocol = location.protocol.startsWith('https') ? 'wss' : 'ws'
    const wsUri = `${protocol}://${location.host}/${is_host ? "host" : "play"}`

    console.log('Connecting...')
    socket = new WebSocket(wsUri)

    socket.onopen = () => {
        console.log('Connected')
    }

    let handle_message = is_host ? handle_message_host : handle_message_player;

    socket.onmessage = (ev) => {
        console.log('Received: ' + ev.data)
        handle_message(ev.data);
    }

    socket.onclose = () => {
        console.log('Disconnected')
        socket = null
    }
}

function disconnect() {
    if (socket) {
        console.log('Disconnecting...')
        socket.close()
        socket = null
    }
}

function handle_message_host(text){
    let object = JSON.parse(text);
}

function handle_message_player(text){
    let object = JSON.parse(text);
}


function text_socket(text){
    socket.send(text);
}