use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::default::Default;
use regex::Regex;
use lazy_static::lazy_static;

struct Grid (
    Box<[[u8; 1000]; 1000]>
);

struct Rectangle {
    bottom_left: Point,
    top_right: Point,
}

struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Operation{
    On,
    Off,
    Toggle,
}

impl Default for Grid {
    fn default() -> Self {
        Grid(Box::new([[0; 1000]; 1000]))
    }
}

fn count_grid_on(grid: &mut Grid) -> u32 {
    let mut counter: u32 = 0;
    for element in grid.0.iter(){
        for inner_element in element.iter(){
            counter += *inner_element as u32;
        }
    }
    counter
}

fn turn_grid_to_value(grid: &mut Grid, rectangle: &Rectangle, operation: Operation) {
    for x in rectangle.bottom_left.x..=rectangle.top_right.x {
        for y in rectangle.bottom_left.y..=rectangle.top_right.y {
            let delta: i32 = match operation {
                Operation::On => 1,
                Operation::Off => -1,
                Operation::Toggle => 2,
            };
            let value: u8 = if (grid.0[x][y] as i32) + delta >= 0 {
                (grid.0[x][y] as i32 + delta) as u8
            } else {
                0
            };
            grid.0[x][y] = value;
        }
    }
}

fn parse_line(instruction: String) -> (Operation, Rectangle) {
    // compiled a single time
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(turn on|turn off|toggle) ([0-9]*),([0-9]*) through ([0-9]*),([0-9]*)").unwrap();
    }
    // need to consume the iterator, even if we expect a single match
    for capture in RE.captures_iter(&instruction[..]){
        let operation = match &capture[1] {
            "turn on" => Operation::On,
            "turn off" => Operation::Off,
            _ => Operation::Toggle,
        };
        let rectangle = Rectangle{bottom_left: Point{x: capture[2].parse().unwrap(), y: capture[3].parse().unwrap()},
                                  top_right: Point{x: capture[4].parse().unwrap(), y: capture[5].parse().unwrap()}};
        return (operation, rectangle)
    }
    panic!()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main(){
    let mut grid: Grid = Default::default();
    if let Ok(lines) = read_lines("./input/day6.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {
                let (operation, rectangle) = parse_line(value);
                turn_grid_to_value(&mut grid, &rectangle, operation);
            }
        }
    }
    println!("Lights on: {}", count_grid_on(&mut grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_grid_on() {
        assert_eq!(count_grid_on(&mut Default::default()), 0);
        assert_eq!(count_grid_on(&mut Grid(Box::new([[1;1000];1000]))), 1000000);
    }

    #[test]
    fn test_turn_grid_to_value() {
        // testing with side-effects...bad. Returning a new grid with the new values would be cleaner...and more expensive
        let mut grid = Default::default();
        turn_grid_to_value(&mut grid, &Rectangle{bottom_left: Point{x: 0, y: 0}, top_right: Point{x: 999, y: 999}}, Operation::On);
        assert_eq!(count_grid_on(&mut grid), 1000000);
        turn_grid_to_value(&mut grid, &Rectangle{bottom_left: Point{x: 0, y: 0}, top_right: Point{x: 499, y: 999}}, Operation::Off);
        assert_eq!(count_grid_on(&mut grid), 500000);
        turn_grid_to_value(&mut grid, &Rectangle{bottom_left: Point{x: 0, y: 0}, top_right: Point{x: 999, y: 499}}, Operation::Toggle);
        assert_eq!(count_grid_on(&mut grid), 1500000); //bad test, not very accurate
    }

    #[test]
    fn test_parse_line() {
        // test toggle
        let (operation, rectangle) = parse_line(String::from("toggle 678,333 through 752,957"));
        assert_eq!(Operation::Toggle, operation);
        assert_eq!(rectangle.bottom_left.x, 678);
        assert_eq!(rectangle.bottom_left.y, 333);
        assert_eq!(rectangle.top_right.x, 752);
        assert_eq!(rectangle.top_right.y, 957);

        // test on
        let (operation, rectangle) = parse_line(String::from("turn on 150,20 through 652,719"));
        assert_eq!(Operation::On, operation);
        assert_eq!(rectangle.bottom_left.x, 150);
        assert_eq!(rectangle.bottom_left.y, 20);
        assert_eq!(rectangle.top_right.x, 652);
        assert_eq!(rectangle.top_right.y, 719);

        //test off
        let (operation, rectangle) = parse_line(String::from("turn off 782,143 through 808,802"));
        assert_eq!(Operation::Off, operation);
        assert_eq!(rectangle.bottom_left.x, 782);
        assert_eq!(rectangle.bottom_left.y, 143);
        assert_eq!(rectangle.top_right.x, 808);
        assert_eq!(rectangle.top_right.y, 802);
    }
}