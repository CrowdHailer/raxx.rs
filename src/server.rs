use super::request;
use super::response;

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
