use std::time::{Duration, Instant};

use actix::{Actor, Handler, StreamHandler};
use actix_web::{get, HttpResponse, Responder};
use actix_web_actors::ws;
use actix::{ActorContext, AsyncContext};
use actix_files::NamedFile;
use actix_web::{
    middleware, rt, App, Error, HttpRequest, HttpServer,
};


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


use actix_web::web;
use actix_ws::Message;
use futures_util::{
    future::{self, Either},
    StreamExt as _,
};
use tokio::{pin, select, sync::broadcast, time::interval};

/// How often heartbeat pings are sent.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Echo text & binary messages received from the client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn handler_echo_heartbeat_ws(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
) {
    println!("Connected");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let reason = loop {
        // create "next client timeout check" future
        let tick = interval.tick();
        // required for select()
        pin!(tick);

        // waits for either `msg_stream` to receive a message from the client or the heartbeat
        // interval timer to tick, yielding the value of whichever one is ready first
        match future::select(msg_stream.next(), tick).await {
            // received message from WebSocket client
            Either::Left((Some(Ok(msg)), _)) => {
                println!("msg: {msg:?}");

                match msg {
                    Message::Text(text) => {
                        session.text(text).await.unwrap();
                    }

                    Message::Binary(bin) => {
                        session.binary(bin).await.unwrap();
                    }

                    Message::Close(reason) => {
                        break reason;
                    }

                    Message::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    Message::Continuation(_) => {
                        println!("no support for continuation frames");
                    }

                    // no-op; ignore
                    Message::Nop => {}
                };
            }

            // client WebSocket stream error
            Either::Left((Some(Err(err)), _)) => {
                println!("{err}");
                break None;
            }

            // client WebSocket stream ended
            Either::Left((None, _)) => break None,

            // heartbeat interval ticked
            Either::Right((_inst, _)) => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    println!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );

                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            }
        }
    };

    // attempt to close connection gracefully
    let _ = session.close(reason).await;

    println!("disconnected");
}

/// Echo text & binary messages received from the client and respond to ping messages.
///
/// This example is just for demonstration of simplicity. In reality, you likely want to include
/// some handling of heartbeats for connection health tracking to free up server resources when
/// connections die or network issues arise.
///
/// See [`echo_heartbeat_ws`] for a more realistic implementation.
pub async fn handler_echo_ws(mut session: actix_ws::Session, mut msg_stream: actix_ws::MessageStream) {
    println!("connected");

    let close_reason = loop {
        match msg_stream.next().await {
            Some(Ok(msg)) => {
                println!("msg: {msg:?}");

                match msg {
                    Message::Text(text) => {
                        session.text(text).await.unwrap();
                    }

                    Message::Binary(bin) => {
                        session.binary(bin).await.unwrap();
                    }

                    Message::Close(reason) => {
                        break reason;
                    }

                    Message::Ping(bytes) => {
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {}

                    Message::Continuation(_) => {
                        println!("no support for continuation frames");
                    }

                    // no-op; ignore
                    Message::Nop => {}
                };
            }

            // error or end of stream
            _ => break None,
        }
    };

    // attempt to close connection gracefully
    let _ = session.close(close_reason).await;

    println!("disconnected");
}

/// Broadcast text & binary messages received from a client, respond to ping messages, and monitor
/// connection health to detect network issues and free up resources.
pub async fn handler_broadcast_ws(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    mut rx: broadcast::Receiver<web::Bytes>,
) {
    println!("connected");

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let reason = loop {
        // waits for either `msg_stream` to receive a message from the client, the broadcast channel
        // to send a message, or the heartbeat interval timer to tick, yielding the value of
        // whichever one is ready first
        select! {
            broadcast_msg = rx.recv() => {
                let msg = match broadcast_msg {
                    Ok(msg) => msg,
                    Err(broadcast::error::RecvError::Closed) => break None,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                };

                let res = match std::str::from_utf8(&msg) {
                    Ok(val) => session.text(val).await,
                    Err(_) => session.binary(msg).await,
                };

                if let Err(err) = res {
                    println!("{err}");
                    break None;
                }
            }

            // heartbeat interval ticked
            _tick = interval.tick() => {
                // if no heartbeat ping/pong received recently, close the connection
                if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                    println!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );

                    break None;
                }

                // send heartbeat ping
                let _ = session.ping(b"").await;
            },

            msg = msg_stream.next() => {
                let msg = match msg {
                    // received message from WebSocket client
                    Some(Ok(msg)) => msg,

                    // client WebSocket stream error
                    Some(Err(err)) => {
                        println!("{err}");
                        break None;
                    }

                    // client WebSocket stream ended
                    None => break None
                };

                dbg!(&msg);

                match msg {
                    Message::Text(_) => {
                        // drop client's text messages
                    }

                    Message::Binary(_) => {
                        // drop client's binary messages
                    }

                    Message::Close(reason) => {
                        break reason;
                    }

                    Message::Ping(bytes) => {
                        last_heartbeat = Instant::now();
                        let _ = session.pong(&bytes).await;
                    }

                    Message::Pong(_) => {
                        last_heartbeat = Instant::now();
                    }

                    Message::Continuation(_) => {
                        println!("no support for continuation frames");
                    }

                    // no-op; ignore
                    Message::Nop => {}
                };
            }
        }
    };

    dbg!(&reason);

    // attempt to close connection gracefully
    let _ = session.close(reason).await;

    println!("Disconnected");
}

/// Handshake and start WebSocket handler with heartbeats.
async fn echo_heartbeat_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    rt::spawn(handler_echo_heartbeat_ws(session, msg_stream));

    Ok(res)
}

/// Handshake and start basic WebSocket handler.
///
/// This example is just for demonstration of simplicity. In reality, you likely want to include
/// some handling of heartbeats for connection health tracking to free up server resources when
/// connections die or network issues arise.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    rt::spawn(handler_echo_ws(session, msg_stream));

    Ok(res)
}

/// Send message to clients connected to broadcast WebSocket.
async fn send_to_broadcast_ws(
    body: web::Bytes,
    tx: web::Data<broadcast::Sender<web::Bytes>>,
) -> Result<impl Responder, Error> {
    tx.send(body)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent())
}

/// Handshake and start broadcast WebSocket handler with heartbeats.
async fn broadcast_ws(
    req: HttpRequest,
    stream: web::Payload,
    tx: web::Data<broadcast::Sender<web::Bytes>>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    rt::spawn(handler_broadcast_ws(session, msg_stream, tx.subscribe()));

    Ok(res)
}


// .service(web::resource("/ws").route(web::get().to(echo_heartbeat_ws)))
// .service(web::resource("/ws-basic").route(web::get().to(echo_ws)))
// .app_data(web::Data::new(tx.clone()))
// .service(web::resource("/ws-broadcast").route(web::get().to(broadcast_ws)))
// .service(web::resource("/send").route(web::post().to(send_to_broadcast_ws)))








///This is to join as a host that moderates the game.
fn ws_host(){

}

///This is to join as a player that can buzz.
fn ws_play(){

}



// ws://ws_play to join as a player
// ws://ws_host to join as a host



//make room
//join room

//buzzer system
// #[get()]
// pub async fn start_buzzer() -> impl Responder{

// }