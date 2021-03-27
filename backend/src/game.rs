use futures::channel::mpsc::{UnboundedReceiver as Receiver};
use futures::{StreamExt};
use tokio::select;
use serde::{Deserialize, Serialize};
use chess::{Game as ChessGame, Color, Square, ChessMove};

use super::msg;

fn safe_make_square(sq: u8) -> Option<Square> {
    if sq < 64 {
        Some(unsafe { Square::new(sq) })
    } else {
        None
    }
}

fn make_move(game_move: ChessMove, board: &mut ChessGame) {

}



fn broadcast(black: &super::player::Player, white: &super::player::Player, msg: msg::FromGame) {
    black.unbounded_send(msg.clone());
    white.unbounded_send(msg);
}

pub async fn run_game(black: super::player::Player, white: super::player::Player, mut from_players: Receiver<super::msg::ToGameWrap>) {
    black.unbounded_send(msg::FromGame::Hello("black".to_string()));
    white.unbounded_send(msg::FromGame::Hello("white".to_string()));


    let mut game = ChessGame::new();
    //let mut board = chess::Game { board: chess::INIT_BOARD, };
    //println!("{}", board.get_fen());

    let mut turn = Color::White;


    let mut is_finished = false;

    loop {
        
        let msg::ToGameWrap(msg, who) =
            from_players.next().await.expect("expcted message from black");
        println!("{:?}", game.side_to_move());
        match msg {
            msg::ToGame::Disconnect => {
                //if the game hasn't finished the game will finish
                if !is_finished {
                    match who {
                        Color::White => black.unbounded_send(msg::FromGame::Win),
                        Color::Black => white.unbounded_send(msg::FromGame::Win),
                    };
                    is_finished = true;
                } else {
                    println!("game will end");
                    break;
                }
            },
            msg::ToGame::MovePiece(from, to) => { 
                //println!("{:?} moved a piece {:?}", who, from);
                //println!("{:?}", game.side_to_move());
                if let (Some(from), Some(to)) = (safe_make_square(from), safe_make_square(to)) {
                    if game.side_to_move() == who {
                        let new_move = ChessMove::new(from, to, None);
                        if game.current_position().legal(new_move) {
                            game.make_move(new_move);
                            broadcast(&black, &white, msg::FromGame::NewBoard(game.current_position()));
                            println!("message is good");
                        } else {
                            println!("illegal move");
                            //handle illegal move
                        }
                    } else {
                        //not the player who send the message's turn.
                        println!("")
                    }
                    
                } else {
                    //handle wrong formated move send from player
                    println!("message is bad!");
                }
            }
            _ => println!("fuck"),
        }
    }
}
