use warp::ws::{Message};
use futures::channel::mpsc::{unbounded, UnboundedSender as Sender, UnboundedReceiver as Receiver};
use futures::{StreamExt};
use futures::channel::oneshot;

use super::msg;
use super::game;



pub async fn lobby_loop(mut rx: Receiver<super::msg::ToLobby>) {
    let mut queue: Vec<Sender<msg::FromLobby>> = vec![];


    while let Some(msg) = rx.next().await {
        println!("{}", queue.len());
        match msg {
            super::msg::ToLobby::Join(greet_tx) => {
                
                //if other user disconnected while in queue remove the user.
                println!("user joined");
                if queue.len() == 1 && queue[0].is_closed() {
                    queue.pop(); 
                    println!("hello");
                }

                //add new user to queue
                queue.push(greet_tx);

                //create a new game when there are enough players to create one
                if queue.len() == 2 {
                    let white_great = queue.pop().unwrap();
                    let black_great = queue.pop().unwrap();

                    let (to_black_tx, to_black_rx) = unbounded();
                    let (to_white_tx, to_white_rx) = unbounded();
                    let (from_black_tx, from_black_rx) = unbounded();
                    let (from_white_tx, from_white_rx) = unbounded();

                    white_great.unbounded_send(msg::FromLobby::Accepted(from_white_tx, to_white_rx));
                    black_great.unbounded_send(msg::FromLobby::Accepted(from_black_tx, to_black_rx));

                    tokio::task::spawn(game::run_game(to_black_tx, to_white_tx, from_black_rx, from_white_rx));
                    println!("created new game");
                }
            },
        }

    }
}
