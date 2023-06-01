# 👾 Knuth–Morris–Pratt algorithm
*String-searching* algorithm that searches for occurrences of a substring within a "main" string

### 📃 Examples
**Seach the first occurence**
```rust
use kmp::Kmp;

fn main() {
    let search = Kmp::new("ARARAQUARA", "ARARA ARARAQUARA");

    // If you want to see the fail_transition table
    search.print_fail_transition();

    // Search first occurence
    match search.search_first() {
        Ok((start, end)) => println!("First occurence found at [{},{}]", start, end),
        Err(e) => println!("{e}"),
    }

    // Search all the occurences
    // to-do 😊

    // Search the occurences from a file path
    // to-do 😊
}

```

### 👻 To-do
- [x]  Return the first occurrence
- [ ]  Return all occurrences
- [ ]  Search a substring from a text file path.
