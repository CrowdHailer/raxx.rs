// pub enum Head<Channel, Inner: super::super::server::Server<Channel>> {
//     On(Inner, std::marker::PhantomData<Channel>),
//     Off(Inner, std::marker::PhantomData<Channel>)
// }

// impl<Channel, Inner: super::super::server::Server<Channel>> Head<Channel, Inner> {
//     pub fn new(inner: Inner) -> Self {
//         Head::Off(inner, std::marker::PhantomData)
//     }
// }
// To test this we need to add content length function
pub struct Head<Inner: super::super::server::Server> {
    inner: Inner,
    active: bool,
}

impl<Inner: super::super::server::Server> Head<Inner> {
    pub fn new(inner: Inner) -> Self {
        Self{inner: inner, active: false}
    }
}
impl<Inner: super::super::server::Server> Head<Inner> {
    fn handle_response(x: (Vec<super::super::response::Part>, Option<Inner>), active: bool) -> (Vec<super::super::response::Part>, Option<Self>) {
        let (parts, maybe_inner) = x;
        println!("{:?}", active);
        let maybe_self = maybe_inner.map(|inner| Self{inner: inner, active: active});
        // let parts = parts.iter().filter(|p| match p {
        //     super::super::response::Part::Head(_) =>
        //         true,
        //     _ =>
        //         false
        // }).collect();
        (parts, maybe_self)
    }
}
impl<Inner: super::super::server::Server> super::super::server::Server for Head<Inner> {
    type Channel = Inner::Channel;
    fn handle_head(self, head: super::super::request::Head, channel: Self::Channel) -> (Vec<super::super::response::Part>, Option<Self>) {
        let (head, me) = match head.method {
            super::super::request::Method::HEAD =>
                (super::super::request::Head{method: super::super::request::Method::GET, ..head}, Self{active: true, ..self}),
            _ =>
                (head, self)
        };
        Head::handle_response(me.inner.handle_head(head, channel), me.active)
    }

    fn handle_data(self, data: String, channel: Self::Channel) -> (Vec<super::super::response::Part>, Option<Self>) {
        Head::handle_response(self.inner.handle_data(data, channel), self.active)
    }

    fn handle_tail(self, (): (), channel: Self::Channel) -> (Vec<super::super::response::Part>, Option<Self>) {
        Head::handle_response(self.inner.handle_tail((), channel), self.active)
    }
}
