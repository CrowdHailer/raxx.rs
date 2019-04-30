extern crate hyper;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
const PHRASE: &str = "Hello, World!";

fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
}

fn main() {
    // Don't need to config port in docker
    // let port = env::var("PORT")
    //     .expect("environment must set a PORT")
    //     .parse()
    //     .expect("could not parse PORT");

    let addr = ([0, 0, 0, 0], 8080).into();

    let new_svc = || hyper::service::service_fn_ok(hello_world);

    let server = hyper::Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);
    hyper::rt::run(server);
}
