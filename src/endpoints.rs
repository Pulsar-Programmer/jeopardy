
#[macro_export]
macro_rules! website {
    ($($i:ident; $e:expr),+) => {
        $(
            pub const $i: &'static str = include_str!(concat!("../src-web/", $e, ".html"));
        )*
    };
}
website!(
    HOMEPAGE; "homepage",
    JOIN; "join",
    PLAY; "play",
    HOST; "host"
);


///This is the `index` endpoint for the browser.
#[get("/")]
pub async fn homepage() -> impl Responder{
    HttpResponse::Ok().body(HOMEPAGE)
}

///This is the `join` endpoint for the browser.
#[get("/join")]
pub async fn join() -> impl Responder{
    HttpResponse::Ok().body(JOIN)
}



///This is the `play` endpoint simply to display the page for the browser.
#[get("/play")]
pub async fn play() -> impl Responder{
    HttpResponse::Ok().body(PLAY)
}

///This is the `host` endpoint simply to display the page for the browser.
#[get("/host")]
pub async fn host() -> impl Responder{
    HttpResponse::Ok().body(HOST)
}



use actix::Addr;
use actix_web::{get, web::{self, Data}, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use rand::Rng;
use uuid::Uuid;
use crate::{lobby::Lobby, server::WebsocketConnection};

#[derive(serde::Serialize, serde::Deserialize)]
struct UserData{
    client_uuid: Uuid,
    client_name: String,
    room_code: u32,
}

/// WebSocket handshake and start `MyWebSocket` actor.
/// This is to join as a host that moderates the game.
#[get("/host")]
pub async fn ws_host(req: HttpRequest, stream: web::Payload, user_data: web::Json<UserData>, srv: Data<Addr<Lobby>>) -> Result<HttpResponse, Error> {
    let user_data = user_data.into_inner();
    let srv = srv.get_ref().clone();
    ws::start(WebsocketConnection::host(srv, user_data.room_code, user_data.client_name, user_data.client_uuid), &req, stream)
}

/// WebSocket handshake and start `MyWebSocket` actor.
/// This is to join as a player that can buzz.
#[get("/play")]
pub async fn ws_play(req: HttpRequest, stream: web::Payload, user_data: web::Json<UserData>, srv: Data<Addr<Lobby>>) -> Result<HttpResponse, Error> {
    let user_data = user_data.into_inner();
    let srv = srv.get_ref().clone();
    ws::start(WebsocketConnection::player(srv, user_data.room_code, user_data.client_name, user_data.client_uuid), &req, stream)
}

//js will have to know the UUID of self and others in the lobby. We might have to store both because the other clients need to know the name

#[get("/new_code")]
pub async fn new_code() -> HttpResponse{
    let code = rand::thread_rng().gen_range(100_000..1_000_000) as u32;
    HttpResponse::Ok().json(code)
}

#[get("/new_uuid")]
pub async fn new_uuid() -> HttpResponse{
    let uuid = uuid::Uuid::new_v4();
    HttpResponse::Ok().json(uuid)
}

// ws://ws_play to join as a player
// ws://ws_host to join as a host

//buzzer system
// #[get()]
// pub async fn start_buzzer() -> impl Responder{

// }
