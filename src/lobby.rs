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

#[derive(Default)]
struct Room {
    clients: HashSet<Uuid>, //perhaps store clients as a HashMap<Uuid, Client> here and eliminate sessions?
    round: u32,
}
impl std::ops::Deref for Room{
    type Target = HashSet<Uuid>;

    fn deref(&self) -> &Self::Target {
        &self.clients
    }
}
impl std::ops::DerefMut for Room{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.clients
    }
}

///Represents all the rooms and clients and everything in the system. 
#[derive(Default)]
pub struct Lobby {
    ///Represents all the clients, connecting their UUID and their ability to receive messages. Given a UUID, a client ID is returned. 
    sessions: HashMap<Uuid, Client>,
    ///Represents the rooms, retrievable via the code, giving access to all the clients' ids in the room.
    rooms: HashMap<u32, Room>,
}
impl Lobby {
    fn send_message(&self, message: &ClientMessage, client_id: &Uuid) {
        if let Some(Client { recipient: socket_recipient, .. }) = self.sessions.get(client_id) {
            socket_recipient
                .do_send(WsMessage { text: serde_json::to_string(message).unwrap_or("\"SerdeError\"".to_string()) });
        } else {
            println!("attempting to send message but couldn't find user id.");
            println!("{}", serde_json::to_string(message).unwrap_or("\"SerdeError\"".to_string()));
        }
    }
    fn broadcast(&self, message: ClientMessage, room_code: u32){
        let Some(our_room) = self.rooms.get(&room_code) else { return };
        our_room.iter().for_each(|client_id|self.send_message(&message, client_id))
    }
    fn broadcast_others(&self, message: ClientMessage, room_code: u32, our_id: &Uuid){
        self.broadcast_filter(message, room_code, |f|{
            f != our_id
        })
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
    fn host_present(&self, room_code: &u32) -> bool{
        match self.rooms.get(room_code) {
            Some(room) => room.clients.iter().map(|id|self.sessions.get(id).map(|client|client.is_host).unwrap_or(false)).reduce(|a,b|a||b).unwrap_or(false),
            None => false,
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
    pub client_name: String,
    pub is_host: bool,
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, mut msg: Connect, _: &mut Context<Self>) -> Self::Result {

        if self.sessions.contains_key(&msg.client_id){
            msg.client_id = Uuid::new_v4();
            msg.addr.do_send(WsMessage { text: "\"Kicked\"".to_string() });
        }

        //only hosts can create a room
        if msg.is_host{
            //If there are other hosts in the room, kick this one. (I can disable this feature)
            if self.host_present(&msg.room_code){
                msg.addr.do_send(WsMessage { text: "\"Kicked\"".to_string() });
            }
            //Create the room if it doesn't exist and join it.
            self.rooms
                .entry(msg.room_code)
                .or_default().insert(msg.client_id);
        } else {
            match self.rooms.get_mut(&msg.room_code){
                Some(room) => {
                    if room.clients.iter().map(|id|self.sessions.get(id).map(|client|client.is_host).unwrap_or(false)).reduce(|a,b|a||b).unwrap_or(false){
                        room.insert(msg.client_id);
                    }
                },
                None => {
                    println!("Odd happening!");
                    msg.addr.do_send(WsMessage { text: "\"CodeNotFound\"".to_string() })
                },
            }
            self.broadcast_others(ClientMessage::AddUser{ client_name: msg.client_name, client_id: msg.client_id }, msg.room_code, &msg.client_id);
        }

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
        if let Some(client) = self.sessions.remove(&msg.client_id) {

            self.broadcast_others(ClientMessage::RemoveUser { client_id: msg.client_id }, msg.room_code, &msg.client_id);

            if client.is_host {
                self.broadcast_others(ClientMessage::Kicked, msg.room_code, &msg.client_id);
                let Some(room) = self.rooms.get_mut(&msg.room_code) else {return}; 
                room.clear();
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
                let Some(client) = self.sessions.get(&msg.client_id) else { return };
                if !client.is_host{
                    return;
                }
                self.send_message(&ClientMessage::Kicked, &uuid);
            },
            ServerMessage::StartTimer { start } => {
                let Some(room) = self.rooms.get_mut(&msg.room_code) else { return };
                room.round += 1;
                let round = room.round;
                self.broadcast_players(ClientMessage::StartTimer { start, round }, msg.room_code)
            },
            ServerMessage::PauseTimer { at } => {
                self.broadcast_players(ClientMessage::PauseTimer { at }, msg.room_code)
            },
            ServerMessage::BuzzCompleted { at, response } => {
                let Some(room) = self.rooms.get(&msg.room_code) else { return };
                if response != room.round {
                    println!("Rounds did not match!"); 
                    return 
                };
                self.broadcast_host(ClientMessage::BuzzCompleted { at }, msg.room_code)
            },
        }        
    }
}
