use futures::channel::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};
use chess::{Board, Color, ChessMove, Game as ChessGame};

use super::game;

pub enum ToLobby {
    Join(Sender<super::msg::FromLobby>),
}

pub enum FromLobby {
    Accepted(Sender<ToGameWrap>, Receiver<super::msg::FromGame>, chess::Color),
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ToGame {
    MovePiece(u8, u8),
    Disconnect,
}

pub struct ToGameWrap(pub ToGame, pub chess::Color);

#[derive(Clone)]
pub enum FromGame {
    Hello(String),
    NewBoard(Board),
    Disconnect,
    Win,
}
