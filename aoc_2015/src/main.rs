use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;


fn check_nice_string(to_test: String) -> bool {
    // A nice string is one with all of the following properties:
    // It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    // It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    // It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let forbidden_combinations = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
    let mut found_vowels: Vec<char> = Vec::new();
    let mut repeated_char = false;
    let mut found_forbidden = false;

    for (key, (a, b)) in to_test.chars().tuple_windows().enumerate(){
        // Check for vowels
        if key == 0 {
            if vowels.contains(&a) {
                found_vowels.push(a);
            }
        }
        if vowels.contains(&b) {
            found_vowels.push(b);
        }

        // check for repeated characters
        if !repeated_char {
            if a == b {
                repeated_char = true;
            }
        }

        // check for forbidden combinations
        if !found_forbidden {
            for combination in forbidden_combinations {
                if (a, b) == combination {
                    found_forbidden = true;
                }
            }
        }
    }
    found_vowels.len() >= 3 && repeated_char && !found_forbidden
}

fn check_nice_string_v2(to_test: String) -> bool {
    // Now, a nice string is one with all of the following properties:
    // It contains a pair of any two letters that appears at least twice in the string without overlapping,
    // like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    // It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
    let mut found_match = false;
    let mut found_overlap = true;
    let mut found_repeated_char = false;
    for (key_1, (a, b)) in to_test.chars().tuple_windows().enumerate(){
        let to_test_with_offset = &to_test[key_1 + 1..];
        for (key_2, (c, d)) in to_test_with_offset.chars().tuple_windows().enumerate(){
            // Check for matches and overlaps in the whole string
            // key_2 starts 1 element after key_1. If key_2 is zero, it means the windows are overlapping.
            // println!("({},{}) - ({}, {}) - ({}, {})", a, b, c, d, key_1, key_2);
            if (a, b) == (c, d) {
                found_match = true;
                if key_2 != 0 { 
                    found_overlap = false;
                }
            }

            // Check for repeated chars with an element in the middle
            if key_2 == 0 && a == d {
                found_repeated_char = true;
            }

        }
    }
    found_match && !found_overlap && found_repeated_char
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main(){
    let mut checked_strings: Vec<bool> = Vec::new();
    if let Ok(lines) = read_lines("./input/day5.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {
                // checked_strings.push(check_nice_string(value));
                checked_strings.push(check_nice_string_v2(value));
            }
        }
    }

    let nice_strings = checked_strings.into_iter().map(|x|if x { 1 } else { 0 }).reduce(|a,b|a+b).unwrap();
    println!("{} nice strings were found", nice_strings);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_nice_string() {
        assert!(check_nice_string(String::from("ugknbfddgicrmopn")));
        assert!(check_nice_string(String::from("aaa")));
        assert!(!check_nice_string(String::from("jchzalrnumimnmhp")));
        assert!(!check_nice_string(String::from("haegwjzuvuyypxyu")));
        assert!(!check_nice_string(String::from("dvszwmarrgswjxmb")));
    }

    #[test]
    fn test_check_nice_string_v2() {
        assert!(check_nice_string_v2(String::from("qjhvhtzxzqqjkmpb")));
        assert!(check_nice_string_v2(String::from("xxyxx")));
        assert!(check_nice_string_v2(String::from("xyxy")));
        assert!(check_nice_string_v2(String::from("aaaa"))); // had trouble figuring out this case was missing
        assert!(check_nice_string_v2(String::from("abaaaa")));
        assert!(!check_nice_string_v2(String::from("uurcxstgmygtbstg")));
        assert!(!check_nice_string_v2(String::from("ieodomkazucvgmuy")));
        assert!(!check_nice_string_v2(String::from("aaa")));
    }
}