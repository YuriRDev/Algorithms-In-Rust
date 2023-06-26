use std::{io::ErrorKind, collections::HashMap};

pub struct Kmp {
    text: String,
    target: String,
    transition: Vec<usize>,
}

// Public implementation
impl Kmp {
    pub fn new(target: &str, text: &str) -> Kmp {
        Kmp {
            text: String::from(text),
            target: String::from(target),
            transition: get_fail_transition(target),
        }
    }

    /// ### Search the first occurency of the pattern on the text
    /// When found, it'll return an tupple of usizes (usize,usize)
    /// 
    /// That means where it found it (first_index, last_index)
    /// 
    /// **It's an inclusive and continuos**
    /// 
    pub fn search_first(&self) -> Result<(usize, usize), &str> {
        let mut index_pattern = 0;

        'outer: for i in 0..self.text.len() {
            let text_char = self.text.as_bytes()[i];
            let mut pattern_char = self.target.as_bytes()[index_pattern];

            while text_char != pattern_char {
                if index_pattern == 0 {
                    continue 'outer;
                }
                index_pattern = self.transition[index_pattern - 1];
                pattern_char = self.target.as_bytes()[index_pattern];
            }

            if index_pattern == self.target.len() - 1 {
                return Ok((i - index_pattern, i));
            }

            index_pattern += 1;
        }

        Err("Nao foi possivel encontrar o resultado")
    }
}

// Debug implementation (also public)
impl Kmp {
    pub fn print_fail_transition(&self) {
        println!("{:?}", self.transition)
    }
}

/// When creating a Kmp pattern matching, it's
/// necessary to create a fail transition. *that's the "beauty" behind KMP*
///
/// <u>The full explanation is on the README</u>
/// ### Examples
/// ```
/// use kmp::get_fail_transition;
///
/// let search_word = "araraquara";
/// let fail_transition = get_fail_transition(&search_word);
///
/// assert_eq!(fail_transition, Vec::from([0, 0, 1, 2, 3, 0, 0, 1, 2, 3]));
///
/// ```
fn get_fail_transition(target: &str) -> Vec<usize> {
    let mut hash: Vec<usize> = Vec::from([0]);

    let mut prefix = 0;

    for current in 1..target.len() {
        let current_char = target.as_bytes()[current];
        let prefix_char = target.as_bytes()[prefix];

        if current_char != prefix_char {
            prefix = 0;
        } else {
            prefix += 1;
        }

        hash.push(prefix);
    }

    hash
}
