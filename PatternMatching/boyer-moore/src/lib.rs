use std::collections::HashMap;

pub struct BoyerMoore {
    pattern: String,
    text: String,
    bad_prefix_hash: HashMap<char, usize>,
}

// Constructor
impl BoyerMoore {
    /// ### "Constructor"
    /// You may initialize the BoyerMoore algorithm from here.
    /// ### Pattern length must be <= text length
    pub fn new(pattern: &str, text: &str) -> BoyerMoore {
        if pattern.len() > text.len() {
            panic!("The pattern length must be less or equal than text length ");
        }

        BoyerMoore {
            pattern: pattern.to_string(),
            text: text.to_string(),
            bad_prefix_hash: generate_bad_prefix_hash(pattern),
        }
    }
}

// Private methods
impl BoyerMoore {
    pub fn bad_prefix_search(&self, index: usize) -> usize {
        let first_char = self.text.as_bytes()[index] as char;
        let last = self.text.as_bytes()[index + self.pattern.len() - 1] as char;

        match self.bad_prefix_hash.get(&last) {
            Some(value) => value.to_owned() ,
            None => self.pattern.len(),
        }
    }
}

// Debug methods
impl BoyerMoore {
    /// Prints the **bad prefix** hash table
    pub fn print_bad_prefix_hash(&self) {
        println!("{:?}", self.bad_prefix_hash);
    }

    /// debug only
    pub fn get_index_char(&self, index: usize) {
        let first_char = self.text.as_bytes()[index] as char;

        println!("{}", first_char);
    }
}

/// Generates the bad_prefix hash table.
/// It's one of the features inside the BoyerMoore pattern matching.
///
/// Searches for the first occurence right-to-left *skipping the last char(AKA the first right char)*
/// and saves it to a hash.
fn generate_bad_prefix_hash(pattern: &str) -> HashMap<char, usize> {
    let mut hash: HashMap<char, usize> = HashMap::new();

    // Don't know how to reverse a iterator skipping the first position..
    // So I made it this way
    for cnt in (0..pattern.len() - 1).rev() {
        let current_char = pattern.as_bytes()[cnt] as char;

        match hash.get(&current_char) {
            None => {
                hash.insert(current_char, cnt);
            }
            _ => {}
        }
    }

    hash
}
