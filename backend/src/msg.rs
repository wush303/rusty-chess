use futures::channel::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use serde::{Deserialize, Serialize};

pub enum ToLobby {
    Join(Sender<FromLobby>),
}

pub enum FromLobby {
    Accepted(Sender<ToGameWrap>, Receiver<FromGame>, chess::Color),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ToGame {
    MovePiece(String, String),
    Disconnect,
}

pub struct ToGameWrap(pub ToGame, pub chess::Color);

#[derive(Clone, Serialize)]
pub enum FromGame {
    Hello(String),
    NewMove { f: String, t: String },
    Win,
    Resign(String),
}
