use futures::channel::mpsc::{unbounded, UnboundedSender as Sender};
use warp::Filter;

mod msg;
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


    let index = warp::get()
        .and(warp::fs::dir("frontend"));

    let routes = index.or(game_filter);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 10000))
        .await;
}
