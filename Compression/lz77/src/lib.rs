// DESCONTINUE FOR A FEW DAYS....
// DON'T KNOW THE COMPACT PROCESS FOR ABA-ABA IF IT'S 3,3,"" OR 3,2,A ....

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

            self.get_window(cnt);
        }
    }

    fn get_window(&self, current_pos: usize) {
        println!("---------------");
        println!("Current char window:");

        for i in (0..self.window) {
            let current = current_pos as i32 + i;

            if current < self.text.len() as i32 {
                print!("{}", self.text.as_bytes()[current as usize] as char);
            }
        }

        let mut pos_found = -1;
        let mut read_many = 0;
        let mut cnt = 0;

        for i in 0..self.window {
            let look_back_offset = current_pos as i32 + i - self.window;
            let current_char_offset = current_pos + cnt;

            if look_back_offset < 0 || current_char_offset >= self.text.len() {
                continue;
            }

            let look_back = self.text.as_bytes()[look_back_offset as usize] as char;
            let current_char = self.text.as_bytes()[current_char_offset] as char;

            println!("{} {}", look_back, current_char);

            if current_char == look_back {
                if pos_found == -1 {
                    pos_found = i;
                } else {
                    read_many += 1;
                }
                cnt += 1;
            }
        }

        println!("({pos_found}, {read_many})");

        println!("\n\n");
    }
}
