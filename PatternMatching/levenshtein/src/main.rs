use levenshtein::Levensthain;

fn main() {
    let mut distance = Levensthain::new("ak39818@91028172", "ak2910@");
    let value = distance.run();
    println!("Distance is: {}", value);

    distance.print_grid();
}
