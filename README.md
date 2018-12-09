### Version 1

I think that I should not be defining traits without self?
Also choice of parameterised vs internal State type
```rs
pub mod raxx {
    #[derive(Debug)]
    pub enum Method {GET, POST}

    #[derive(Debug)]
    pub struct Request {
        pub method: Method
    }
    #[derive(Debug, PartialEq)]
    pub struct Response {
        pub body: String
    }
    pub trait Server<State> {
        fn handle_request(request: Request, state: State) -> Response;
    }

    pub fn request() -> i32 {
        2
    }
}

pub struct MyServer {

}

impl raxx::Server<i32> for MyServer {
    fn handle_request(request: raxx::Request, _state: i32) -> raxx::Response {
        match request.method {
            raxx::Method::GET =>
                raxx::Response{body: "GET".to_string()},
            raxx::Method::POST =>
                raxx::Response{body: "POST".to_string()}
        }
    }
}
impl raxx::Server<String> for MyServer {
    fn handle_request(request: raxx::Request, _state: String) -> raxx::Response {
        match request.method {
            raxx::Method::GET =>
                raxx::Response{body: "GET".to_string()},
            raxx::Method::POST =>
                raxx::Response{body: "POST".to_string()}
        }
    }
}
// Do rust traits always have a self

#[cfg(test)]
mod tests {
    use raxx;
    use raxx::Server;
    use MyServer;

    #[test]
    fn it_works() {
        let request = raxx::Request{method: raxx::Method::POST};
        println!("{:?}", request);

        assert_eq!(raxx::Response{body: "POST".to_string()}, MyServer::handle_request(request, "5".to_string()));
    }
}
```
