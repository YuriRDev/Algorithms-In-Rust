/// Add docs later for compression
pub fn compress_rle(text: &str) -> String {
    let mut count = 0;
    let mut return_string = String::new();

    while count != text.len() {
        let current_char = text.bytes().nth(count).unwrap() as char;
        let equals = count_equals(text, count);

        // Did not find a better solution for this...
        return_string += &format!("{}{}", equals, current_char);

        count += equals as usize;
    }

    return_string
}

/// Add docs later for decompression
fn decompres_rle(compressed: Vec<(u32, char)>) -> String {
    let mut return_string = String::new();

    for (quantity, character) in compressed {
        for _b in 0..quantity {
            return_string.push(character);
        }
    }

    return_string
}

/// Add docs later for compression as vec
fn compress_rle_as_vec(text: &str) -> Vec<(u32, char)> {
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

/// Add docs later for decompression from vec
fn decompres_rle_from_vec(compressed: Vec<(u32, char)>) -> String {
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

    }

    #[test]
    fn test_compression_medium() {

    }

    #[test]
    fn test_compression_big() {

    }
}
