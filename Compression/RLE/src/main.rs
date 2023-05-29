use RLE::{compress_rle, decompres_rle};

fn main() {
    let text = "AAAAABBBBBBCCCCCCCCDD000000000LLLBCC";

    let compressed = compress_rle(text);
    println!("{}", decompres_rle(compressed));
}
