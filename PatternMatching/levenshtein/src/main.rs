use levenshtein::Levensthain;

fn main() {
    let mut distance = Levensthain::new("denyam", "ephref");
    let value = distance.run();
    
    distance.print_grid();
    println!("{value}");
}
