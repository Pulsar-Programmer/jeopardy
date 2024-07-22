/** @type {WebSocket | null} */
var socket = null;

var round = 0;

function connect(is_host, room_code, uuid, name) {
    disconnect()

    const { location } = window

    const protocol = location.protocol.startsWith('https') ? 'wss' : 'ws'
    const wsUri = `${protocol}://${location.host}/ws_${is_host ? "host" : "play"}`
    
    const wsUriData = `${wsUri}/${room_code}${uuid}${name}`;

    console.log('Connecting...')
    socket = new WebSocket(wsUriData)

    socket.onopen = () => {
        
        console.log('Connected');
        // socket.send(JSON.stringify(userData));
        // console.log(`Sent data: ${userData}`);
    }

    const handle_message = is_host ? handle_message_host : handle_message_player;

    socket.onmessage = (ev) => {
        console.log('Received: ' + ev.data)
        handle_message(ev.data);
    }

    socket.onclose = () => {
        alert("Unexpected close!");
        console.log("Unexpected close!");
        disconnect();
    }

    socket.onerror = (ev) => {
        console.log(ev);
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
    } else if(object.NewCode){
        document.getElementById("gamecode").innerHTML = `Game Code: ${object.NewCode.code}`;
    }
}

function handle_message_player(text){
    let object = JSON.parse(text);
    if(object === "LockBuzzer"){
        disable_buzzer();
    } else if(object === "ClearBuzzer"){
        enable_buzzer();
    } else if(object === "Kicked"){
        alert("Kicked from the lobby!");
        disconnect();
    } else if(object.StartTimer){
        round = object.StartTimer.round;
        start_timer(object.StartTimer.start);
    } else if(object.PauseTimer){
        pause_timer(object.PauseTimer.at);
    } else if(object === "CodeNotFound"){
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

var setting = 0;
var start_time;
var timer;

function start_timer(ms){
    setting = ms;
    start_time = Date.now();
    clearInterval(timer);
    timer = setInterval(update_display, 1);
}

function pause_timer(ms){
    setting = ms;
    document.getElementById("timer").innerHTML = `Timer: ${Math.max(0, Math.floor(ms / 1000))}s ${Math.max(0, ms % 1000)}ms`;
    clearInterval(timer);
}

function update_display() {
    console.log(setting);
    console.log(start_time);
    let time = current_time();
    document.getElementById("timer").innerHTML = `Timer: ${Math.max(0, Math.floor(time / 1000))}s ${Math.max(0, time % 1000)}ms`;
    if (time === 0 ){
        pause_timer(0);
    }
}

function current_time(){
    let current_time = Date.now();
    let elapsed_time = current_time - start_time;
    let mille = Math.floor(elapsed_time);
    return Math.max(0, setting - mille);
}