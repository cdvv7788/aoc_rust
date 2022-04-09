use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn calculate_delta(x: char) -> i32 {
    match x {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

fn count_parenthesis(address: &String) -> i32 {
    let count = &address
        .chars()
        .map(|x: char| -> i32 {
            calculate_delta(x)
        })
        .reduce(|a, b| a + b);
    count.unwrap()
}

fn find_basement_position(address: &String) -> u32 {
    let mut current_address = 0;
    for (position, value) in address.chars().enumerate() {
        current_address += calculate_delta(value);
        if current_address == -1 {
            return (position+1) as u32
        }
    }
    panic!("Never reached the basement");
}

fn main(){
    let path = Path::new("./input/day1.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut test_str = String::new();
    match file.read_to_string(&mut test_str) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    println!("The final floor is: {}", count_parenthesis(&test_str));
    println!("The first basement entry position is: {}", find_basement_position(&test_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_parenthesis(){
        assert_eq!(count_parenthesis(&String::from("(())")), 0);
        assert_eq!(count_parenthesis(&String::from("()()")), 0);
        assert_eq!(count_parenthesis(&String::from("(((")), 3);
        assert_eq!(count_parenthesis(&String::from("(()(()(")), 3);
        assert_eq!(count_parenthesis(&String::from("))(((((")), 3);
        assert_eq!(count_parenthesis(&String::from("())")), -1);
        assert_eq!(count_parenthesis(&String::from("))(")), -1);
        assert_eq!(count_parenthesis(&String::from(")))")), -3);
        assert_eq!(count_parenthesis(&String::from(")())())")), -3);
    }

    #[test]
    fn test_basement_position(){
        assert_eq!(find_basement_position(&String::from(")")), 1);
        assert_eq!(find_basement_position(&String::from("()())")), 5);
    }
}
