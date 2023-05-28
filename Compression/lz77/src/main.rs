use std::collections::HashMap;

const WINDOW_SIZE: usize = 3;
const TEXT: &str = "hello, my cute world! hello, my cute world!";
fn main() {
    let mut behind: HashMap<char, usize> = HashMap::new();
    let mut count: usize = 0;

    while count != TEXT.len() {
        print!("Current char: ");
        println!("{}", TEXT.bytes().nth(count).unwrap() as char);
        print!("Current hash: ");
        println!("{:?}", behind);
        count += 1;
    }
}

fn search_for_text(text: &str, map: HashMap<char, usize>) -> (usize, usize) {


    (0, 0)
}

fn search_for_char() {}
