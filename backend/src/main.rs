use futures::channel::mpsc::{unbounded, UnboundedSender as Sender};
use warp::Filter;

use msg;

mod lobby;
mod player;
mod game;

#[tokio::main]
async fn main() {
    //create channel between game loop and player_joined
    let (tx, rx) = unbounded();
    let tx = warp::any().map(move || tx.clone());
    tokio::task::spawn(lobby::lobby_loop(rx));

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let game_filter = warp::path("game")
        .and(warp::ws())
        .and(tx)
        .map(move |ws: warp::ws::Ws, tx: Sender<msg::ToLobby>| {
            ws.on_upgrade(|socket| player::player_joined(socket, tx))
        },);


    let index = warp::path("play")
        .and(warp::fs::dir("../static"));

    let routes = index.or(game_filter);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
