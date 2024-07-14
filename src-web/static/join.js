function join(){
    let name = document.getElementById("name").value;
    let room_code = document.getElementById("room_code").value;
    redirect(`/play?room_code=${room_code}&name=${name}`);
}