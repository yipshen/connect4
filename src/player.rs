use std::io;

pub trait Player {
    fn name(&self) -> &str;
    fn next_drop(&self) -> Result<usize, String>;
}

pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Player for HumanPlayer {
    fn name(&self) -> &str {
        &self.name
    }

    fn next_drop(&self) -> Result<usize, String> {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                buffer.pop();
                match buffer.parse::<usize>() {
                    Ok(col) => Ok(col),
                    Err(err) => Err(err.to_string()),
                }
            },
            Err(err) => Err(err.to_string()),
        }
    }
}
