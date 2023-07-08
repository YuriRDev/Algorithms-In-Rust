pub struct Flpm<'s> {
    text: &'s str,
    pattern: &'s str,
}

impl<'s> Flpm<'s> {
    pub fn new<'a>(text: &'a str, pattern: &'a str) -> Flpm<'a> {
        Flpm { text, pattern }
    }
}

impl<'s> Flpm<'s> {
    pub fn search(&self) -> Vec<usize> {
        let window = self.get_window();
        let mut answer: Vec<usize> = Vec::new();

        // validate window
        'outer: for value in window {
            for cnt in 1..self.pattern.len() - 1 {
                if self.text.as_bytes()[value + cnt] != self.pattern.as_bytes()[cnt] {
                    continue 'outer;
                }
            }
            answer.push(value);
        }

        answer
    }

    /// ## Pre-Process fase
    /// Get the position where the first-last chars are equal to the pattern
    pub fn get_window(&self) -> Vec<usize> {
        let mut window: Vec<usize> = Vec::new();

        let text_bytes = self.text.as_bytes();
        let pattern_bytes = self.pattern.as_bytes();

        for cnt in 0..(self.text.len() - self.pattern.len() + 1) {
            let first_equal = text_bytes[cnt] == pattern_bytes[0];
            let last_equal =
                text_bytes[cnt + self.pattern.len() - 1] == pattern_bytes[self.pattern.len() - 1];

            if first_equal && last_equal {
                window.push(cnt);
            }
        }

        window
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_test() {
        let a = Flpm::new("the prince of pride", "pri").search();
        assert_eq!(a, [4, 14]);
    }

    #[test]
    fn medium_test() {
        let a = Flpm::new("can you can a can as a canner can can a can?", "can").search();
        assert_eq!(a, [0, 8, 14, 23, 30, 34, 40]);
    }

    #[test]
    fn big_test() {
        let a = Flpm::new("How much wood would a woodchuck chuck if a woodchuck could chuck wood?", "wood").search();
        assert_eq!(a, [9, 22, 43, 65]);
    }
}
