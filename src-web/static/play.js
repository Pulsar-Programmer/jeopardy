


function play(){
    let name = null;
    let code = null;
    let uuid;
    
    fetch("/new_uuid", {
        method: 'GET', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data), 
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

    //join with data
}