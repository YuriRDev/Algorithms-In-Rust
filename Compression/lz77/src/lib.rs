pub struct Lz77 {
    text: String,
    window: i32,
}

impl Lz77 {
    /// "Constructor"
    pub fn new(text: &str, window: i32) -> Lz77 {
        Lz77 {
            text: text.to_string(),
            window,
        }
    }
}

impl Lz77 {
    pub fn start(&self) {
        for cnt in 0..self.text.len() {
            let character = self.text.as_bytes()[cnt] as char;

            println!("{}", character);
            println!("Current_Dictionary");
            self.get_window(cnt);
        }
    }

    fn get_window(&self, current_pos: usize) {
        println!("crnt: {}", current_pos);
    }
}
