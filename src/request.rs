extern crate url;
// use self::url::Url;
use self::url::{Host, Origin, Url};

#[derive(Debug)]
pub enum Method {HEAD, GET, POST}

#[derive(Debug)]
pub struct Head {
    pub method: Method,
    pub path: String,
    pub body: bool
}

pub struct Message {
    pub head: Head,
    pub body: String
}

// require full path.
pub fn new(method: Method, path: String) -> Head {
    let foo = Url::parse(&path).unwrap();
    println!("{:?}", foo.origin());
    println!("{:?}", foo.path_segments());
    // I wonder if there is any reason to not just put url on request struct
    Head{method: method, path: path, body: false}
}

impl Head {
    pub fn set_body(self, body: String) -> Message {
        let head = Self{body: true, ..self};
        Message{head: head, body: body}
    }
}
