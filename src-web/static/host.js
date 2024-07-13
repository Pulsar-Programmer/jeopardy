
document.addEventListener('DOMContentLoaded', (_event) => {
    host();
});


function host(){

    let name = "Host";
    let uuid;
    let code;
    
    fetch("/new_uuid", {
        method: 'GET', 
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    .then(new_uuid => {
        uuid = new_uuid;
    })
    .catch(notify);

    fetch("/new_code", {
        method: 'GET', 
        headers: {
            'Content-Type': 'application/json',
        },
    })
    .then(handle)
    .then(new_code => {
        code = new_code;
    })
    .catch(notify);

    let data = {
        client_uuid: uuid,
        client_name: name,
        room_code: code,
    }

    connect(true, data);
}

var users = [];

function add_user(name, id){
    users.push({id: id, name: name});
    //add user on frontend 
}

function remove_user(id){
    users = users.filter((data)=>{
        return data.id !== id;
    });
    //remove user on frontend
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

function start_timers(secs){
    start_timer();
    let nanos = 1_000_000_000 * secs; 
    data = {
        StartTimer: {
            start: nanos,
        }
    }
    text_socket(JSON.stringify(data))
}

function pause_timers(secs){
    pause_timer();
    let nanos = 1_000_000_000 * secs; 
    data = {
        PauseTimer : {
            at: nanos,
        }
    }
    text_socket(JSON.stringify(data))
}