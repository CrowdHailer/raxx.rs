pub mod request;
pub mod response;
pub mod server;
pub mod middleware;

pub fn request(method: request::Method, path: String) -> request::Head {
    request::Head{method, path}
}

pub fn response(status: response::Status) -> response::Head {
    response::Head{status, body: false}
}


// channel in certain states. If we have a push! function that blows up at the right point. Can even count bytes.
// call push function send. To be efficient it should have a to_iolist trait.

// have a push function but I don't really know how to rescue it.
// Rather than returning an out of order list. need a Raxx.Channel trait. that does the ordering
// If parameterised by channel we can have one that does some checks is just not gerally orderable.
// Eg We should implement some kind of ip trait. We can also wrap the request once if we know what we are doing
// but then we don't get a concrete request type in the server.
// Still want to assert that all request -> string -> server gets the same request. Can create a checker server that take request as state.
// Can pass the match in the single updated channel.
// enum of all the routes that channel might have then controller receives of particular kind. Let's not worry about method at this stage.
// have a public channel state type with a send/push/apply fn that returns new state that can be stored in a server specific struct
// channel.send.state -> return struct

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(1, 2);
    }
}

// If every middleware that would extend the channel/request is NOT a middleware Then this wrapper might be enough
// Can use the plan of pushing it into the state to make it work.
// State is not really accessible here
