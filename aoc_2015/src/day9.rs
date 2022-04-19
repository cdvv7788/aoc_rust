use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::{HashMap, HashSet};

const FILE_TEXT: &str = include_str!("../input/day9.txt");

#[derive(Eq, PartialEq, Debug)]
struct Distance {
    origin: String,
    destination: String,
    distance: u32,
}

impl From<&str> for Distance {
    fn from(data: &str) -> Self {
        lazy_static! {
            static ref DISTANCE_RE: Regex =
                Regex::new(r"^([a-zA-Z]*)\sto\s([a-zA-Z]*)\s=\s([0-9]*)$").unwrap();
        }
        for cap in DISTANCE_RE.captures_iter(data) {
            return Distance {
                origin: cap[1].to_string(),
                destination: cap[2].to_string(),
                distance: cap[3].parse::<u32>().unwrap(),
            };
        }
        panic!()
    }
}

fn parse_list_into_distance(distances_str: &str) -> Vec<Distance> {
    let mut distances: Vec<Distance> = Vec::new();
    for line in distances_str.lines() {
        distances.push(Distance::from(line));
    }
    distances
}

fn construct_distance_table(distances: &Vec<Distance>) -> HashMap<(String, String), u32> {
    let mut distances_map = HashMap::new();
    for distance in distances {
        distances_map.insert(
            (distance.origin.clone(), distance.destination.clone()),
            distance.distance,
        );
        distances_map.insert(
            (distance.destination.clone(), distance.origin.clone()),
            distance.distance,
        );
    }
    distances_map
}

fn get_all_locations(distances: &Vec<Distance>) -> HashSet<String> {
    let mut cities = HashSet::new();
    for distance in distances {
        cities.insert(distance.origin.clone());
        cities.insert(distance.destination.clone());
    }
    cities
}

fn calculate_shortest_distance(
    cities: &HashSet<String>,
    distances_map: &HashMap<(String, String), u32>,
) -> u32 {
    let mut min_distance = 0;
    for route in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        for (city_a, city_b) in route.iter().tuple_windows() {
            distance += distances_map[&(String::from(*city_a), String::from(*city_b))];
        }
        if min_distance == 0 {
            min_distance = distance;
        } else {
            min_distance = cmp::min(min_distance, distance);
        }
    }
    min_distance
}

fn calculate_longest_distance(
    cities: &HashSet<String>,
    distances_map: &HashMap<(String, String), u32>,
) -> u32 {
    let mut max_distance = 0;
    for route in cities.iter().permutations(cities.len()) {
        let mut distance = 0;
        for (city_a, city_b) in route.iter().tuple_windows() {
            distance += distances_map[&(String::from(*city_a), String::from(*city_b))];
        }
        max_distance = cmp::max(max_distance, distance);
    }
    max_distance
}

fn main() {
    let distances = parse_list_into_distance(FILE_TEXT);
    let distance_table = construct_distance_table(&distances);
    let cities = get_all_locations(&distances);
    let min_distance = calculate_shortest_distance(&cities, &distance_table);
    let max_distance = calculate_longest_distance(&cities, &distance_table);
    println!(
        "min_distance is {} and max_distance is {}",
        min_distance, max_distance
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_longest_distance() {
        let mut cities = HashSet::new();
        cities.insert(String::from("London"));
        cities.insert(String::from("Dublin"));
        cities.insert(String::from("Belfast"));

        let mut map = HashMap::new();
        map.insert((String::from("London"), String::from("Dublin")), 464);
        map.insert((String::from("Dublin"), String::from("London")), 464);
        map.insert((String::from("London"), String::from("Belfast")), 518);
        map.insert((String::from("Belfast"), String::from("London")), 518);
        map.insert((String::from("Dublin"), String::from("Belfast")), 141);
        map.insert((String::from("Belfast"), String::from("Dublin")), 141);

        assert_eq!(calculate_longest_distance(&cities, &map), 982);
    }

    #[test]
    fn test_calculate_shortest_distance() {
        let mut cities = HashSet::new();
        cities.insert(String::from("London"));
        cities.insert(String::from("Dublin"));
        cities.insert(String::from("Belfast"));

        let mut map = HashMap::new();
        map.insert((String::from("London"), String::from("Dublin")), 464);
        map.insert((String::from("Dublin"), String::from("London")), 464);
        map.insert((String::from("London"), String::from("Belfast")), 518);
        map.insert((String::from("Belfast"), String::from("London")), 518);
        map.insert((String::from("Dublin"), String::from("Belfast")), 141);
        map.insert((String::from("Belfast"), String::from("Dublin")), 141);

        assert_eq!(calculate_shortest_distance(&cities, &map), 605);
    }

    #[test]
    fn test_get_all_locations() {
        let mut distances = Vec::new();
        distances.push(Distance {
            origin: String::from("London"),
            destination: String::from("Dublin"),
            distance: 464,
        });
        distances.push(Distance {
            origin: String::from("London"),
            destination: String::from("Belfast"),
            distance: 518,
        });
        distances.push(Distance {
            origin: String::from("Dublin"),
            destination: String::from("Belfast"),
            distance: 141,
        });
        let mut cities = HashSet::new();
        cities.insert(String::from("London"));
        cities.insert(String::from("Dublin"));
        cities.insert(String::from("Belfast"));
        assert_eq!(get_all_locations(&distances), cities)
    }

    #[test]
    fn test_construct_distance_table() {
        let mut distances = Vec::new();
        distances.push(Distance {
            origin: String::from("London"),
            destination: String::from("Dublin"),
            distance: 464,
        });
        distances.push(Distance {
            origin: String::from("London"),
            destination: String::from("Belfast"),
            distance: 518,
        });
        distances.push(Distance {
            origin: String::from("Dublin"),
            destination: String::from("Belfast"),
            distance: 141,
        });
        let mut map = HashMap::new();
        map.insert((String::from("London"), String::from("Dublin")), 464);
        map.insert((String::from("Dublin"), String::from("London")), 464);
        map.insert((String::from("London"), String::from("Belfast")), 518);
        map.insert((String::from("Belfast"), String::from("London")), 518);
        map.insert((String::from("Dublin"), String::from("Belfast")), 141);
        map.insert((String::from("Belfast"), String::from("Dublin")), 141);
        assert_eq!(construct_distance_table(&distances), map);
    }

    #[test]
    fn test_distance_from_string_list() {
        let str_1 = "London to Dublin = 464";
        let str_2 = "London to Belfast = 518";
        let str_3 = "Dublin to Belfast = 141";
        let final_str = format!("{}\n{}\n{}", str_1, str_2, str_3);

        let mut distances: Vec<Distance> = Vec::new();
        distances.push(Distance::from(str_1));
        distances.push(Distance::from(str_2));
        distances.push(Distance::from(str_3));
        assert_eq!(parse_list_into_distance(&final_str), distances);
    }

    #[test]
    fn test_create_distance() {
        let str_1 = "London to Dublin = 464";
        let str_2 = "London to Belfast = 518";
        let str_3 = "Dublin to Belfast = 141";

        assert_eq!(
            Distance::from(str_1),
            Distance {
                origin: String::from("London"),
                destination: String::from("Dublin"),
                distance: 464
            }
        );
        assert_eq!(
            Distance::from(str_2),
            Distance {
                origin: String::from("London"),
                destination: String::from("Belfast"),
                distance: 518
            }
        );
        assert_eq!(
            Distance::from(str_3),
            Distance {
                origin: String::from("Dublin"),
                destination: String::from("Belfast"),
                distance: 141
            }
        );
    }
}
