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
    pub fn new(pattern: &str, text: &str) -> BoyerMoore {
        BoyerMoore {
            pattern: pattern.to_string(),
            text: text.to_string(),
            bad_prefix_hash: generate_bad_prefix_hash(pattern),
        }
    }
}

// Debug methods
impl BoyerMoore {
    /// Prints the **bad prefix** hash table
    pub fn print_bad_prefix_hash(&self) {
        println!("{:?}", self.bad_prefix_hash);
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
