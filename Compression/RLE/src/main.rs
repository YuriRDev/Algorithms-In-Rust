use RLE::compress_rle;

fn main() {
    let text = "AAAAABBBBBBCCCCCCCCDD000000000LLLBCC";

    println!("{:?}", compress_rle(text));
}
