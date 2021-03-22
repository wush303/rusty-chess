use futures::channel::mpsc::{UnboundedReceiver as Receiver};
use futures::{StreamExt};
use tokio::select;
use serde::{Deserialize, Serialize};

use super::msg;
use super::chess;


#[derive(Serialize, Deserialize, Debug)]
pub struct Move((usize, usize), (usize, usize));

fn check_move(game_move: &Move, board: &super::chess::Game, color: super::chess::Color) -> bool {
    true
}

fn make_move(game_move: Move, board: &mut super::chess::Game) {
    let Move((from_x, from_y), (to_x, to_y)) = game_move;
    let piece = board.board[from_x][from_y];
    board.board[from_x][from_y] = super::chess::Field::Empty;
    board.board[to_x][to_y] = piece;
    
}



fn broadcast(black: &super::player::Player, white: &super::player::Player, msg: msg::FromGame) {
    black.unbounded_send(msg.clone());
    white.unbounded_send(msg);
}

pub async fn run_game(black: super::player::Player, white: super::player::Player, mut from_players: Receiver<super::msg::ToGame>) {
    black.unbounded_send(msg::FromGame::Hello("black".to_string()));
    white.unbounded_send(msg::FromGame::Hello("white".to_string()));


    let mut board = chess::Game { board: chess::INIT_BOARD, };
    println!("{}", board.get_fen());

    let mut turn = super::chess::Color::White;


    let mut is_finished = false;

    loop {
        match from_players.next().await.expect("expcted message from black") {
            msg::ToGame::Disconnect(who) => {
                //if the game hasn't finished the game will finish
                if !is_finished {
                    match who {
                        super::chess::Color::White => black.unbounded_send(msg::FromGame::Win),
                        super::chess::Color::Black => white.unbounded_send(msg::FromGame::Win),
                    };
                    is_finished = true;
                } else {
                    println!("game will end");
                    break;
                }
            },
            msg::ToGame::MovePiece(m, who) => { 
                println!("{:?} moved a piece {:?}", who, m);
                let is_legal = check_move(&m, &board, turn);
                turn = super::chess::Color::Black;
                if is_legal {
                    make_move(m, &mut board);
                    println!("{}", board.get_fen());
                    broadcast(&black, &white, msg::FromGame::Fen(board.get_fen()));
                }
            }
            _ => println!("fuck"),
        }
    }
}
