use md5::{Md5, Digest};

fn check_signature(target: &[u8]) -> bool {
    // this is a list of u8 (bytes) so all we need to check is if the first
    // bytes are zero, and the third one is below 16 (00001111)
    // When looking for the 6 zeroes, just check for the 3 first bytes == 0
    //target[0] == 0 && target[1] == 0 && target[2] == 16
    target[0] == 0 && target[1] == 0 && target[2] == 0
}

fn mine_adventcoin(secret_key: &String) -> i32{
    let mut decimal = 0;
    loop {
        let to_test = format!("{}{}", secret_key, decimal);
        let mut hasher = Md5::new();
        hasher.input(to_test.as_bytes()); //ewwww
        let digest = hasher.result();
        if check_signature(&digest[0..=2]) {
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
