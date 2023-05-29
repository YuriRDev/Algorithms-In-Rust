/// Add docs later for compression
pub fn compress_rle(text: &str) -> Vec<(u32, char)> {
    let mut count = 0;

    let mut compressed: Vec<(u32, char)> = Vec::new();

    while count != text.len() {
        let current_char = text.bytes().nth(count).unwrap() as char;
        let equals = count_equals(text, count);

        compressed.push((equals, current_char));

        count += equals as usize;
    }

    compressed
}

/// Add docs later for decompression
pub fn decompres_rle(compressed: Vec<(u32, char)>) -> String {
    let mut return_string = String::new();

    for (quantity, character) in compressed {
        for _b in 0..quantity {
            return_string.push(character);
        }
    }

    return_string
}

/// Add docs later for count_equal, not necessary, but why not
fn count_equals(text: &str, count: usize) -> u32 {
    let current_char = text.bytes().nth(count).unwrap() as char;

    if count + 1 == text.len() {
        return 1;
    }

    if current_char == text.bytes().nth(count + 1).unwrap() as char {
        return 1 + count_equals(text, count + 1);
    }

    1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compression_small() {
        let answer: Vec<(u32, char)> = Vec::from([
            (5, 'A'),
            (6, 'B'),
            (8, 'C'),
            (2, 'D'),
            (9, '0'),
            (3, 'L'),
            (1, 'B'),
            (2, 'C'),
        ]);
        assert_eq!(answer, compress_rle("AAAAABBBBBBCCCCCCCCDD000000000LLLBCC"));
    }

    #[test]
    fn test_compression_medium() {
        let answer: Vec<(u32, char)> = Vec::from([
            (5, 'A'),
            (6, 'B'),
            (8, 'C'),
            (2, 'D'),
            (9, '0'),
            (3, 'L'),
            (1, 'B'),
            (2, 'C'),
        ]);
        assert_eq!(answer, compress_rle("AAAAABBBBBBCCCCCCCCDD000000000LLLBCC"));
    }

    #[test]
    fn test_compression_big() {
        let answer: Vec<(u32, char)> = Vec::from([
            (5, 'A'),
            (6, 'B'),
            (8, 'C'),
            (2, 'D'),
            (9, '0'),
            (3, 'L'),
            (1, 'B'),
            (2, 'C'),
        ]);
        assert_eq!(answer, compress_rle("AAAAABBBBBBCCCCCCCCDD000000000LLLBCC"));
    }
}
