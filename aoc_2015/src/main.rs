use std::collections::HashSet;
use std::convert::From;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Location(
    i32, i32
);

impl From<&Location> for String {
    fn from(item: &Location) -> Self {
        format!("{},{}", &item.0, &item.1)
    }
}

fn calculate_location(current_location: &Location, direction: char) ->  Location {
    match direction {
        '<' => Location((&current_location).0 - 1, (&current_location).1),
        '^' => Location((&current_location).0, (&current_location).1 + 1),
        '>' => Location((&current_location).0 + 1, (&current_location).1),
        'v' => Location((&current_location).0, (&current_location).1 - 1),
        _ => Location((&current_location).0, (&current_location).1),
    }
}

fn calculate_visits(movements: &String) -> u32{
    let mut visited: HashSet<String> = HashSet::new();
    let mut current_location = Location(0, 0);
    visited.insert(String::from(&current_location));

    for movement in movements.chars() {
        current_location = calculate_location(&current_location, movement);
        visited.insert(String::from(&current_location));
    }

    visited.len() as u32

}

fn main(){
    let path = Path::new("./input/day3.txt");
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

    println!("{}", calculate_visits(&test_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_visits(){
        assert_eq!(calculate_visits(&String::from(">")), 2);
        assert_eq!(calculate_visits(&String::from("^>v<")), 4);
        assert_eq!(calculate_visits(&String::from("^v^v^v^v^v")), 2);
    }
}