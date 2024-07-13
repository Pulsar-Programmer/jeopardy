
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

    //join with data
    
}