#[derive(Debug)]
pub enum Status {
    OK,
    BadRequest,
    NotFound
    // Do other which has number and reason phrase
}

#[derive(Debug)]
pub struct Head {
    pub status: Status,
    pub body: bool
}

// Maybe body true/false shouldn't be on head but be part of part
#[derive(Debug)]
pub enum Part {
    Head(Head),
    Data(String),
    Tail
}

// There might not be any real reason to parameterise message,
// can't think of an occasion where you would be unsure,
// if want same builders the implemeent a Raxx.Message trait on both
#[derive(Debug)]
pub struct Message { pub head: Head, pub body: String }

impl Head {
    pub fn set_body(self, body: String) -> Message {
        let head = Self{body: true, ..self};
        Message{head: head, body: body}
    }
}

impl Message {
    pub fn to_parts(self) -> Vec<Part> {
        let Self{head, body} = self;
        vec![Part::Head(head), Part::Data(body), Part::Tail]
    }
}
