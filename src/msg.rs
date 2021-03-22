use futures::channel::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};

use super::chess;
use super::game;

pub enum ToLobby {
    Join(Sender<super::msg::FromLobby>),
}

pub enum FromLobby {
    Accepted(Sender<super::msg::ToGame>, Receiver<super::msg::FromGame>, super::chess::Color),
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ToGame {
    MovePiece(game::Move, super::chess::Color),
    Disconnect(super::chess::Color),
}

#[derive(Clone)]
pub enum FromGame {
    Hello(String),
    Fen(String),
    Disconnect,
    Win,
}
