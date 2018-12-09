use std;
pub trait Simple<C> {
    fn handle_request(&self, super::request::Message, C) -> super::response::Message;
}

// Sized otherwise need to return Boxed self
// If returns None then server has stop. probably best to make a stop(reason) continue state enum.
// https://github.com/actix/actix/blob/master/src/actor.rs#L188-L190
// Stop should contain enum Normal, Shutdown??? Exit.
// Although dont want errors?
pub trait Server<C>: Sized {
    fn handle_head(self, super::request::Head, C) -> (Vec<super::response::Part>, Option<Self>);

    fn handle_data(self, String, C) -> (Vec<super::response::Part>, Option<Self>);

    fn handle_tail(self, (), C) -> (Vec<super::response::Part>, Option<Self>);
}

// Needs a buffer limit
pub struct SimpleWrapper<C, Inner: Simple<C>> {
    inner: Inner,
    head: Option<super::request::Head>,
    buffer: String,
    phantom: std::marker::PhantomData<C>
}

// Can enum through the states rather than have optional head
impl<C, I: Simple<C>> SimpleWrapper<C, I> {
    pub fn new(server: I) -> Self {
        Self{inner: server, head: None, buffer: "".to_string(), phantom: std::marker::PhantomData}
    }
}

impl<C, I: Simple<C>> Server<C> for SimpleWrapper<C, I> {
    fn handle_head(self, head: super::request::Head, _channel: C) -> (Vec<super::response::Part>, Option<Self>) {
        (vec![], Some(Self{head: Some(head), ..self}))
    }

    fn handle_data(self, data: String, _channel: C) -> (Vec<super::response::Part>, Option<Self>) {
        (vec![], Some(Self{buffer: self.buffer + data.as_str(), ..self}))
    }

    fn handle_tail(self, (): (), channel: C) -> (Vec<super::response::Part>, Option<Self>) {
        let Self{inner, head, buffer, ..} = self;
        let head = head.expect("The head should have already been sent");
        let message = super::request::Message{head, body: buffer};
        let response = inner.handle_request(message, channel);
        (response.to_parts(), None)
    }
}
