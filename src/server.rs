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
    lobby: Addr<Lobby>,
    room_code: u32,
    client_id: Uuid,
    client_name: String, //replace with Arc<str>
}

impl WebsocketConnection {

    pub fn host(lobby: Addr<Lobby>, room_code: u32, client_name: String, client_id: Uuid) -> Self{
        Self { last_hb: Instant::now(), is_host: true, room_code, lobby, client_id, client_name }
    }

    pub fn player(lobby: Addr<Lobby>, room_code: u32, client_name: String, client_id: Uuid) -> Self{
        Self { last_hb: Instant::now(), is_host: false, room_code, lobby, client_id, client_name}
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
        self.lobby
        .send(Connect {
            addr: our_addr.recipient(),
            room_code: self.room_code,
            client_id: self.client_id,
            is_host: self.is_host,
            client_name: self.client_name.clone(),
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
        self.lobby.do_send(Disconnect { client_id: self.client_id, room_code: self.room_code });
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
            Ok(ws::Message::Text(s)) => {
                let msg = match ServerMessage::create(s.to_string()) {
                    Ok(msg) => msg,
                    Err(e) => {
                        println!("Error {e}");
                        println!("The message intended for the Server could not be received.");
                        return;
                    },
                };

                self.lobby.do_send(LobbyMessage {
                    client_id: self.client_id,
                    msg,
                    room_code: self.room_code,
                })
            },
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

//WsConn responds to this to pipe it through to the actual client
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage{
    pub text: String,
}

impl Handler<WsMessage> for WebsocketConnection {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.text)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Nanos(pub u128);
//use Duration or Nanos










#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClientMessage{
    ///Locks the buzzer of the client.
    LockBuzzer,
    ///Sent to a client when they are kicked from the lobby.
    Kicked,
    StartTimer{
        start: Nanos,
        ///Keeps track of which round the buzzer was started on.
        round: u32
    },
    PauseTimer{
        at: Nanos,
    },
    ///Sent to the host to compare only after the response is checked to match the current round of the lobby.
    BuzzCompleted{
        ///When the buzzer was hit.
        at: Nanos,
    },
    AddUser{
        client_name: String,
        client_id: Uuid,
    },
    RemoveUser{
        client_id: Uuid,
    },
    ///A message for when the code is not found in a room.
    CodeNotFound,
    NewCode{
        code: u32,
    },
}
// impl ClientMessage{
//     fn is_for_host(&self) -> bool{
//         match self{
//             ClientMessage::LockBuzzer => false,
//             ClientMessage::Kicked => false,
//             ClientMessage::StartTimer { .. } => false,
//             ClientMessage::PauseTimer { .. } => false,
//             ClientMessage::BuzzCompleted { .. } => true,
//         }
//     }
// }
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ServerMessage{
    ///This is to lock all the buzzers.
    LockBuzzers,
    // ///This is to enable all the buzzers back again.
    // ClearBuzzers,
    ///This is to kick a certain user.
    Kick{
        ///Identifies which user to kick.
        uuid: Uuid,
    },
    ///This is sent to the clients to start the time of the buzzer. Every time it is sent by the host, make sure to increment the response.
    StartTimer{
        start: Nanos,
    },
    PauseTimer{
        at: Nanos,
    },
    ///This is sent to the host when a user buzzes. The host is meant to check against other BuzzCompleted signals for the shortest one.
    BuzzCompleted{
        ///When the buzzer was hit.
        at: Nanos,
        ///Which buzzer round it was sent to respond to (prevents time errors carrying into next rounds).
        response: u32,
    }
}
impl ServerMessage{
    fn create(s: String) -> Result<Self, serde_json::Error>{
        serde_json::from_str(&s)
    }
}