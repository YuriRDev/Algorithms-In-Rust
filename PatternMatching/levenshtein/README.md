# 📏 Levenshtein Distance
*String-metric* algorithm that measures the difference between two strings.

### 📃 Example
```rust
use levenshtein::Levensthain;

fn main() {
    let mut distance = Levensthain::new("I'm super cool", "I'm just cool");
    let value = distance.run();
    println!("Distance is: {value}");

    // If you wan't to see the grid.
    // For now you must call this method after the .run()
    distance.print_grid();
    
}


```

### 👻 To-do
- [ ]  Use comfy_table to print the grid more User friendly