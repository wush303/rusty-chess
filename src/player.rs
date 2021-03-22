use futures::{FutureExt, StreamExt};
use futures::channel::oneshot;
use tokio::sync::{mpsc};
use warp::ws::{Message, WebSocket};
use uuid::Uuid;
use futures::SinkExt;
use tokio_stream::wrappers::UnboundedReceiverStream;
use futures::channel::mpsc::{unbounded, UnboundedSender as Sender};
use tokio::select;

use super::msg;


pub type Player = Sender<msg::FromGame>;


pub async fn player_joined(ws: WebSocket, mut to_lobby: Sender<super::msg::ToLobby>) {
    // Split the socket into a sender and receive of messages.
    let (player_ws_tx, mut player_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (sender, receiver) = mpsc::unbounded_channel();
    let receiver = UnboundedReceiverStream::new(receiver);
    tokio::task::spawn(receiver.forward(player_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    //greet message for answering the join request
    let (greet_tx, mut greet_rx) = unbounded();

    //send join message to game
    to_lobby
        .send(super::msg::ToLobby::Join(greet_tx))
        .await;

    //get answer to join request
    let mut greet_answer = None;

    loop {
        select! {
            gr = player_ws_rx.next() => {
                let gr = match gr.unwrap() {
                    Ok(gr) => gr,
                    Err(e) => {
                        eprintln!("websocket error: {}", e);
                        break;
                    }
                };
                if gr.is_close() {
                    break;
                }            
            }
            gr = greet_rx.next() => {
                greet_answer = match gr.expect("Receive answer to join request") {
                    msg::FromLobby::Accepted(to_game, from_game, turn) => Some((to_game, from_game, turn)),
                    msg::FromLobby::Rejected => None,
                };
                break;
            }
            
        }
    }

    //tick with each new message:
    while let Some((ref to_game, ref mut from_game, me)) = greet_answer {
        select! {
            msg = player_ws_rx.next() => {
                let msg = match msg.unwrap() {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("websocket error: {}", e);
                        break;
                    }
                };

                //player loop
                //to_game.unbounded_send(msg::ToGame::MovePiece("Hello".to_string()));
               if msg.is_text() {
                    //parse the message because it's text
                   match serde_json::from_str(msg.to_str().unwrap()) {
                       Ok(m) => {
                           match m {
                               msg::ToGame::MovePiece(mo, _) =>
                                   to_game.unbounded_send(msg::ToGame::MovePiece(mo, me)).expect("send move piece to game"),
                               _ => 
                                   to_game.unbounded_send(m).expect("send message to game"),
                           }
                       },
                       Err(e) => eprintln!("error: {}", e),
                   }                    
                } else if msg.is_close() {
                    //to_game.unbounded_send(msg::ToGame::Disconnect);
                    break;
                } else if msg.is_binary() {
                    println!("message is binary, client will disconnect");
                    //to_game.unbounded_send(msg::ToGame::Disconnect);
                    break;
                } else if msg.is_ping() {
                    println!("user pinged");
                }             
            }
            
            msg = from_game.next() => {
                match msg.expect("expected this") {
                    msg::FromGame::Hello(id) => {
                        sender.send(Ok(Message::text(id)));
                        //println!("hejsa ");
                        
                       //to_game.unbounded_send(msg::ToGame::Disconnect);
                    }
                    msg::FromGame::Fen(f) => {
                        sender.send(Ok(Message::text(f)));
                    }
                    msg::FromGame::Disconnect => {
                        //break when server closes
                        break;
                    }
                    msg::FromGame::Win => {
                        println!("{:?} wins", me);
                    }
                }
            }
        
        }
    }

    //if game hasn't closed, close the connection
    match greet_answer {
        Some((mut to_game, _, me)) => {
            to_game
                .unbounded_send(super::msg::ToGame::Disconnect(me)).expect("what")
        },
        None => println!("user left the queue"),
    }
}

