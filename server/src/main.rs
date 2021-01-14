mod ws;
mod lobby;
use lobby::Lobby;
mod messages;
mod start_connection;
use start_connection::start_connection as start_connection_route;
use actix::Actor;
use actix_web::{ HttpServer, App };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start();

    HttpServer::new(move || {
        App::new()
            .service(start_connection_route)
            .data(chat_server.clone())
    })
    .bind("127.0.0.1:5000")? 
    .run()
    .await
}

// todo:
//  Continuation requests
//  Error handling
//  Reddis database for lobbies
//  Name to lobbies
//  
