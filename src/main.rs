// Implement a method to perform basic string compression using the counts of repeated characters.
// For example, the string aabcccccaaa would become a2b1c5a3. If the "compressed" string would not
// become smaller than the original string, your method should return the original string. You can
// assume the string has only upper and lower case letters (a - z).

use std::io::{self, Write};

fn main() {
    print!("Input: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(error) => println!("error: {}", error),
    }

    println!("{:?}", compress(&input));
}

pub fn compress(source: &str) -> String {
    let mut count = 0;
    let mut compressed = String::new();

    let mut prev_char = source.chars().nth(0).unwrap();
    for cur_char in source.chars().skip(1) {
        if prev_char == cur_char {
            count += 1;
        } else {
            count += 1;
            compressed.push(prev_char);
            compressed.push_str(&count.to_string());
            count = 0;
        }

        prev_char = cur_char;
    }

    if source.len() < compressed.len() {
        source.to_string()
    } else {
        compressed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compresses() {
        let input = "aaaabbbbbcccc\n";
        assert_eq!(compress(input), "a4b5c4".to_string());
    }

    #[test]
    fn test_compresses_one_char() {
        let input = "a\n";
        assert_eq!(compress(input), "a1".to_string());
    }

    #[test]
    fn test_returns_orig_if_compressed_is_bigger() {
        let input = "abcdefghijk\n";
        assert_eq!(compress(input), input.to_string());
    }

    #[test]
    fn test_multibyte_chars() {
        let input = "真棒棒棒棒棒棒棒\n";
        assert_eq!(compress(input), "真1棒7".to_string());
    }
}
