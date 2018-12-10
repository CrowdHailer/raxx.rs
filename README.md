# Raxx.rs

**Interface for building HTTP client and server applications.**

*Ported from the original Elixir [Raxx](github.com/CrowdHailer/raxx) project.*

## SimpleServer

Main web applications can be implemented as a simple function translating a clients request to the servers response.
*If this is not your application jump to the [Server](#server) section.*

```rs
extern crate raxx;
use raxx::SimpleServer;

use raxx::request;
use raxx::response;
use raxx::response::Status::*;

pub struct MyApp;
impl SimpleServer for MyApp {
    type Channel = ();
    fn handle_request(&self, _request: request::Message, _channel: ())
        -> response::Message {
            response(OK)
                .set_header("content-lenth".to_string(), "text/plain".to_string())
                .set_body("Hello, World!".to_string())
    }
}
```

Channel is a field provided by the server (or middleware) that provides information not contained in the request.
An app may depend on a channel that implements, for example, a ClientIP trait to access the ip address of the client.
Or an app might depend on an authenticated channel, which can be provided by a wrapper (middleware) that extends the channel to have the required properties.

```rs
struct HelloPeer<Channel> { _channel: std::marker::PhantomData<Channel> }

impl<Channel> HelloPeer<Channel> {
    pub fn new() -> Self {
        Self{_channel: std::marker::PhantomData}
    }
}
impl<Channel: ClientIP> SimpleServer for HelloPeer<Channel> {
    type Channel = ();
    fn handle_request(&self, _request: request::Message, channel: ()) -> response::Message {
        response(OK)
            .set_header("content-lenth".to_string(), "text/plain".to_string())
            .set_body(format!("Hello, {}!", channel.client_ip()))
    }
}

trait ClientIP {
    fn client_ip(&self) -> String;
}

// This ClientIP implementation is a dummy implementation, simplistic but useful for testing.
impl ClientIP for () {
    fn client_ip(&self) -> String {
        "127.0.0.1".to_string()
    }
}
```

## Server

This interface should be used when either the request or response should be dealt with in a partial matter.
File uploads or long polling are example cases where more fine grained control is required.  
