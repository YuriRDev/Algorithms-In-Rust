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

![Haha bogo](https://preview.redd.it/all-my-homies-use-bogosort-v0-ilnfldkc28ka1.jpg?auto=webp&s=c6df3a1d3d377f08324b8c5b87ef92caf008e5ff "Bogo king")
