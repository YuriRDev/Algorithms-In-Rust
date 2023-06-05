use std::{collections::HashMap, f64::RADIX};

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

    /// ### Decompact text from Lz78 text.
    /// Return the decompacted format as an error or a String.
    ///
    /// ## Warning
    /// Input must be ```DecimalCharDecimalChar```
    //
    /// For now, dictionary must be length < 10, I was kinda dumb to do it that way
    ///
    /// ```
    /// use lz78::Lz78;
    ///
    /// let mut decompress = Lz78::new("ABABCABCABCAABCAB");
    /// let decompressed_text = compression.decompact();
    ///
    /// println!("{}", compacted_text);
    /// ```
    pub fn decompact(&mut self) -> Result<String, &str> {
        if validate_decompact_string(&self.text) == false {
            return Err(
                "Invalid decompact string format \nMust be `number``char``number``char``...
            ",
            );
        }

        let mut decompact_str = String::new();
        let mut cnt = 0;

        while cnt < self.text.len() {
            let pos = (self.text.as_bytes()[cnt] as char).to_digit(10).unwrap();
            cnt += 1;

            let value = self.text.as_bytes()[cnt] as char;
            cnt += 1;

            if pos == 0 {
                self.add_prefix(&format!("{}", value));
                decompact_str.push(value);
            } else {
                let mut tmp = String::new();
                tmp.push_str(self.search_index(pos as usize).as_str());
                tmp.push(value);

                self.add_prefix(&tmp);
                decompact_str.push_str(&tmp);
            }
        }

        Ok(decompact_str)
    }

    /// ### Print's dictionary table
    /// You can use this method after the ```.compact()``` or ```.decompact()``` method.
    /// 
    /// It'll will print the table in {:?} format.
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

    /// Searches an map by it's value instead of the key
    /// I'm pretty sure that exists more methods to do that.
    /// But I couldn't find it at the time.
    ///
    /// ### Dev Note
    /// I know I should have use an Option<String>, but to be honest, I'm lazy rn
    fn search_index(&self, index: usize) -> String {
        for (key, value) in &self.dictionary {
            if *value + 1 == index {
                return key.to_string();
            }
        }

        return String::new();
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
