use actix::prelude::{Actor, Context, Handler, Recipient, Message};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use crate::server::*;



///This is a data component of the Lobby actor.
struct Client{
    ///Represents the physical socket connection that can actually accept the message.
    recipient: Recipient<WsMessage>,
    ///Whether the client is a host or not. The types of messages received differ.
    is_host: bool,
}

///Represents all the rooms and clients and everything in the system. 
pub struct Lobby {
    ///Represents all the clients, connecting their UUID and their ability to receive messages. Given a UUID, a client ID is returned. 
    sessions: HashMap<Uuid, Client>,
    ///Represents the rooms, retrievable via the code, giving access to all the clients' ids in the room.
    rooms: HashMap<u32, HashSet<Uuid>>,
}
impl Lobby {
    fn send_message(&self, message: ClientMessage, client_id: &Uuid) {
        if let Some(Client { recipient: socket_recipient, .. }) = self.sessions.get(client_id) {
            let _ = socket_recipient
                .do_send(WsMessage(serde_json::to_string(&message).unwrap()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}
impl Actor for Lobby {
    type Context = Context<Self>;
}




//WsConn sends this to the lobby to say "put me in please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub room_code: u32,
    pub client_id: Uuid,
    pub is_host: bool,
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // create a room if necessary, and then add the id to it
        self.rooms
            .entry(msg.room_code)
            .or_insert_with(HashSet::new).insert(msg.client_id);

        // send to everyone in the room that new uuid just joined
        self
            .rooms
            .get(&msg.room_code)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.client_id)
            .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.client_id), conn_id));

        // store the address
        self.sessions.insert(
            msg.client_id,
            Client { recipient: msg.addr, is_host: msg.is_host },
        );

        // send self your new uuid
        self.send_message(&format!("your id is {}", msg.client_id), &msg.client_id);
    }
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_code: u32,
    pub client_id: Uuid,
}
/// Handler for Disconnect message.
impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.client_id).is_some() {
            self.rooms
                .get(&msg.room_code)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.client_id)
                .for_each(|user_id| self.send_message(&format!("{} disconnected.", &msg.client_id), user_id));
            if let Some(lobby) = self.rooms.get_mut(&msg.room_code) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.client_id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_code);
                }
            }
        }
    }
}


//client sends this to the lobby for the lobby to echo out.
#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyMessage {
    pub client_id: Uuid,
    pub msg: ServerMessage,
    pub room_code: u32,
}

impl Handler<LobbyMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: LobbyMessage, _: &mut Context<Self>) -> Self::Result {

        match msg.msg{
            ServerMessage::LockBuzzers => {
                self.rooms.get(&msg.room_code).unwrap().iter().for_each(|client| self.send_message(ClientMessage::LockBuzzer, client));
            },
            ServerMessage::Kick { username } => todo!(),
            ServerMessage::StartTimer { start } => todo!(),
            ServerMessage::PauseTimer { at } => todo!(),
            ServerMessage::BuzzCompleted { at, response } => todo!(),
        }

        // self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
        
        
    }
}
