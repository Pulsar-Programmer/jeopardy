use actix::Addr;
use actix_web::{get, http::header::ContentType, web::{self, Data, Path}, Error, HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web_actors::ws;
use rand::Rng;
use uuid::Uuid;
use crate::{lobby::Lobby, server::WebsocketConnection};


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

///This is an error that can occur during the parsing of the room code and uuid.
#[derive(Debug)]
struct ParseError{
    error: String,
}
impl std::fmt::Display for ParseError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.error)
    }
}
impl ResponseError for ParseError{
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }
}

/// WebSocket handshake and start `MyWebSocket` actor.
/// This is to join as a host that moderates the game.
#[get("/ws_host/{dat}")]
pub async fn ws_host(req: HttpRequest, stream: web::Payload, srv: Data<Addr<Lobby>>, dat: Path<String>) -> Result<HttpResponse, Error> {
    let room_code = &dat[0..6];
    let uuid = &dat[6..42];
    let name = &dat[42..];
    let room_code = match room_code.parse::<u32>(){
        Ok(t) => t,
        Err(e) => return Err(Error::from(ParseError{error: e.to_string()}))
    };
    let name = name.to_string();
    let uuid = match uuid.parse::<Uuid>(){
        Ok(t) => t,
        Err(e) => return Err(Error::from(ParseError{error: e.to_string()}))
    };
    let srv = srv.get_ref().clone();
    ws::start(WebsocketConnection::host(srv, room_code, name, uuid), &req, stream)
}

/// WebSocket handshake and start `MyWebSocket` actor.
/// This is to join as a player that can buzz.
#[get("/ws_host/{dat}")]
pub async fn ws_play(req: HttpRequest, stream: web::Payload, dat: Path<String>, srv: Data<Addr<Lobby>>) -> Result<HttpResponse, Error> {
    let room_code = &dat[0..6];
    let uuid = &dat[6..42];
    let name = &dat[42..];
    let room_code = match room_code.parse::<u32>(){
        Ok(t) => t,
        Err(e) => return Err(Error::from(ParseError{error: e.to_string()}))
    };
    let name = name.to_string();
    let uuid = match uuid.parse::<Uuid>(){
        Ok(t) => t,
        Err(e) => return Err(Error::from(ParseError{error: e.to_string()}))
    };
    let srv = srv.get_ref().clone();
    ws::start(WebsocketConnection::player(srv, room_code, name, uuid), &req, stream)
}

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