use lazy_static::lazy_static;
use regex::Regex;

const FILE_TEXT: &str = include_str!("../input/day12.txt");

fn get_numbers_from_json(json: &str) -> Vec<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(-?\d+)[,}\]]").unwrap();
    }
    let mut numbers: Vec<i32> = Vec::new();

    for capture in RE.captures_iter(json) {
        numbers.push(capture[1].parse::<i32>().unwrap());
    }
    numbers
}

fn sum_input(numbers: Vec<i32>) -> i32 {
    numbers.into_iter().fold(0, |a, b| a + b)
}

fn main() {
    let numbers = get_numbers_from_json(FILE_TEXT);
    let total = sum_input(numbers);
    println!("The total sum of numbers is {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_input() {
        assert_eq!(sum_input(vec![1, 2, 3]), 6);
        assert_eq!(sum_input(vec![2, 4]), 6);
        assert_eq!(sum_input(vec![]), 0);
    }

    #[test]
    fn test_get_numbers_from_json() {
        assert_eq!(get_numbers_from_json("[1,2,3]"), vec![1, 2, 3]);
        assert_eq!(get_numbers_from_json(r#"{"a":2,"b":4}"#), vec![2, 4]);
        assert_eq!(get_numbers_from_json("[[[3]]]"), vec![3]);
        assert_eq!(
            get_numbers_from_json(r#"{"a":{"b":4},"c":-1}"#),
            vec![4, -1]
        );
        assert_eq!(get_numbers_from_json(r#"{"a":[-1,1]}"#), vec![-1, 1]);
        assert_eq!(get_numbers_from_json(r#"[-1,{"a":1}]"#), vec![-1, 1]);
        assert_eq!(get_numbers_from_json("[]"), vec![]);
        assert_eq!(get_numbers_from_json("{}"), vec![]);
    }
}
