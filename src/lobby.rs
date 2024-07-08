
//NOTE: YOU ARE IN THE WRONG BRANCH
use actix::prelude::{Actor, Context, Handler, Recipient, Message};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::server::WebsocketConnection;



struct Client{ //not an actor, but a component of an actor
    is_host: bool,
}


pub struct Room{
    room_code: u32,
}
impl Actor for Room{
    type Context = Context<Self>;
}







impl Handler<WsMessage> for WebsocketConnection {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

//WsConn responds to this to pipe it through to the actual client
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub lobby_id: Uuid,
    pub self_id: Uuid,
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: Uuid,
    pub id: Uuid,
}

//client sends this to the lobby for the lobby to echo out.
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid
}