use std::{collections::HashMap, vec};

// There's not a vector os bits, we can stick with vector of booleans, the theory is that is a bit anyways..
pub struct ShiftAnd {
    pattern: String,
    text: String,
    mask: HashMap<char, Vec<bool>>,
}

/// Constructor
impl ShiftAnd {
    pub fn new(pattern: &str, text: &str) -> ShiftAnd {
        ShiftAnd {
            pattern: pattern.to_string(),
            text: text.to_string(),
            mask: generate_mask(&pattern),
        }
    }
}

impl ShiftAnd {
    pub fn search(&self) {
        let mut current = self.generate_empty_vec();
        for character in self.text.chars() {
            // println!("Current: {:?}", current);

            let character_mask = match self.mask.get(&character) {
                Some(mask) => mask.to_owned(),
                None => vec![false; self.pattern.len()],
            };

            // println!("Mask de {}: {:?}", character, character_mask);

            // Current and mask
            let and = vec_and_vec(&current, &character_mask);
            if and.get(self.pattern.len() - 1).unwrap() == &true {
                println!("Encontrou padrao no character");
            }
            // println!("And: {:?}", and);

            current = shift_vec(&and);
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

fn shift_vec(vec: &Vec<bool>) -> Vec<bool> {
    let mut vec_tmp: Vec<bool> = Vec::from([true]);

    for cnt in 0..vec.len() - 1 {
        vec_tmp.push(vec[cnt]);
    }

    vec_tmp
}

impl ShiftAnd {
    fn generate_empty_vec(&self) -> Vec<bool> {
        let mut tmp_vec = vec![false; self.pattern.len()];
        tmp_vec[0] = true;

        tmp_vec
    }
}

fn vec_and_vec(vec1: &Vec<bool>, vec2: &Vec<bool>) -> Vec<bool> {
    let mut vec_tmp: Vec<bool> = Vec::new();

    for cnt in 0..vec1.len() {
        vec_tmp.push(vec1[cnt] == true && vec2[cnt] == true);
    }

    vec_tmp
}
