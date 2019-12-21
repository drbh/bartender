use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use json::JsonValue;
use serde_derive::{Deserialize, Serialize};

use std::time::SystemTime;
use std::{env, process};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use nng::{Message, Protocol, Socket};


use std::io::Write;
use std::io::Read;
use std::str;

use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    trig: String,
    number: i32,
}


const DATE_REQUEST: u64 = 1;

// const ADDRESS: &'static str = "inproc://nng/example";
const ADDRESS: &'static str = "tcp://127.0.0.1:13131";

async fn index(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    let data = json!(item.0).to_string();
    let s = client(ADDRESS, &data).unwrap();
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(s) // <- send response
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .start()
    .await
}


/// Run the client portion of the program.
fn client(url: &str, data: &str) -> Result<String, nng::Error> {
    let s = Socket::new(Protocol::Req0)?;
    s.dial(url)?;
    // println!("CLIENT: SENDING DATE REQUEST");
    let mut req = Message::new()?;
    req.write_all(data.as_bytes()).unwrap();
    s.send(req)?;
    // println!("CLIENT: WAITING FOR RESPONSE");
    let msg = s.recv()?;
    let sparkle_heart = str::from_utf8(&msg.as_slice()).unwrap().to_owned();
    Ok(sparkle_heart)
}



// /// Run the server portion of the program.
// fn server(url: &str) -> Result<(), nng::Error> {
//     let s = Socket::new(Protocol::Rep0)?;
//     s.listen(url)?;

//     loop {
//         println!("SERVER: WAITING FOR COMMAND");
//         let mut msg = s.recv()?;

//         let cmd = msg.as_slice().read_u64::<LittleEndian>().unwrap();
//         if cmd != DATE_REQUEST {
//             println!("SERVER: UNKNOWN COMMAND");
//             continue;
//         }

//         println!("SERVER: RECEIVED DATE REQUEST");
//         let rep = SystemTime::now()
//             .duration_since(SystemTime::UNIX_EPOCH)
//             .expect("Current system time is before Unix epoch")
//             .as_secs();

//         msg.clear();
//         msg.write_u64::<LittleEndian>(rep).unwrap();

//         println!("SERVER: SENDING {}", rep);
//         s.send(msg)?;
//     }
// }



// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::dev::Service;
//     use actix_web::{http, test, web, App};

//     #[actix_rt::test]
//     async fn test_index() -> Result<(), Error> {
//         let mut app = test::init_service(
//             App::new().service(web::resource("/").route(web::post().to(index))),
//         )
//         .await;

//         let req = test::TestRequest::post()
//             .uri("/")
//             .set_json(&MyObj {
//                 name: "my-name".to_owned(),
//                 number: 43,
//             })
//             .to_request();
//         let resp = app.call(req).await.unwrap();

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let response_body = match resp.response().body().as_ref() {
//             Some(actix_web::body::Body::Bytes(bytes)) => bytes,
//             _ => panic!("Response error"),
//         };

//         assert_eq!(response_body, r##"{"name":"my-name","number":43}"##);

//         Ok(())
//     }
// }