extern crate tmux_interface;
use crate::tmux_interface::{TmuxInterface, NewSession};

use actix_web::{
 middleware, web, App,  HttpResponse, HttpServer,
};
use serde_derive::{Deserialize, Serialize};
use nng::{Message, Protocol, Socket};
use std::io::Write;
use std::str;
use std::sync::Mutex;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct GatewayRequest {
    trig: String,
    number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceItem {
    key: String,
    loc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ApplicationData {
    counter: usize,
    locations: HashMap<String, String>
}

async fn index(item: web::Json<GatewayRequest>, app_dat: web::Data<Mutex<ApplicationData>>) -> HttpResponse {
    println!("model: {:?}", &item.trig);
    let data = json!(item.0).to_string();
    let app_dat = app_dat.lock().unwrap();

    match app_dat.locations.get(&item.trig) {
        Some(loc) => {

    let s = client(loc, &data).unwrap();
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(s) // <- send response

        },
        None =>  HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body("service not found") // <- send response
    }



}

async fn config(item: web::Json<ServiceItem>, app_dat: web::Data<Mutex<ApplicationData>>) -> HttpResponse {
    let mut app_dat = app_dat.lock().unwrap();
    app_dat.locations.insert(item.key.clone(), item.loc.clone());
    println!("{:#?}", app_dat);
    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body("s") // <- send response
}

async fn list(app_dat: web::Data<Mutex<ApplicationData>>) -> HttpResponse {
    let app_dat = app_dat.lock().unwrap();


    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("session_name"),
        shell_command: Some("python3 /Users/davidholtz/Desktop/bartender/funcs/funca.py"),
        ..Default::default()
    };
    tmux.new_session(&new_session).unwrap();

// println!("{:?}", new_session);
// tmux.kill_session(None, None, Some("session_name")).unwrap();



    HttpResponse::Ok()
        .json(app_dat.clone()) // <- send response
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    let data = web::Data::new(Mutex::new(ApplicationData{counter:0, locations: HashMap::new()}));
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/").route(web::post().to(index)))
            .service(web::resource("/list").route(web::get().to(list)))
            .service(web::resource("/config").route(web::post().to(config)))
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