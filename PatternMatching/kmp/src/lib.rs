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
