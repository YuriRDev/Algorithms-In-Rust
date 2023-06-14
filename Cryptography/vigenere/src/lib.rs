pub struct Vigenere {
    text: String,
    key: String,
}

// Constructor
impl Vigenere {
    pub fn new(text: &str, key: &str) -> Vigenere {
        validate_input(text);
        validate_input(key);

        Vigenere {
            text: text.to_uppercase().to_string(),
            key: key.to_uppercase().to_string(),
        }
    }
}

// Private methods
impl Vigenere {
    pub fn crypt(&self) -> String {
        let mut return_string = String::new();

        for cnt in 0..self.text.len() {
            let pos1 = self.text.as_bytes()[cnt];
            let pos2 = self.key.as_bytes()[cnt % self.key.len()];
            return_string.push(get_pos_value(pos1, pos2));
        }

        return_string
    }
}

fn get_pos_value(pos1: u8, pos2: u8) -> char {
    // Some crazy math, but trust me, it makes sense
    let pos_as_bytes = (pos1 + pos2) % 26 + 65;

    pos_as_bytes as char
}

fn validate_input(text: &str) {
    for &byte_value in text.as_bytes() {
        if byte_value < 65 || byte_value > 90 {
            panic!(
                "[Panic] Invalid input type, expected byte in rage 65-90(A-Z), received: {}({})",
                byte_value, byte_value as char
            );
        }
    }
}
