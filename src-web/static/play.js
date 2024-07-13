


function play(){
    let name = null;
    let code = null;
    let uuid;
    
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


    let data = {
        client_uuid: uuid,
        client_name: name,
        room_code: code,
    }

    connect(false, data);
}

function buzz(){
    
}


function enable_buzzer(){
    
}

function disable_buzzer(){

}

function clear_buzzer(){

}