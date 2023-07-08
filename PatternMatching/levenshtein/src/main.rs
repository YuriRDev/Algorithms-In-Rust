use levenshtein::Levensthain;

fn main() {
    use std::time::Instant;
    
    let mut distance = Levensthain::new("Nam quis nulla. Integer malesuada. In in enim a arcu imperdiet m", "Nam quis nulla. Nam quis n malesuada enim a arcu imperdiet m");
    let now = Instant::now();
    let value = distance.run();
    println!("Distance is: {}", value);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    // distance.print_grid();
}
