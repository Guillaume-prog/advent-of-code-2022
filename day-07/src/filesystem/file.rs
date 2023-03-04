use std::fmt;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub size: usize
}

impl File {
    pub fn new(name: &str, size: usize) -> Self {
        Self { name: name.to_string(), size }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} bytes", self.name, self.size)
    }
}