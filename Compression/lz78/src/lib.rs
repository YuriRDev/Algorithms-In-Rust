use std::collections::HashMap;

pub struct Lz78 {
    text: String,
    dictionary: HashMap<String, usize>,
}

// Constructor call
impl Lz78 {
    /// ## Constructor for Lz78
    /// The text **should be** a to-compress or to-decompress.
    ///
    /// For compression there's no input validation.
    ///
    /// In the other hand, for the decompression method there's a input validation.
    /// **Text must be a number followed by a character**
    ///
    /// ---
    ///
    /// ### Compress Example
    /// ```
    /// let mut compression = Lz78::new("ABABCABCABCAABCAB");
    /// let compacted_text = compression.compact();
    ///
    /// assert_eq!(compacted_text, "0A0B1B0C3C5A6B");
    ///
    /// ```
    ///
    /// ---
    ///
    /// ### Decompress Example
    /// ```
    /// let mut decompress = Lz78::new("0A0B1B0C3C5A6B");
    /// let decompacted_text = compression.decompact();
    ///
    /// assert_eq!(decompacted_text, "ABABCABCABCAABCAB");
    /// ```
    pub fn new(text: &str) -> Lz78 {
        Lz78 {
            text: text.to_string(),
            dictionary: HashMap::new(),
        }
    }
}

// Public methods
impl Lz78 {
    /// ### Compact text from Lz78 text.
    /// Return the compacted format as string.
    /// ```
    /// use lz78::Lz78;
    ///
    /// let mut compression = Lz78::new("ABABCABCABCAABCAB");
    /// let compacted_text = compression.compact();
    ///
    /// println!("{}", compacted_text);
    /// ```
    pub fn compact(&mut self) -> String {
        let mut compacted_string = String::new();

        let mut prefix = String::new();
        let mut last_occurence = 0;

        for i in 0..self.text.len() {
            let current_char = self.text.as_bytes()[i] as char;
            prefix.push(current_char);

            match self.search_prefix(&prefix) {
                None => {
                    self.add_prefix(&prefix);
                    prefix = String::new();

                    compacted_string
                        .push_str(format!("{}{}", last_occurence, current_char).as_str());

                    last_occurence = 0;
                }
                Some(occurence) => {
                    last_occurence = occurence + 1;
                }
            }
        }

        compacted_string
    }

    pub fn decompact(&self) -> Result<String, &str> {
        if validate_decompact_string(&self.text) == false {
            return Err(
                "Invalid decompact string format \nMust be `number``char``number``char``...
            ",
            );
        }

        let mut decompact_str = String::new();

        Ok(decompact_str)
    }

    pub fn print_table(&self) {
        println!("{:?}", self.dictionary);
    }
}

// Private methods
impl Lz78 {
    /// Search prefix on the hash_table
    ///
    /// Returns An usize if found.
    /// Returns None if doesn't exists.
    fn search_prefix(&self, prefix: &str) -> Option<usize> {
        match self.dictionary.get(prefix) {
            Some(value) => Some(*value),
            None => None,
        }
    }

    /// Add a prefix on the hash_table
    ///
    /// Does not verify if it already exists
    fn add_prefix(&mut self, prefix: &str) {
        let dictionary_len = self.dictionary.len();
        self.dictionary.insert(prefix.to_string(), dictionary_len);
    }
}

/// Check if the text string is valid for decompress.
///
/// String must be ```numeric,char,numeric,char``` to be valid as "to_decompact" string
fn validate_decompact_string(value: &str) -> bool {
    for cnt in 0..value.len() {
        let character = value.as_bytes()[cnt] as char;

        if cnt % 2 == 0 {
            if !character.is_numeric() {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
/// # Test modules
mod tests {
    use super::*;

    #[test]
    /// Compact test is called "easy" because it's a small text with pattern
    fn compact_test_easy() {
        let mut compression = Lz78::new("ABABCABCABCAABCAB");
        let compacted_text = compression.compact();
        assert_eq!(compacted_text, "0A0B1B0C3C5A6B");
    }

    #[test]
    /// Compact test is called "medium" because it's a medium text with *some* pattern
    fn compact_test_medium() {
        let mut compression = Lz78::new("ABABABABABBACDABBABABACDCDCDCDACDABCDABCDACBDCA");
        let compacted_text = compression.compact();
        assert_eq!(
            compacted_text,
            "0A0B1B3A2A2B1C0D3B4B7D0C8C13D11A2C8A16D7B13A"
        );
    }

    #[test]
    /// Compact test is called "hard" becaus it's a relative big text with and *some* pattern
    fn compact_test_hard() {
        let mut compression =
            Lz78::new("ABCDEACBDE#@ABCDE#@ABCDEF@#ABCDEFG;ABC;ABCDEFG;ABCDEF#ABC@#ABCDEFG@#ABGD");
        let compacted_text = compression.compact();
        assert_eq!(
            compacted_text,
            "0A0B0C0D0E1C2D5#0@1B3D8@10C4E0F9#13D5F0G0;13;17E15G20A2C14F0#13@27A25D18G16A2G"
        );
    }
}
