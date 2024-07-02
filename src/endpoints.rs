use std::time::{Duration, Instant};

use actix::{Actor, Handler, StreamHandler};
use actix_web::{get, HttpResponse, Responder};
use actix_web_actors::ws;
use actix::{ActorContext, AsyncContext};


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



///Time between pinging the client checking for the heartbeat.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
///Time to expect a message back from the Client.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

///Represents a connection between the server and a single client.
pub struct WebsocketConnection{
    room_code: usize,
    is_host: bool,
    last_hb: Instant,
}

impl WebsocketConnection{
    fn new(room_code: usize, is_host: bool) -> Self{
        Self { room_code, is_host, last_hb: Instant::now() }
    }
    fn setup_hb(&self, ctx: &mut <Self as Actor>::Context){
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx|{
            if Instant::now().duration_since(actor.last_hb) > CLIENT_TIMEOUT{
                println!("Disconnecting due to failed heartbeat.");
                //disconnect the lobby
                todo!();
                ctx.stop();
                return
            }
            ctx.ping(b"PING")
        });
    }
}

impl Actor for WebsocketConnection{
    ///This is an actor for Websocket management.
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        //heartbeat something


    }
    
    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        actix::Running::Stop
    }

    
}

type Msg = Result<ws::Message, ws::ProtocolError>;
impl StreamHandler<Msg> for WebsocketConnection{
    fn handle(&mut self, item: Msg, ctx: &mut Self::Context) {
        match item{
            Ok(msg) => {
                match msg{
                    ws::Message::Text(string) => {
                        todo!("{string}");
                    },
                    ws::Message::Binary(msg) => ctx.binary(msg),
                    ws::Message::Continuation(_) => ctx.stop(),
                    ws::Message::Ping(msg) => {
                        self.last_hb = Instant::now();
                        ctx.pong(&msg);
                    },
                    ws::Message::Pong(_) => self.last_hb = Instant::now(),
                    ws::Message::Close(reason) => {
                        ctx.close(reason);
                        ctx.stop();
                    },
                    ws::Message::Nop => {},
                }
            },
            Err(e) => panic!("{e}"),
        }
    }
}

impl Handler<WsMessage> for WebsocketConnection{
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

use actix::prelude::Message;
#[derive(Message)]
#[rtype(result = "()")] //Return type of Actor message;
struct WsMessage(String);

#[derive(Message)]
#[rtype(result = "()")]
struct Connect{
    //populates
}

#[derive(Message)]
#[rtype(result = "()")]
struct Disconnect{
    //populates
}

#[derive(Message)]
#[rtype(result = "()")]
struct ClientActorMessage{
    //populates
}








///This is to join as a host.
fn join_as_host(){

}

///This is to join as a player that can buzz.
fn join_as_player(){

}



// ws://play to join as a player
// ws://host to join as a host



//make room
//join room

//buzzer system
// #[get()]
// pub async fn start_buzzer() -> impl Responder{

// }