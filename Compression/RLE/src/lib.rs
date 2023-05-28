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
    fn small_test() {
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
