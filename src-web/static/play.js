const urlParams = new URLSearchParams(window.location.search);

document.addEventListener('DOMContentLoaded', play);

async function play(){
    let name = urlParams.get('name');
    let code = urlParams.get('code');
    let uuid;
    
    await fetch("/new_uuid", {
        method: 'GET', 
    })
    .then(handle)
    .then(new_uuid => {
        uuid = new_uuid;
    })
    .catch(notify);

    connect(false, code, uuid, name);
}

function buzz(){
    let time = timer ? current_time() : setting;
    disable_buzzer();
    buzz_completed(time);
}

function enable_buzzer(){
    let buzzer = document.getElementsByClassName('buzzer')[0];
    buzzer.disabled = false;
    buzzer.innerHTML = "Buzz!"
}

function disable_buzzer(){
    let buzzer = document.getElementsByClassName('buzzer')[0];
    buzzer.disabled = true;
    buzzer.innerHTML = "Buzzed!"
}

function buzz_completed(milles){
    data = {
        BuzzCompleted : {
            at: milles,
            response: round,
        }
    }
    text_socket(JSON.stringify(data))
}