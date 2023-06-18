pub struct RabinKarp {
    pattern: String,
    text: String,
    prime_number: u32,
}

impl RabinKarp {
    pub fn new(pattern: &str, text: &str) -> RabinKarp {
        RabinKarp {
            pattern: pattern.to_string(),
            text: text.to_string(),
            prime_number: 101
        }
    }
}


pub fn generate_hash(character: char,) {
    
}