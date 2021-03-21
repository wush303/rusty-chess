use futures::channel::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

use super::chess;
use super::game;

pub enum ToLobby {
    Join(Sender<super::msg::FromLobby>),
}

pub enum FromLobby {
    Accepted(Sender<super::msg::ToGame>, Receiver<super::msg::FromGame>),
    Rejected,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ToGame {
    MovePiece(game::Move),
    Disconnect,
}

#[derive(Clone)]
pub enum FromGame {
    Hello(String),
    Fen(String),
    Disconnect,
}
