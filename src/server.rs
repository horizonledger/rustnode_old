
//TODO 404 not found redirect
// https://stackoverflow.com/questions/70635118/redirect-or-show-html-with-warp-in-rust

//TODO new function for connection
//TODO js file
//TODO handle disconnect
//TODO new class for peer
//TODO simple tx 
//TODO signatures
//TODO map of peers/conns
//TODO set name abilitiy (pubkey, DNS)
//TODO broadcast
//TODO pubsub
//TODO handle bad json message
//DONE simple webserver



//#![allow(non_snake_case)]


use std::{net::TcpListener, thread::spawn};
use std::net::SocketAddr;
use std::str::FromStr;

use simple_logger::SimpleLogger;
//use tungstenite::protocol::WebSocket;
//use tokio::net::TcpStream;

extern crate serde;
extern crate serde_json;

use warp::Filter;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    #[serde(rename = "type")]
    msg_type: String,
    value: String,
}

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

// fn handle_message(ws: &mut WebSocket<TcpStream>) {

// }

// fn handleMessage(message: Message, ws: &mut WebSocket) {
//     if message.msg_type == "chat" {
//         println!("chat : {:?}", message);
//         websocket.write_message(message.value.into()).unwrap();
//     }

//     else if message.msg_type == "name" {
//         println!("name {:?}", message);
//         websocket.write_message("registered".into()).unwrap();
//     }
// }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


async fn run_websocket_server(host: String, port: u16) {
    
    let addr = format!("{}:{}", host, port);
    let server = TcpListener::bind(addr.clone()).unwrap();
    log::info!("listening on {}", addr);
    //let srv = socket.incoming().for_each(move |(tcpstream, addr)| {
    for stream in server.incoming() {
        spawn(move || {            
            let callback = |req: &Request, response: Response| {
                log::info!("Received a new ws handshake");
                log::info!("The request's path is: {}", req.uri().path());
                // log::info!("The request's headers are:");
                // for (ref header, _value) in req.headers() {
                //     log::info!("* {}", header);
                // }

                // Let's add an additional header to our response to the client.
                //let headers = response.headers_mut();
                //headers.append("MyCustomHeader", ":)".parse().unwrap());
                //headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                Ok(response)
            };
            let mut ws = accept_hdr(stream.unwrap(), callback).unwrap();

            print_type_of(&ws);

            loop {
                let msg = ws.read_message().unwrap();
                if msg.is_binary() || msg.is_text() {
                    log::info!("msg: {}", msg);

                    // let msg_string = match msg {
                    //     tungstenite::Message::Text(s) => { s }
                    //     _ => { panic!() }
                    // };

                    //generic
                    //let parsedMsg: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
                    //println!("msg: {:?}", parsed["type"]);

                    //deal with bad parsing
                    //let message: Message = serde_json::from_str(&msg_string).unwrap();

                    //handle_message(&mut ws)

                    
                    //websocket.write_message("test from server".into()).unwrap();
                }
            }
        });
    }
}



#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("start websocket server");
    let ws_port = 3012;
    let host = "127.0.0.1";
    run_websocket_server(host.to_owned(), ws_port);

    log::info!("start HTTP server");

    // let http_route = warp::path!("block" / String).and_then(|block_hash: String| async move {

    // })

    let hello = warp::path!("hello" / String) 
        .map(|name| format!("Hello, {}!", name));

    //let static_route = warp::path("/").and(warp::fs::dir("static"));        
    let index_route = warp::path::end().and(warp::fs::dir("static"));        

    //let test = warp::path("test").and(warp::fs::file("test.txt"));
    //let test = warp::path("client.js").and(warp::fs::file("client.js"));

    let static_files = warp::path("static")
        .and(warp::fs::dir("static"));

    // let root = warp::path::end().map(|| {
    //     log::info("map request")
    //     match fs::read_to_string("index.html") {
    //         Ok(body) => {
    //             //a_response(200, "text/html; charset=utf-8", &body)
    //         },
    //         Err(_) => {
    //             //a_redirect(303, "/500")
    //         }
    //     }
    // });

    //let routes = hello.or(static_route).or(root);
    let routes = hello.or(index_route).or(static_files);

    let port = 8080;
    let host = "127.0.0.1";

    let addr = format!("{}:{}", host, port);
    let address = SocketAddr::from_str(&addr.clone()).unwrap();

    warp::serve(routes).run(address).await;
    
}