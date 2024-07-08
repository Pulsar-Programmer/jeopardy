use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_web_actors::ws;
use uuid::Uuid;
use crate::lobby::*;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct WebsocketConnection {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    last_hb: Instant,
    is_host: bool, //fix this later
    room: Addr<Room>,
    room_code: u32,
    client_id: Uuid,
}

impl WebsocketConnection {

    pub fn host(room: Addr<Room>, room_code: u32) -> Self{
        Self { last_hb: Instant::now(), is_host: true, room_code, room, client_id: Uuid::new_v4()}
    }

    pub fn player(room: Addr<Room>, room_code: u32) -> Self{
        Self { last_hb: Instant::now(), is_host: false, room_code, room, client_id: Uuid::new_v4()}
    }

    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.last_hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WebsocketConnection {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let our_addr = ctx.address();
        self.room
        .send(Connect {
            addr: our_addr.recipient(),
            lobby_id: self.room,
            self_id: self.id,
        })
        .into_actor(self)
        .then(|res, _, ctx| {
            match res {
                Ok(_res) => (),
                _ => ctx.stop(),
            }
            fut::ready(())
        })
        .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.room.do_send(Disconnect { id: self.id, room_id: self.room });
        Running::Stop
    }
}



type WsInput = Result<ws::Message, ws::ProtocolError>;
/// Handler for `ws::Message`
impl StreamHandler<WsInput> for WebsocketConnection {
    fn handle(&mut self, msg: WsInput, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {msg:?}");
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_hb = Instant::now();
            }
            // Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Text(s)) => self.room.do_send(ClientActorMessage {
                id: self.client_id,
                msg: s,
                room_id: self.room
            }),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Nop) => {}
            Err(e) => {
                println!("{e}");
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}










#[derive(serde::Serialize, serde::Deserialize)]
struct Time;

#[derive(serde::Serialize, serde::Deserialize)]
enum ClientMessage{
    ///Locks the buzzer of the client.
    LockBuzzer,
    ///Sent to a client when they are kicked from the lobby.
    Kicked,
    TimerStart{
        start: Time,
    },
    ///Sent to the host to compare only after the response is checked to match the current round of the lobby.
    BuzzCompleted{
        ///When the buzzer was hit.
        at: Time,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
enum ServerMessage{
    LockBuzzers,
    ///This is to kick a certain user.
    Kick{
        ///Identifies which user to kick.
        username: String,
    },
    ///This is sent to the clients to start the time of the buzzer.
    TimerStart{
        start: Time,
    },
    ///This is sent to the host when a user buzzes. The host is meant to check against other BuzzCompleted signals for the shortest one.
    BuzzCompleted{
        ///When the buzzer was hit.
        at: Time,
        ///Which buzzer round it was sent to respond to (prevents time errors carrying into next rounds).
        response: u32,
    }
}
