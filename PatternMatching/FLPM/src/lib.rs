pub struct Flmp<'s> {
    text: &'s str,
    pattern: &'s str,
}

impl<'s> Flmp<'s> {
    pub fn new<'a>(text: &'a str, pattern: &'a str) -> Flmp<'a> {
        Flmp { text, pattern }
    }
}

impl<'s> Flmp<'s> {
    pub fn search(&self) -> Vec<usize> {
        let mut window: Vec<usize> = Vec::new();
        let mut answer: Vec<usize> = Vec::new();

        let text_bytes = self.text.as_bytes();
        let pattern_bytes = self.pattern.as_bytes();

        // get window
        for cnt in 0..(self.text.len() - self.pattern.len() + 1) {
            if text_bytes[cnt] == pattern_bytes[0]
                && text_bytes[cnt + self.pattern.len() - 1] == pattern_bytes[self.pattern.len() - 1]
            {
                window.push(cnt);
            }
        }

        // validate window
        for value in window {
            let mut founded = true;
            for cnt in 1..self.pattern.len() - 1 {
                if text_bytes[value + cnt] != pattern_bytes[cnt] {
                    founded = false;
                    break;
                }
            }
            if founded {
                answer.push(value);
            }
        };

        answer
    }
}
