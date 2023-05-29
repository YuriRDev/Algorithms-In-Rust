# Run-Length Encoding
Lossless data compression. 

## Compression from:
- [x] "Hard" writed text *(in code)*
- [ ] CLI Text
- [ ] CLI File


## Examples & Notes
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
