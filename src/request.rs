#[derive(Debug)]
pub enum Method {HEAD, GET, POST}

#[derive(Debug)]
pub struct Head {
    pub method: Method,
    pub path: String,
}

pub struct Message {
    pub head: Head,
    pub body: String
}
