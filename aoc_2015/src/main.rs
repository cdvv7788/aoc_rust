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
                checked_strings.push(check_nice_string(value));
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
}