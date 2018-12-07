pub struct Board {
    name: String,
}

impl Board {
    pub fn new(name: &str) -> Board {
        Board {
            name: String::from(name),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
