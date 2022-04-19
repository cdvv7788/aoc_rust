use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref ALPHA_CHARACTERS: HashMap<char, u8> = HashMap::from([
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
        ('i', 8),
        ('j', 9),
        ('k', 10),
        ('l', 11),
        ('m', 12),
        ('n', 13),
        ('o', 14),
        ('p', 15),
        ('q', 16),
        ('r', 17),
        ('s', 18),
        ('t', 19),
        ('u', 20),
        ('v', 21),
        ('w', 22),
        ('x', 23),
        ('y', 24),
        ('z', 25),
    ]);
}
const BASE: u8 = 26;

fn increment(input: &str) -> String {
    let mut values: Vec<u8> = Vec::new();
    let mut reversed_updated_values: Vec<u8> = Vec::new();
    let mut output: String = "".to_string();

    // first, convert chars into integers
    for c in input.chars() {
        values.push(ALPHA_CHARACTERS[&c]);
    }

    // reverse iteration, increment as needed
    let mut rotated = true;
    for value in values.iter().rev() {
        // if rotated, increment current value further
        let to_increment = if rotated { 1 } else { 0 };
        let new_value = (value + to_increment) % BASE;
        rotated = new_value != value + to_increment;
        reversed_updated_values.push(new_value);
    }
    // If still needs to be rotated, add a new entry
    if rotated {
        reversed_updated_values.push(0);
    }

    //finally, convert back to a chars list
    for value in reversed_updated_values.iter().rev() {
        let mut new_value = '0';
        for (key, alpha_value) in ALPHA_CHARACTERS.iter() {
            if alpha_value == value {
                new_value = *key;
                break;
            }
        }
        if new_value == '0' {
            // probably cleaner to use Option here
            panic!();
        }
        output.push(new_value);
    }
    output
}

fn check_increments(input: &str) -> bool {
    for (char_1, char_2, char_3) in input.chars().tuple_windows::<(char, char, char)>() {
        if ALPHA_CHARACTERS[&char_1] + 1 == ALPHA_CHARACTERS[&char_2]
            && ALPHA_CHARACTERS[&char_2] + 1 == ALPHA_CHARACTERS[&char_3]
        {
            return true;
        }
    }
    false
}

fn check_forbidden_chars(input: &str) -> bool {
    for current_char in input.chars() {
        if current_char == 'i' || current_char == 'o' || current_char == 'l' {
            return false;
        }
    }
    true
}

fn check_non_overlapping_repeated(input: &str) -> bool {
    match input.len() {
        3.. => {
            let mut current_findings = 0;
            let mut repetitions = 0;
            let mut previous_char: char = input.chars().take(1).last().unwrap();
            for current_char in input[1..].chars() {
                if current_char == previous_char {
                    if repetitions == 0 {
                        current_findings += 1;
                    }
                    repetitions += 1;
                }
                if current_char != previous_char {
                    repetitions = 0;
                }
                if current_findings == 2 {
                    return true;
                }
                previous_char = current_char;
            }
            false
        }
        _ => false,
    }
}

fn check_password(input: &str) -> bool {
    check_increments(input) && check_forbidden_chars(input) && check_non_overlapping_repeated(input)
}

fn find_next_password(input: &str) -> String {
    let mut next_password = increment(input);
    loop {
        if check_password(&next_password) {
            return next_password;
        } else {
            if !check_forbidden_chars(&next_password) {
                //needs to skip the whole group with the forbidden characters
                let mut skipped = String::from("");
                let mut truncate = false;
                for c in next_password.chars() {
                    if truncate {
                        skipped.push('a');
                    } else {
                        if c != 'i' && c != 'l' && c != 'o' {
                            // just keep this one
                            skipped.push(c);
                        } else {
                            skipped.push(match c {
                                'i' => 'j',
                                'l' => 'm',
                                _ => 'p',
                            });
                            truncate = true;
                        }
                    }
                }
                next_password = skipped;
            }
            next_password = increment(&next_password);
        }
    }
}

fn main() {
    let input = "vzbxkghb";
    let new_password = find_next_password(input);
    println!("The new password is {}", new_password);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_next_password() {
        assert_eq!(find_next_password("abcdefgh"), "abcdffaa");
        assert_eq!(find_next_password("ghijklmn"), "ghjaabcc");
    }

    #[test]
    fn test_check_password() {
        assert!(!check_password("hijklmmn"));
        assert!(!check_password("abbceffg"));
        assert!(!check_password("abbcegjk"));
        assert!(check_password("abcdffaa"));
        assert!(check_password("ghjaabcc"));
    }

    #[test]
    fn test_check_non_overlapping_repeated() {
        assert!(!check_non_overlapping_repeated("aa"));
        assert!(check_non_overlapping_repeated("aabb"));
        assert!(!check_non_overlapping_repeated("aaa"));
        assert!(!check_non_overlapping_repeated("baaab"));
        assert!(!check_non_overlapping_repeated("abcdeggg"));
    }

    #[test]
    fn test_check_forbidden_chars() {
        assert!(!check_forbidden_chars("hijklmmn"));
        assert!(check_forbidden_chars("hjkmmn"));
    }

    #[test]
    fn test_check_increments() {
        assert!(check_increments("abc"));
        assert!(check_increments("bcd"));
        assert!(check_increments("cde"));
        assert!(check_increments("xyz"));
        assert!(!check_increments("abd"));
    }

    #[test]
    fn test_increment() {
        assert_eq!(increment("x"), String::from("y"));
        assert_eq!(increment("xx"), String::from("xy"));
        assert_eq!(increment("xy"), String::from("xz"));
        assert_eq!(increment("xz"), String::from("ya"));
        assert_eq!(increment("ya"), String::from("yb"));
    }
}
