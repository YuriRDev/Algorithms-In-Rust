use lz78::Lz78;

fn main() {
    let mut compression = Lz78::new("0A0B1B0C3C5A6B");
    // let compacted_text = compression.compact();
    match compression.decompact() {
        Err(err) => println!("{err}"),
        Ok(v) => println!("{v}"),
    }

    // Debug, mut run after then compact or decompacted method
    // compression.print_table();
}
