
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
    text_socket(`"LockBuzzers"`)
}

function clear_buzzers(){
    text_socket(`"ClearBuzzers"`)
}

function start(){
    let seconds = setting;
    start_timers(seconds);
}

function pause(){
    let seconds = current_time();
    console.log(`Pause: ${seconds}`);
    pause_timers(seconds);
}

function set(){
    setting = document.getElementById("start_time").value * 1000;
    start_time = Date.now();
    let time = setting;
    document.getElementById("timer").innerHTML = `Timer: ${Math.max(0, Math.floor(time / 1000))}s ${Math.max(0, time % 1000)}ms`;
}

function start_timers(milles){
    start_timer(milles);
    data = {
        StartTimer: {
            start: milles,
        }
    }
    text_socket(JSON.stringify(data))
}

function pause_timers(milles){
    pause_timer(milles);
    data = {
        PauseTimer : {
            at: milles,
        }
    }
    text_socket(JSON.stringify(data))
}