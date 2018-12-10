use std;
use super::request;
use super::response;

pub trait Simple {
    type Channel;
    fn handle_request(&self, request::Message, Self::Channel) -> response::Message;
}

// Sized otherwise need to return Boxed self
// If returns None then server has stop. probably best to make a stop(reason) continue state enum.
// https://github.com/actix/actix/blob/master/src/actor.rs#L188-L190
// Stop should contain enum Normal, Shutdown??? Exit.
// Although dont want errors?
pub trait Server: Sized {
    type Channel;

    fn handle_head(self, request::Head, Self::Channel) -> (Vec<response::Part>, Option<Self>);

    fn handle_data(self, String, Self::Channel) -> (Vec<response::Part>, Option<Self>);

    fn handle_tail(self, (), Self::Channel) -> (Vec<response::Part>, Option<Self>);
}

// Needs a buffer limit
pub struct SimpleWrapper<Inner: Simple> {
    inner: Inner,
    head: Option<request::Head>,
    buffer: String,
}

// Can enum through the states rather than have optional head
impl<Inner: Simple> SimpleWrapper<Inner> {
    pub fn new(server: Inner) -> Self {
        Self{inner: server, head: None, buffer: "".to_string()}
    }
}

impl<Inner: Simple> Server for SimpleWrapper<Inner> {
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
