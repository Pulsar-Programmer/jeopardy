
document.addEventListener('DOMContentLoaded', host);


async function host(){

    let name = "Host";
    let uuid;
    let code;
    
    await fetch("/new_uuid", {
        method: 'GET',
    })
    .then(handle)
    .then(new_uuid => {
        uuid = new_uuid;
    })
    .catch(notify);

    await fetch("/new_code", {
        method: 'GET', 
    })
    .then(handle)
    .then(new_code => {
        console.log(new_code);
        code = new_code;
    })
    .catch(notify);

    connect(true, code, uuid, name);

    document.getElementById("gamecode").innerHTML = `Game Code: ${code}`;
}

function add_user(name, id){
    document.getElementById("players").innerHTML += `<p class="subtitle player" id="${id}">${name}<button class="kick" onclick="kick_user('${id}')">X</button></p>`;
}

function remove_user(id){
    document.getElementById(`${id}`).remove();
}

function kick_user(id){
    data = {
        Kick : {
            uuid: id,
        }
    }
    text_socket(JSON.stringify(data))
}

function lock_buzzers(){
    text_socket("LockBuzzers")
}

function clear_buzzers(){
    text_socket("ClearBuzzers")
}

function start(){
    let seconds = document.getElementById("timer").innerHTML;
    seconds = seconds.replace("Timer: ", "");
    seconds = seconds.replace("s", "");
    start_timers(Number(seconds));
}

function pause(){
    let seconds = document.getElementById("timer").innerHTML;
    seconds = seconds.replace("Timer: ", "");
    seconds = seconds.replace("s", "");
    pause_timers(Number(seconds));
}

function restart(){
    let seconds = document.getElementById("start_time").value;
    start_timers(Number(seconds));
}

function start_timers(secs){
    start_timer(secs);
    let nanos = 1_000_000_000 * secs; 
    data = {
        StartTimer: {
            start: nanos,
        }
    }
    text_socket(JSON.stringify(data))
}

function pause_timers(secs){
    pause_timer(secs);
    let nanos = 1_000_000_000 * secs; 
    data = {
        PauseTimer : {
            at: nanos,
        }
    }
    text_socket(JSON.stringify(data))
}