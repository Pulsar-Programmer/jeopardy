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
        disable_buzzer();
    } else if(object.Kicked){
        alert("Kicked from the lobby!");
        disconnect();
    } else if(object.StartTimer){
        round = object.StartTimer.round;
    } else if(object.PauseTimer){
        pause_timer();
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


var seconds = 0;
var timer_enabled = false;
var timer;

function start_timer(secs) {
    seconds = secs;
    timer_enabled = true;
    timer = setInterval(function() {
        if (!timer_enabled) {
            clearInterval(timer);
            return;
        }

        seconds += 0.01; // Increment seconds
        seconds = Math.floor(seconds * 100) / 100; // Round to two decimal places

        document.getElementById("timer").innerHTML = `Timer: ${seconds}s`;

        // If the count down is over, write some text 
        if (seconds >= 60) { // Assuming you want to count up to 60 seconds
            clearInterval(timer);
            return;
        }
    }, 10);
}

function pause_timer(){
    timer_enabled = false;
}

function resume_timer(){
    timer_enabled = true;
}


function stop_timer() {
    timer_enabled = false;
    clearInterval(timer);
}

// document.getElementById("start_timer").addEventListener("click", start_timer);
// document.getElementById("pause_timer").addEventListener("click", pause_timer);
// document.getElementById("stop_timer").addEventListener("click", stop_timer);

