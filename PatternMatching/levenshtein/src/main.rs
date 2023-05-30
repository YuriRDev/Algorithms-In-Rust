use comfy_table::Table;
use levenshtein::Levensthain;

fn main() {
    let source = "I'm12 super cool";
    let target = "I'm super coo34l";
    let mut distance = Levensthain::new(target, source);
    let value = distance.run();
    println!("Distance is: {value}");
}
