#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub symbol: char,
}

impl Player {
    pub fn new(name: &String, symbol: char) -> Self {
        Self {
            name:name.clone(),
            symbol
        }
    }
}
