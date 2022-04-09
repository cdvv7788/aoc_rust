
fn check_signature(target: &String) -> bool {
    let mut to_match = target.chars();
    let mut counter = 0;
    for _ in 0..5 {
        if (to_match).next().unwrap() == '0' {
            counter += 1;
            if counter == 5 {
                return true
            }
        }
    }
    false
}

fn mine_adventcoin(secret_key: &String) -> i32{
    let mut decimal = 0;
    loop {
        let to_test = format!("{}{}", secret_key, decimal);
        let digest = format!("{:x}", md5::compute(to_test.as_bytes()));
        if check_signature(&digest) {
            return decimal
        }
        decimal += 1;
    }
}

fn main() {
    let test_str = mine_adventcoin(&String::from("yzbqklnj"));
    println!("the key is: {}", test_str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_hash() {
        assert_eq!(mine_adventcoin(&String::from("abcdef")), 609043);
        assert_eq!(mine_adventcoin(&String::from("pqrstuv")), 1048970);
    }
}
