const PHRASE: &str = "Hello, World!";

struct Segments<'a>(std::str::Split<'a, &'a str>);
impl<'a> Segments<'a> {
    fn new<B>(request: &'a hyper::Request<B>) -> std::iter::Peekable<Self> {
        match request.uri().path().split_at(1) {
            ("/", rest) => Self(rest.split("/")).peekable(),
            _ => unimplemented!(),
        }
    }
}

impl<'a> Iterator for Segments<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        match self.0.next() {
            None => None,
            Some("") => self.next(),
            // TODO handle `..`
            Some(value) => Some(value),
        }
    }
}

// Call Server
// Server::setup
struct Config {

}

trait Action<C> {
    fn handle<B>(&self, r: &hyper::Request<B>, c: C)-> hyper::Response<hyper::Body>;
}
trait TracingId {

}
// Could also turn inside out and insist a match has a tracing id for example.
// Or match.handle(request, config) match is data structure with path info inside.
// FetchUser::handle(request, user_id) <- user_id is just a string there.
// THIS C is channel
impl<C> Action<C> for Config where C: Match<HomePage> + TracingId {
    fn handle<B>(&self, r: &hyper::Request<B>, channel: C) -> hyper::Response<hyper::Body>{
        hyper::Response::new(hyper::Body::from(PHRASE))
    }
}

struct FetchEntity<C> {
    entity_id: String,
    channel: C
}
// There is alot of bundling and debundling. which suggests that a routing tree is good.
// There are just less things that need a name
// Don't want to write lots of delegating functions into the Channel
impl<C: std::fmt::Debug> FetchEntity<C> {

}

impl TracingId for HomePage {

}

// Not an associated trait implement it lots of times
trait Match<X> {
    fn to_match(&self) -> &X;
}
impl<T> Match<T> for T {
    fn to_match(&self) -> &T {
        self
    }

}
impl Config {
    // fn handle<B, M>(self, r: &hyper::Request<B>, m: M, (): ()) -> hyper::Response<hyper::Body> {
    //     unimplemented!()
    // }
}
struct HomePage {

}
// hyper::Response::new(hyper::Body::from(PHRASE))
// End | Seg("foo", )
// Option A, Actions want to work with as near HTTP as possible
// Option B, Great big routing tree
pub fn handle<B>(request: &hyper::Request<B>) -> hyper::Response<hyper::Body> {
    let server = Config{};
    let mut segments = Segments::new(request);
    match segments.next() {
        None => server.handle(request, HomePage{}),
        Some("api") => match segments.next() {
            // Would have a data.json line
            None => unimplemented!(),
            Some("users") => match segments.next() {
                None => unimplemented!(),
                Some(user_id) => unimplemented!("{}", user_id),
            },
            _ => unimplemented!()
        },
        Some("login") => hyper::Response::new(hyper::Body::from("foo")),
        _segment => {
            let _session = ();
            unimplemented!();
        }
    }
    // Can check that all the segments have been used before calling function.
}
//
// pub fn other<B>(request: &hyper::Request<B>) {
//     let mut segments = Segments::new(request);
//     match segments.next() {
//         Some(_) if segments.peek().is_none() => (),
//         None  => ()
//         // cannot mutate in guard, use peekable
//         // s if s.next().is_none() => 5
//         // s if let x = s.to_string => x
//
//     };
//     unimplemented!()
// }
