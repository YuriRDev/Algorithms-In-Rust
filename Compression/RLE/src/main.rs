use RLE::compress_rle;

fn main() {
    let to_compress = "AAAAABBBBBBCCCCCCCCDD000000000LLLBCC";

    // Compress
    println!("{}", compress_rle(to_compress));


    // decompress
    // I forgot to change that haha, wait a few days :D
    // println!("{}", decompres_rle("5A6B8C2D903L1B2C"));
}
