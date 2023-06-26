# 📏 Exact Shift-And
The exact version of shift-and algorithm. 

### 📃 Example
```rust
use shift_and::ShiftAnd;

fn main() {
    let mut shiftand = ShiftAnd::new("teste", "os testeste");
    let patterns_window = shiftand.search();
    
    // Wow! You can print the bit mask!
    shiftand.print_bit_mask();

    assert_eq!(patterns_window, [(3, 7), (6, 10)]);
}


```

### 👻 To-do
- [ ]  Use bitvec instead of Vec<bool>
- [ ]  Add docs