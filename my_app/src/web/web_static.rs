// Name static is not ok for module.

pub fn handle<B>(request: &hyper::Request<B>) -> Option<hyper::Response<hyper::Body>> {
    match lookup(request.uri().path()) {
        Some((content_type, body)) => match request.method() {
            &hyper::Method::GET => {
                let response = hyper::Response::builder()
                    .status(hyper::StatusCode::OK)
                    .header("content-type", content_type)
                    .body(hyper::Body::from(body))
                    .unwrap();
                Some(response)
            }
            _ => {
                let response = hyper::Response::builder()
                    .status(hyper::StatusCode::NOT_IMPLEMENTED)
                    .body(hyper::Body::from(""))
                    .unwrap();
                Some(response)
            }
        },
        None => None,
    }
}

fn lookup(path: &str) -> Option<(&'static str, &'static [u8])> {
    match path {
        "/favicon.ico" => Some(("image/x-icon", include_bytes!("fixed_assets/favicon.ico"))),
        "/app.css" => Some(("text/css", include_bytes!("built_assets/app.css"))),
        _ => None,
    }
}
