use std::collections::HashMap;

// There's not a vector os bits, we can stick with vector of booleans, the theory is that is a bit anyways..
pub struct ShiftAnd {
    pattern: String,
    mask: HashMap<char, Vec<bool>>,
}

/// Constructor
impl ShiftAnd {
    pub fn new(pattern: &str) -> ShiftAnd {
        ShiftAnd {
            pattern: pattern.to_string(),
            mask: generate_mask(&pattern),
        }
    }
}

/// Generates the mask for the pre-process.
/// It's the Hash Table to get the vector of bits of a key
fn generate_mask(text: &str) -> HashMap<char, Vec<bool>> {
    let mut mask_hash: HashMap<char, Vec<bool>> = HashMap::new();

    // Generate empty hashmap
    for character in text.chars() {
        match mask_hash.get(&character) {
            Some(_) => continue,
            None => {
                mask_hash.insert(character, get_ocurrence(&text, character));
            }
        }
    }

    mask_hash
}

/// Get ocurrences of a char in a text
/// Returns a Vector of bits
///
/// *Lmao, in this case vector of boolean, cause Rust don't have Vec<bit>, It's the same i guess*
fn get_ocurrence(text: &str, character: char) -> Vec<bool> {
    let mut vec: Vec<bool> = Vec::new();

    for value in text.chars() {
        if value == character {
            vec.push(true);
        } else {
            vec.push(false);
        }
    }

    vec
}
