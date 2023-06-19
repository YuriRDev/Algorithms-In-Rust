#[derive(Debug)]
pub struct RabinKarp {
    pattern: String,
    text: String,
    prime_number: u32,
    pattern_hash: u32,
}

impl RabinKarp {
    pub fn new(pattern: &str, text: &str) -> RabinKarp {
        RabinKarp {
            pattern: pattern.to_string(),
            text: text.to_string(),
            prime_number: 101,
            pattern_hash: generate_pattern_hash(pattern, 101),
        }
    }

    pub fn print(&self) {
        println!("{:?}", &self);
    }
}

impl RabinKarp {
    fn roll_hash() {}
}

pub fn generate_pattern_hash(pattern: &str, prime_number: u32) -> u32 {
    let mut cnt = pattern.len() as i32 - 1;
    let mut hash_value = 0;

    for character in pattern.chars() {
        println!("{}*10 ^{}", character as usize, cnt);
        let power = 10u32.pow(cnt as u32);
        let number_hash = character as u32 * power;
        hash_value += number_hash;

        cnt -= 1;
    }
    hash_value % prime_number
}

pub fn generate_hash(character: char) {}
