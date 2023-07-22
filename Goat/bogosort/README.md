# HAHA BOGO
Randomize the array:
* Is it sorted? No? Then Randomize it again 😎

```rust
use bogosort::Bogosort;

fn main() {
    // Haha, try running this one:
    let mut a = Bogosort::new(vec![17, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    a.start();
}
```

Some of the results:
```text
It took: 174798 "randomizations": [2, 3, 4, 5, 10, 17, 20, 102]
It took: 121182 "randomizations": [2, 3, 4, 5, 10, 17, 20, 102]
It took: 5866 "randomizations": [2, 3, 4, 5, 10, 17, 20, 102]
It took: 73797 "randomizations": [2, 3, 4, 5, 10, 17, 20, 102]
```

![Haha bogo](https://img.ifunny.co/images/81b86ba267b2d400621b09125f753a0a12c30f36da0174de4a427f4d8e3a620f_1.jpg "Bogo king")