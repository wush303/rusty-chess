use futures::channel::mpsc::{UnboundedReceiver as Receiver};
use futures::{StreamExt};
use chess::{Game as ChessGame, Color, ChessMove};
use std::str::FromStr;

use super::msg;

fn broadcast(black: &super::player::Player, white: &super::player::Player, msg: msg::FromGame) {
    black.unbounded_send(msg.clone()).unwrap();
    white.unbounded_send(msg).unwrap();
}

fn winner(black: &super::player::Player, white: &super::player::Player, who: Color) {
    match who {
        Color::White => black.unbounded_send(msg::FromGame::Win),
        Color::Black => white.unbounded_send(msg::FromGame::Win),
    }.unwrap();
}


pub async fn run_game(black: super::player::Player, white: super::player::Player, mut from_players: Receiver<super::msg::ToGameWrap>) {
    black.unbounded_send(msg::FromGame::Hello("b".to_string())).unwrap();
    white.unbounded_send(msg::FromGame::Hello("w".to_string())).unwrap();

    let mut game = ChessGame::new();
    let mut is_finished = false;

    loop {
        
        let msg::ToGameWrap(msg, who) =
            from_players.next().await.expect("expcted message from black");
        println!("{:?}", game.side_to_move());
        match msg {
            msg::ToGame::Disconnect => {
                //if the game hasn't finished the game will finish

                if !is_finished {
                    winner(&black, &white, who);
                    println!("game will close");
                    is_finished = true;
                } else {
                    println!("game will end");
                    break;
                }
            },
            msg::ToGame::MovePiece(from, to) => { 
                if let Ok(new_move) = ChessMove::from_str(&(from.to_string() + &to)) {
                    if game.side_to_move() == who {
                        if game.current_position().legal(new_move) {
                            game.make_move(new_move);
                            broadcast(&black, &white, msg::FromGame::NewMove{f: from.to_string(), t: to.to_string()});
                        } else {
                            //handle illegal move
                            let reason = format!("{:?} made an illegal move.", who);

                            println!("{}", &reason);

                            winner(&black, &white, who);

                            game.resign(who);
                            broadcast(&black, &white, msg::FromGame::Resign(reason));
                        }
                    } else {
                        //not the player who send the message's turn.
                        let reason = format!("It's not: {:?} turn. Games done", who);

                        println!("{}", &reason);

                        winner(&black, &white, who);

                        game.resign(who);
                        broadcast(&black, &white, msg::FromGame::Resign(reason));
                    }
                    
                } else {
                    //handle wrong formated move send from player
                    let reason = format!("Message from: {:?} was formated wrong. Games done", who);

                    println!("{}", reason);

                    winner(&black, &white, who);

                    game.resign(who);
                    broadcast(&black, &white, msg::FromGame::Resign(reason));
                }
            }
        }
    }
}
