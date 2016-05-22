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

    for i in 0..source.len() {
        let cur_char = source.chars().nth(i).unwrap();
        match source.chars().nth(i + 1) {
            Some(next_char) => {
                if cur_char == next_char {
                    count += 1;
                } else {
                    count += 1;
                    compressed.push(cur_char);
                    compressed.push_str(&count.to_string());
                    count = 0;
                }
            },
            None => { }
        }
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
}
