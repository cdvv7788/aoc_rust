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

fn calculate_visits_v2(movements: &String) -> u32 {
    let mut visited: HashSet<String> = HashSet::new();
    let mut current_location_santa = Location(0, 0);
    let mut current_location_robo = Location(0, 0);
    visited.insert(String::from(&current_location_santa));

    let mut is_santa_turn = true;
    for movement in movements.chars() {
        if is_santa_turn {
            current_location_santa = calculate_location(&current_location_santa, movement);
            visited.insert(String::from(&current_location_santa));
            is_santa_turn = false;
        } else {
            current_location_robo = calculate_location(&current_location_robo, movement);
            visited.insert(String::from(&current_location_robo));
            is_santa_turn = true;
        }
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

    println!("visits for santa alone: {}", calculate_visits(&test_str));
    println!("visits for santa with robo help: {}", calculate_visits_v2(&test_str));
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

    #[test]
    fn test_calculate_visits_v2(){
        assert_eq!(calculate_visits_v2(&Strings::from("^v")), 3);
        assert_eq!(calculate_visits_v2(&Strings::from("^>v<")), 3);
        assert_eq!(calculate_visits_v2(&Strings::from("^v^v^v^v^v")), 11);
    }
}