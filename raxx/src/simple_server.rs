use super::request;
use super::response;

pub trait SimpleServer: Sized {
    type Channel;
    // Can have just a reference to self, never any need to update it.
    fn handle_request(&self, request::Message, Self::Channel) -> response::Message;

    fn to_server(self) -> Wrapper<Self> {
        Wrapper::new(self)
    }
}

// Needs a buffer limit
pub struct Wrapper<Inner: SimpleServer> {
    inner: Inner,
    head: Option<request::Head>,
    buffer: String,
}

// Can enum through the states rather than have optional head
impl<Inner: SimpleServer> Wrapper<Inner> {
    pub fn new(server: Inner) -> Self {
        Self{inner: server, head: None, buffer: "".to_string()}
    }
}

impl<Inner: SimpleServer> super::server::Server for Wrapper<Inner> {
    type Channel = Inner::Channel;
    fn handle_head(self, head: request::Head, _channel: Inner::Channel) -> (Vec<response::Part>, Option<Self>) {
        (vec![], Some(Self{head: Some(head), ..self}))
    }

    fn handle_data(self, data: String, _channel: Inner::Channel) -> (Vec<response::Part>, Option<Self>) {
        (vec![], Some(Self{buffer: self.buffer + data.as_str(), ..self}))
    }

    fn handle_tail(self, (): (), channel: Inner::Channel) -> (Vec<response::Part>, Option<Self>) {
        let Self{inner, head, buffer, ..} = self;
        let head = head.expect("The head should have already been sent");
        let message = request::Message{head, body: buffer};
        let response = inner.handle_request(message, channel);
        (response.to_parts(), None)
    }
}
