use crate::printable::Printable;

pub struct File {
    name: String,
}

impl Printable for File {
    fn print(&self) {
        println!("File: {}", self.name);
    }
}

impl File {
    pub fn new(name: &str) -> Self {
        File {
            name: name.to_string(),
        }
    }
}
