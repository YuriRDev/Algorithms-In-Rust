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

    pub fn start(&self) {
        let mut same_hash_index: Vec<usize> = Vec::new();

        let first_hash = self.first_hash();
        if first_hash == self.pattern_hash {
            same_hash_index.push(0);
        }

        let mut previous_hash = first_hash;

        for cnt in 1..self.text.len() {
            let current_hash = self.roll_hash(cnt, previous_hash);
            if current_hash == self.pattern_hash {
                same_hash_index.push(cnt);
            }

            previous_hash = current_hash;
        }
    }
}

impl RabinKarp {
    fn roll_hash(&self, pos: usize, previous_hash: u32) -> u32 {
        let mut new_hash = previous_hash;
        println!("hashInicial: {}", new_hash);

        let to_remove_char = self.text.as_bytes()[pos - 1] as char;
        let to_remove = generate_character_hash(to_remove_char, self.pattern.len() - 1) % self.prime_number;
        print!("({} - {}) * 10 = ", new_hash, to_remove);
        new_hash -= to_remove;
        new_hash *= 10;
        print!("{} % 101 = ", new_hash);
        new_hash = new_hash % self.prime_number;
        print!("{} \n", new_hash);
        
        let new_char = self.text.as_bytes()[pos + self.pattern.len() - 1] as u32;
        println!("{} + {} = ", new_hash, new_char);
        new_hash = new_hash + new_char;
        print!("{} \n", new_hash);
        new_hash = new_hash%self.prime_number;
        print!(" % 101 = {} \n", new_hash);
        new_hash
    }

    fn first_hash(&self) -> u32 {
        let mut hash_total = 0;
        for cnt_pattern in 0..self.pattern.len() {
            hash_total +=
                generate_character_hash(self.text.as_bytes()[cnt_pattern] as char, self.pattern.len() - cnt_pattern - 1);
        }

        hash_total % self.prime_number
    }
}

pub fn generate_pattern_hash(pattern: &str, prime_number: u32) -> u32 {
    let mut cnt = pattern.len() as i32 - 1;
    let mut hash_value = 0;

    for character in pattern.chars() {
        println!("{}*10 ^{}", character as usize, cnt);
        hash_value += generate_character_hash(character, cnt as usize);

        cnt -= 1;
    }
    hash_value % prime_number
}

/// Generates character hash
/// Add doc later...
pub fn generate_character_hash(character: char, pos: usize) -> u32 {
    let power = 10u32.pow(pos as u32);
    let number_hash = character as u32 * power;

    number_hash
}
