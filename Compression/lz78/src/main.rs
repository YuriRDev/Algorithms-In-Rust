use lz78::Lz78;

fn main() {
    let mut compression = Lz78::new("1a0b0c0d0e0f0f0g03020103");
    // let compacted_text = compression.compact();
    match compression.decompact() {
        Err(err) => println!("{err}"),
        Ok(v) => println!("{v}"),
    }

    // Debug, mut run after then compact or decompacted method
    // compression.print_table();
}
