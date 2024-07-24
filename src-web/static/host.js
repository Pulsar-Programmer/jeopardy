
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
    document.getElementById("players").innerHTML += `<p class="subtitle player" id="${id}"><button class="unbuzzed card" onclick="reset_buzz('${id}')">--.--</button> ${name}<button class="kick" onclick="kick_user('${id}')">X</button></p>`;
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
    text_socket(`"LockBuzzers"`);
}

function clear_buzzers(){
    buzzed = [];
    update_buzzed();
    text_socket(`"ClearBuzzers"`);
}

function start(){
    let seconds = setting;
    start_timers(seconds);
}

function pause(){
    let seconds = timer ? current_time() : setting;
    console.log(`Pause: ${seconds}`);
    pause_timers(seconds);
}

function set(){
    setting = document.getElementById("start_time").value * 1000;
    start_time = Date.now();
    let time = setting;
    document.getElementById("timer").innerHTML = `Timer: ${Math.max(0, Math.floor(time / 1000))}s ${Math.max(0, time % 1000)}ms`;
    if(document.getElementById("set_merge").checked){
        start();
    }
}

function start_timers(milles){
    start_timer(milles);
    data = {
        StartTimer: {
            start: milles,
        }
    }
    text_socket(JSON.stringify(data))
    if(document.getElementById("start_merge").checked){
        clear_buzzers();
    }
}

function pause_timers(milles){
    pause_timer(milles);
    data = {
        PauseTimer : {
            at: milles,
        }
    }
    text_socket(JSON.stringify(data))
    if(document.getElementById("pause_merge").checked){
        lock_buzzers();
    }
}

var buzzed = [];

function new_buzz(at, uuid){
    if(document.getElementById("pause_buzz").checked){
        pause_timers(at);
    }
    buzzed.push({at, uuid});
    update_buzzed();
    if(document.getElementById("pause_all_buzz").checked && Array.from(document.getElementsByClassName("card")).length === buzzed.length){
        pause_timers(at);
    }
}

function update_buzzed(){
    Array.from(document.getElementsByClassName("card")).forEach((elem) => {
        elem.className = "unbuzzed card";
        elem.innerHTML = "--.--";
    });
    let highest = {at: 0, uuid: "none"};
    buzzed.forEach(({at, uuid}) => {
        let elem = document.getElementById(uuid).querySelector('.card');
        elem.className = "buzzed card";
        elem.innerHTML = `${Math.floor(at / 1000)}.${at % 1000}`;
        if(highest.at <= at){
            highest = {at, uuid};
        }
    });
    if(highest.uuid !== "none"){
        let best = document.getElementById(highest.uuid).querySelector('.card');
        best.className = "first card";
    }
}

function reset_buzz(id){
    let elem = document.getElementById(id).querySelector('.card');
    elem.className = "unbuzzed card";
    elem.innerHTML = "--.--";
    let new_buzzed = [];
    buzzed.forEach(({at, uuid}) => {
        if(uuid !== id){
            new_buzzed.push({at, uuid})
        }
    });
    buzzed = new_buzzed;
}