extern crate raxx;
use raxx::SimpleServer;

use raxx::request;
use raxx::response;
use raxx::request::Method::*;
use raxx::response::Status::*;

pub struct MyApp;
impl SimpleServer for MyApp {
    type Channel = ();
    fn handle_request(&self, _request: request::Message, _channel: ()) -> response::Message {
        response(OK)
            .set_header("content-lenth".to_string(), "text/plain".to_string())
            .set_body("Hello, World!".to_string())
    }
}

struct HelloPeer<Channel> { _channel: std::marker::PhantomData<Channel> }

impl<Channel> HelloPeer<Channel> {
    pub fn new() -> Self {
        Self{_channel: std::marker::PhantomData}
    }
}
impl<Channel: ClientIP> SimpleServer for HelloPeer<Channel> {
    type Channel = ();
    fn handle_request(&self, _request: request::Message, channel: ()) -> response::Message {
        response(OK)
            .set_header("content-lenth".to_string(), "text/plain".to_string())
            .set_body(format!("Hello, {}!", channel.client_ip()))
    }
}

trait ClientIP {
    fn client_ip(&self) -> String;
}

impl ClientIP for () {
    fn client_ip(&self) -> String {
        "127.0.0.1".to_string()
    }
}

fn main() {
    let request = request(GET, "http://example.me/".to_string()).set_body("".to_string());
    let server: HelloPeer<()> = HelloPeer::new();
    let response = server.handle_request(request, ());
    println!("{:?}", response);
}
