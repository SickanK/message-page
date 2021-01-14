use crate::ws::WsConn;
use crate::lobby::Lobby;
use actix::Addr;
use actix_web::{
    HttpResponse, 
    HttpRequest,
    Error,
    get,
    web::{ Path, Payload, Data }
};
use actix_web_actors::ws;
use uuid::Uuid;
    
#[get("/{group_id}")]
pub async fn start_connection(
        req: HttpRequest,
        stream: Payload,
        Path(group_id): Path<Uuid>,
        srv: Data<Addr<Lobby>>,
    ) -> Result<HttpResponse, Error> {
    let ws = WsConn::new(
            group_id,
            srv.get_ref().clone()
        );

    println!("Connection recieved");

    let resp = ws::start(ws, &req, stream)?;

    Ok(resp)
}

//Note
// https://medium.com/@romaric.mourgues/how-we-improved-our-websockets-by-removing-all-database-access-83d4e356da8e

