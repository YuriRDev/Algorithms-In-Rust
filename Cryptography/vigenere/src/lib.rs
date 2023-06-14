pub struct Vigenere {
    text: String,
    key: String,
    key_stream: String,
}

// Constructor
impl Vigenere {
    pub fn new(text: &str, key: &str) -> Vigenere {
        validate_input(text);
        validate_input(key);

        Vigenere {
            text: text.to_uppercase().to_string(),
            key: key.to_uppercase().to_string(),
            key_stream: generate_key_stream(&text.to_uppercase(), &key.to_uppercase()),
        }
    }
}

// Private methods
impl Vigenere {
    pub fn crypt(&self) -> String {
        let mut return_string = String::new();

        for cnt in 0..self.text.len() {
            let pos1 = self.text.as_bytes()[cnt] as char;
            let pos2 = self.key_stream.as_bytes()[cnt] as char;

            return_string.push(get_pos_value(pos1, pos2));
        }

        return_string
    }
}

fn get_pos_value(pos1: char, pos2: char) -> char {
    // Some crazy math, but trust me, it makes sense
    let pos_as_bytes = (pos1.to_string().as_bytes()[0] + pos2.to_string().as_bytes()[0]) % 26 + 65;

    pos_as_bytes as char
}

/// Will remove it later
fn generate_key_stream(text: &str, key: &str) -> String {
    let mut return_string = String::new();
    for i in 0..text.len() {
        let key_char = key.as_bytes()[i % key.len()] as char;
        return_string.push(key_char);
    }

    return_string
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
