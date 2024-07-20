const urlParams = new URLSearchParams(window.location.search);

document.addEventListener('DOMContentLoaded', play);

async function play(){
    let name = urlParams.get('name');
    let code = urlParams.get('code');

    connect(false, code, name);
}

function buzz(){
    let time = current_time();
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

// function clear_buzzer(){

// }

function buzz_completed(milles){
    data = {
        BuzzCompleted : {
            at: milles,
            response: round,
        }
    }
    text_socket(JSON.stringify(data))
}