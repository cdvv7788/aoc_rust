fn parse_input(input: String) -> String {
    let mut previous_char = 'x';
    let mut previous_count: u8 = 1;
    let mut output: String = "".to_string();
    for (current_pos, current_char) in input.chars().enumerate() {
        if current_pos == 0 {
            previous_char = current_char;
        }
        // Conditions to move the count and digit into the output
        // 1. The current_char is different than the previous one
        if current_char != previous_char {
            output.push_str(&previous_count.to_string());
            output.push(previous_char);
            previous_count = 1;
        }

        // Conditions to increment counter
        // 1. previous_char is equal to current_char
        // 2. not the first char in the string
        if current_char == previous_char && current_pos != 0 {
            previous_count += 1;
        }
        previous_char = current_char;
    }
    // move one last time
    output.push_str(&previous_count.to_string());
    output.push(previous_char);
    output
}
fn main() {
    let mut input = "1113222113".to_string();

    for _ in 0..40 {
        //println!("current output is {}", input);
        input = parse_input(input);
    }

    println!("length of result is {}", input.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(String::from("1")), String::from("11"));
        assert_eq!(parse_input(String::from("11")), String::from("21"));
        assert_eq!(parse_input(String::from("21")), String::from("1211"));
        assert_eq!(parse_input(String::from("1211")), String::from("111221"));
        assert_eq!(parse_input(String::from("111221")), String::from("312211"));
    }
}
