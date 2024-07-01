
function create_room(){
    
    lock_btn();

    fetch("/verify-email", {
        method: 'POST', 
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data), 
    })
    .then(handle)
    .then(_ => {

    })
    .catch(unlock_notify);
}