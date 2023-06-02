use boyer_moore::BoyerMoore;

fn main() {
    println!("Hello, world!");
    let search = BoyerMoore::new("TEST", "THIS IS A TEST");
    let jump = search.bad_prefix_search(6);
    println!("Should jump: {}", jump);
    search.get_index_char(jump);
}
