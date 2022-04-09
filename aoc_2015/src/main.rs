use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn count_parenthesis(address: String) -> i32 {
    let count = address
        .chars()
        .map(|x: char| -> i32 {
            match x {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        })
        .reduce(|a, b| a + b);
    count.unwrap()
}

fn main() {
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

    println!("The final floor is: {}", count_parenthesis(test_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($fn_name:ident, $test_str:expr, $expected_output:expr) => {

            #[test]
            fn $fn_name() {
                    assert_eq!(count_parenthesis($test_str), $expected_output);
            }
        }
    }

    test!(test_0_0, String::from("(())"), 0);
    test!(test_0_1, String::from("()()"), 0);
    test!(test_3_0, String::from("((("), 3);
    test!(test_3_1, String::from("(()(()("), 3);
    test!(test_3_2, String::from("))((((("), 3);
    test!(test_minus_1_0, String::from("())"), -1);
    test!(test_minus_1_1, String::from("))("), -1);
    test!(test_minus_3_0, String::from(")))"), -3);
    test!(test_minus_3_1, String::from(")())())"), -3);

}
