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
    // ///The username of the user.
    // client_name: String,
}

struct Room {
    clients: HashSet<Uuid>, //perhaps store clients as a HashMap<Uuid, Client> here and eliminate sessions?
    round: u32,
}

///Represents all the rooms and clients and everything in the system. 
pub struct Lobby {
    ///Represents all the clients, connecting their UUID and their ability to receive messages. Given a UUID, a client ID is returned. 
    sessions: HashMap<Uuid, Client>,
    ///Represents the rooms, retrievable via the code, giving access to all the clients' ids in the room.
    rooms: HashMap<u32, HashSet<Uuid>>,
}
impl Lobby {
    fn send_message(&self, message: &ClientMessage, client_id: &Uuid) {
        if let Some(Client { recipient: socket_recipient, .. }) = self.sessions.get(client_id) {
            let _ = socket_recipient
                .do_send(WsMessage { text: serde_json::to_string(message).unwrap() });
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
    fn broadcast(&self, message: ClientMessage, room_code: u32){
        let Some(our_room) = self.rooms.get(&room_code) else { return };
        our_room.iter().for_each(|client_id|self.send_message(&message, client_id))
    }
    fn broadcast_host(&self, message: ClientMessage, room_code: u32){
        self.broadcast_filter(message, room_code, |f|{
            if let Some(client) = self.sessions.get(f){
                client.is_host
            } else {
                false
            }
        })
    }
    fn broadcast_players(&self, message: ClientMessage, room_code: u32){
        self.broadcast_filter(message, room_code, |f|{
            if let Some(client) = self.sessions.get(f){
                !client.is_host
            } else {
                false
            }
        })
    }
    fn broadcast_filter(&self, message: ClientMessage, room_code: u32, mut filter: impl FnMut(&Uuid) -> bool){
        let Some(our_room) = self.rooms.get(&room_code) else { return };
        our_room.iter().filter(|&f|filter(f)).for_each(|client_id|self.send_message(&message, client_id))
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
    pub client_name: String,
    pub is_host: bool,
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {

        //only hosts can create a room
        if msg.is_host{
            self.rooms
                .entry(msg.room_code)
                .or_default().insert(msg.client_id);
        } else {
            msg.addr.do_send(WsMessage { text: serde_json::to_string(&ClientMessage::CodeNotFound).unwrap() })
        }

        self.broadcast(ClientMessage::AddUser{ client_name: msg.client_name, client_id: msg.client_id }, msg.room_code);

        // store the address
        self.sessions.insert(
            msg.client_id,
            Client { recipient: msg.addr, is_host: msg.is_host },
        );
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
        if let Some(_client) = self.sessions.remove(&msg.client_id) {

            self.broadcast(ClientMessage::RemoveUser { client_id: msg.client_id }, msg.room_code);

            if let Some(room) = self.rooms.get_mut(&msg.room_code) {
                if room.len() > 1 {
                    room.remove(&msg.client_id);
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

        // let Some(our_room) = self.rooms.get(&msg.room_code) else { return };

        match msg.msg{
            ServerMessage::LockBuzzers => {
                self.broadcast_players(ClientMessage::LockBuzzer, msg.room_code);
            },
            ServerMessage::Kick { uuid } => {
                self.send_message(&ClientMessage::Kicked, &uuid);
            },
            ServerMessage::StartTimer { start } => {
                self.broadcast_players(ClientMessage::StartTimer { start, round: todo!() }, msg.room_code)
            },
            ServerMessage::PauseTimer { at } => {
                self.broadcast_players(ClientMessage::PauseTimer { at }, msg.room_code)
            },
            ServerMessage::BuzzCompleted { at, response } => {
                // if response != self.rooms.get(&msg.room_code).round { return };
                todo!("check that response matches the round we are on");
                self.broadcast_host(ClientMessage::BuzzCompleted { at }, msg.room_code)
            },
        }        
    }
}
