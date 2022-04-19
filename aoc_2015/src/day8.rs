const FILE_TEXT: &str = include_str!("../input/day8.txt");

fn count_characters(line: &str) -> u32 {
    // very lazy hardcoded approach
    let count_chars = line.len();
    let mut count_escaped: i32 = 0;
    let mut previous = 'x';
    for current_char in line.chars() {
        count_escaped += 1;
        if current_char == 'x' {
            if previous == '\\' {
                count_escaped -= 3;
            }
        }
        if current_char == '\\' && previous == '\\' {
            count_escaped -= 1;
            previous = 'x';
            continue;
        }
        if current_char == '\"' {
            count_escaped -= 1;
        }
        previous = current_char;
    }
    count_chars as u32 - count_escaped as u32
}

fn encode_line(line: &str) -> u32 {
    let mut count_encoded = 2;
    for current_char in line.chars() {
        count_encoded += 1;
        if current_char == '\\' || current_char == '\"' {
            count_encoded += 1;
        }
    }
    count_encoded - line.len() as u32
}

fn sum_lines(strings: &str) -> u32 {
    strings
        .lines()
        .map(|x| count_characters(x))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn sum_lines_2(strings: &str) -> u32 {
    strings
        .lines()
        .map(|x| encode_line(x))
        .reduce(|a, b| a + b)
        .unwrap()
}

fn main() {
    let output = sum_lines_2(FILE_TEXT);
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_line() {
        let str_1 = "\"\"";
        let str_2 = "\"abc\"";
        let str_3 = "\"aaa\\\"aaa\"";
        let str_4 = "\"\\x27\"";
        let str_5 = "\"\\\"";

        assert_eq!(encode_line(str_1), 4);
        assert_eq!(encode_line(str_2), 4);
        assert_eq!(encode_line(str_3), 6);
        assert_eq!(encode_line(str_4), 5);
        assert_eq!(encode_line(str_5), 5);
    }

    #[test]
    fn test_count_characters() {
        let str_1 = "\"\"";
        let str_2 = "\"abc\"";
        let str_3 = "\"aaa\\\"aaa\"";
        let str_4 = "\"\\x27\"";
        let str_5 = "\"\\\"";

        assert_eq!(count_characters(str_1), 2);
        assert_eq!(count_characters(str_2), 2);
        assert_eq!(count_characters(str_3), 3);
        assert_eq!(count_characters(str_4), 5);
        assert_eq!(count_characters(str_5), 2);
    }
}
