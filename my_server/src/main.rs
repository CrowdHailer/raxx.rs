extern crate raxx;
use raxx::Server;
use raxx::SimpleServer;

use raxx::request;
use raxx::response;
use raxx::request::Method::*;
use raxx::response::Status::*;

pub struct HomePage {
}
pub trait MyIP {

}

impl SimpleServer for HomePage {
    type Channel = ();
    fn handle_request(&self, request: request::Message, _channel: Self::Channel) -> raxx::response::Message {
        match request.head.method {
            GET =>
                response(OK)
                    .set_body("GET".to_string()),
            _ =>
                response(BadRequest)
                    .set_body("".to_string()),
        }
    }
}

pub struct MyRouter<C> {
    channel: C
}

pub struct Match<Endpoint: SimpleServer> {
    inner: Endpoint,
    channel: Endpoint::Channel
}

pub enum Routes {
    A(Match<HomePage>)
}
impl<C> MyRouter<C> {
    fn route(ref request: &raxx::request::Message) -> Routes {
        match request.head.path.as_ref() {
            "/" =>
                Routes::A(Match{inner: HomePage{}, channel: ()}),
            _ =>
                unimplemented!()
                // response(NotFound)
                //     .set_body("Not Found".to_string()),
        }
    }
}

// TODO streaming server
impl<C: MyIP> SimpleServer for MyRouter<C> {
    type Channel = C;

    fn handle_request(&self, request: raxx::request::Message, _channel: Self::Channel) -> response::Message {

        // This hasn't really helped as I will need an implementation for every match
        match Self::route(&request) {
            Routes::A(Match{inner, channel}) =>
                inner.handle_request(request, channel)
        }
    }
}

impl MyIP for usize {

}

// Server IP

fn main() {
    let request = raxx::request(HEAD, "/".to_string());
    println!("{:?}", request);
    let server = MyRouter{channel: 3}.to_server();
    let server = raxx::middleware::head::Head::new(server);
    if let (_parts, Some(server)) = server.handle_head(request, 3) {
        if let (_parts, Some(server)) = server.handle_data("".to_string(), 3) {
            let (parts, _server) = server.handle_tail((), 3);
            println!("{:?}", parts);
        }
    }
}
