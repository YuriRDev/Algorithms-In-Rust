# ➿ First-Last PatternMatching
*String-searching* algorithm divided in two parts:
  * **Preprocessing**: _searches for the first and last character on the current position_
  * **Matching**: _after the preprocessing phase indentifies the window, this phase will investigates the windows to find all occurrences_

I discovered this algorithm in this [article](https://ieeexplore.ieee.org/document/8967097)

The method ```search()``` will return an Vector of the position that the pattern starts 

### 📃 Examples
```rust
use FLPM::Flpm;

fn main() {
    let flpm = Flpm::new("can you can a can as a canner can can a can?", "can");
    let result = flpm.search();
    
    println!("{:?}", result); // [0, 8, 14, 23, 30, 34, 40]
}
```