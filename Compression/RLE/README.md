# 🏃‍♂️ Run-Length Encoding
Super basic *lossless* data compression. 



### 📃 Example
```rust
use RLE::compress_rle;

fn main() {
    let to_compress = "AAAAABBBBBBCCCCCCCCDD000000000LLLBCC";

    // Compress
    println!("{}", compress_rle(to_compress));


    // decompress
    // I forgot to change that haha, wait a few days :D
    // println!("{}", decompres_rle("5A6B8C2D903L1B2C"));
}

```

### 🤓 Notes
First of all, I don't know why I made the decompress_rle method that way, really, no clue. I'll change that later. haha


In my implementation, if a character only appears only once, it'll have the number 1 behind it on the compression.

``` 
AAAAABBBBBBCCCCCCCCDD000000000LLLBCC
4A3B2A5B8C1D1A1BC3A4B
```

I'm aware that the "correct" implementation wouldn't have the number before the char *(I could argue the pros and cons)*, but I'll add a settings later for preferences.
``` 
AAAABBBAABBBBBCCCCCCCCDABCAAABBBB
4A3BAA5B8CDABC3A4B
```


### 👻 To-do
- [ ] Compress/Decompress text from file format 
- [ ] Compress/Decompress text from CLI input

