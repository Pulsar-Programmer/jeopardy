
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



use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use crate::server::WebsocketConnection;


/// WebSocket handshake and start `MyWebSocket` actor.
/// This is to join as a host that moderates the game.
#[get("/host")]
pub async fn ws_host(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WebsocketConnection::host(), &req, stream)
}

/// WebSocket handshake and start `MyWebSocket` actor.
/// This is to join as a player that can buzz.
#[get("/play")]
pub async fn ws_play(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WebsocketConnection::player(), &req, stream)
}


#[get("/new_code")]
pub async fn new_code() -> Result<HttpResponse, Error>{
    todo!("check that headers are not from browser (necessary? idk)");
    todo!("generate a random code and return it to be used in the creation of a new room")
}


// ws://ws_play to join as a player
// ws://ws_host to join as a host



//make room
//join room

//buzzer system
// #[get()]
// pub async fn start_buzzer() -> impl Responder{

// }
