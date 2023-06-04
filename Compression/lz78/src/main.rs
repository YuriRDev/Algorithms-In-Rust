use lz78::Lz78;

fn main() {
    let mut compression = Lz78::new("ABABCABCABCAABCAB");
    let compacted_text = compression.compact();
    
    // Debug, mut run after then compact or decompacted method
    compression.print_table();
}
