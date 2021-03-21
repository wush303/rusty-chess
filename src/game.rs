use futures::channel::mpsc::{UnboundedReceiver as Receiver};
use futures::{StreamExt};
use tokio::select;

use super::msg;
use super::chess;


pub type Move = ((usize, usize), (usize, usize));

fn check_move(game_move: Move, board: &super::chess::Game, color: super::chess::Color) -> bool {
    true
}

fn make_move(game_move: Move, board: &mut super::chess::Game) {
    let ((from_x, from_y), (to_x, to_y)) = game_move;
    let piece = board.board[from_x][from_y];
    board.board[to_x][to_y] = piece;
}



fn broadcast(black: &super::player::Player, white: &super::player::Player, msg: msg::FromGame) {
    black.unbounded_send(msg.clone());
    white.unbounded_send(msg);
}

pub async fn run_game(black: super::player::Player, white: super::player::Player, mut from_black: Receiver<super::msg::ToGame>, mut from_white: Receiver<super::msg::ToGame>) {
    black.unbounded_send(msg::FromGame::Hello("black".to_string()));
    white.unbounded_send(msg::FromGame::Hello("white".to_string()));

    let mut board = chess::Game { board: chess::INIT_BOARD, };
    println!("{}", board.get_fen());

    let mut turn = super::chess::Color::White;



    loop {
        select! {
            msg = from_black.next() => {
                match msg.expect("expcted message from black") {
                    msg::ToGame::Disconnect => {
                        println!("black disconnected, game will close");
                        white.unbounded_send(msg::FromGame::Disconnect);
                        break
                    },
                    msg::ToGame::MovePiece(m) if turn == super::chess::Color::White => { 
                        println!("white moved a piece {:?}", m);
                        let is_legal = check_move(m, &board, turn);
                        turn = super::chess::Color::Black;
                        if is_legal {
                            make_move(m, &mut board);
                            broadcast(&black, &white, msg::FromGame::Fen(board.get_fen()));
                        }
                    }
                    _ => println!("fuck"),
                }
            }
            msg = from_white.next() => {
                match msg.expect("expect message from white") {
                    msg::ToGame::Disconnect => {
                        println!("white disconnectedd, game will close");
                        black.unbounded_send(msg::FromGame::Disconnect);
                        break
                    },
                    msg::ToGame::MovePiece(m) if turn == super::chess::Color::Black => {
                        println!("white moved a piece {:?}", m);
                        let is_legal = check_move(m, &board, turn);
                        turn = super::chess::Color::White;
                        if is_legal {
                            make_move(m, &mut board);
                            broadcast(&black, &white, msg::FromGame::Fen(board.get_fen()));
                        }
                    },
                    _ => println!("fuck"),
                }
            }
        }
    }
}
