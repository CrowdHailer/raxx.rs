extern crate raxx;
use raxx::server::Server;

use raxx::request::Method::*;
use raxx::response::Status::*;

pub struct MyServer {
}
pub trait MyIP {

}

impl<C> raxx::server::Simple<C> for MyServer {
    fn handle_request(&self, request: raxx::request::Message, _channel: C) -> raxx::response::Message {
        match request.head.method {
            GET =>
                raxx::response(OK)
                    .set_body("GET".to_string()),
            _ =>
                raxx::response(BadRequest)
                    .set_body("".to_string()),
        }
    }
}

// associated type
// Router match
// Routing should be in a tuple with core data on one side and route match in other.
// Server IP

fn main() {
    let request = raxx::request(HEAD, "/".to_string());
    println!("{:?}", request);
    let server = raxx::server::SimpleWrapper::new(MyServer{});
    let server = raxx::middleware::head::Head::new(server);
    if let (_parts, Some(server)) = server.handle_head(request, 4) {
        if let (_parts, Some(server)) = server.handle_data("".to_string(), 4) {
            let (parts, _server) = server.handle_tail((), 4);
            println!("{:?}", parts);
        }
    }
}
