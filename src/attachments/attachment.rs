#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Attachment {
    name: String,
}

impl Attachment {
    pub fn new(name: String) -> Self {
        Attachment {
            name
        }
    }
}
