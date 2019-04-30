# Rust Web

Experiments for productive Rust web development.

### What are the most important utilities.

## Routing
TODO
```rust
match r.segments() {
  End() => (),
  S("ping", End()) => (),
  S("users", S(user_id, End()))
}
match r.segments() {
  Zero() => homepage::handle(r),
  One("users") => list_users::handle,
  Two("users", user_id) => fetch_user::handle(r, user_id),
}
let mut segments = r.segments();
match segments.next() {
    None => homepage::handle(r),
    Some("users") => match segments.next() {
        None => list_users::handle(r),
        Some(user_id) => fetch_user::handle(r, c.matched(FetchUsers(user_id)))
        // can return a function that takes a request
    }
    Some("api") => api(segments, request, channel),
    Some("admin") => {
        let credentials = r.authentication()?;
        let () = authenticate_admin(credentials)?;
    }
    next => {
        let tracing_id = r.tracing_id();
        // mutable session
        let session = r.session();
        let channel = (tracing_id, session);
        match next {
          ("login") => state.handle(r, channel)
        }
    }
}

impl<R: Request> Action<Context<FetchUser>> for MyAppState {
  fn handle(self, request: R, c: Context<FetchUser>) {
    let FetchUser(user_id) = c.path;
    unimplemented!()
  }
}
// segments.next
// segments.collect
// segments.pop -> Some(part, s) if s.done()
```

## Views and Templates
TODO - Not part of API template

## Forms
TODO

## Built assets

Assets are built in a separate container for development.
The built assets are made available to the running app through a shared volume.

*Assets without a build step can simply be added to `my_app/web/fixed_assets`.*

Each new asset must be added to the lookup function in `my_app/web/web_static`.

- :-/ It would be neater to have a macro that looked through public dir at compile time.

## Reloading

Use [watchexec](https://github.com/watchexec/watchexec) to restart the server when any `src` files change.

- :-) Static files are compiled into release the same as in production.
- :-( Page needs to be reloaded to see changes.
- :-/ State is lost, not a problem for development of statless app.

https://github.com/SergioBenitez/Rocket/issues/163
https://github.com/SergioBenitez/Rocket/issues/292
https://github.com/emoon/dynamic_reload
https://crates.io/crates/cargo-watch
https://crates.io/crates/live-reload

use `watchexec` `cargo check` exists so that you can only stop when will compile

https://blog.fgribreau.com/2017/04/docker-compose-watch-k-docker-compose.html
http://cavaliercoder.com/blog/restarting-services-in-docker-compose.html


## Docker

- :-) Ensures set up of things like `watchexec` are portable.
- :-) Can use a completely isolated container for asset building
- ??? Would it be quicker to build npm tools into container?

## Authentication

Helper so that only the decode step is needed

```rust
let credentials: Basic = r.credentials()
```

## Logging/Monitoring
TODO
sign in page, put name in encrypted session
show on other pages

## Tracing
TODO
Part of a client module

## Sessions
TODO
Flash

## DB pool
TODO

### Config, lazy config
TODO
`config.global()` in the controllers
```rust
lazy_static! {
    something to get env
}

fn check() {
  // Fetches the first time, logs stuff if successful
}
fn config() {
  // uses expect as the only place in config
  unimplemented!()
}
```

## Error handling
TODO

## Middleware or NOT
TODO

## Streams for bodies is a bad idea
BUT a statemachine actor kinda thing is complicated to do without handling the other `info` messages.
router can return an object that gets iterated on to return responses. but a match function given a handle_request function does the right thing
