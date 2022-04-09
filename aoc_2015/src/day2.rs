use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Rectangular{
    height: u32,
    length: u32,
    width : u32,
}

fn calculate_area(rect: &Rectangular) -> u32{
    // 2*l*w + 2*w*h + 2*h*l
    let faces = [rect.length*rect.width, rect.width*rect.height, rect.height*rect.length];
    let min_area = faces.iter().min().unwrap();
    let total_area = faces.iter().map(|x| 2*x).reduce(|a, b| a+b).unwrap();
    total_area + min_area
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(rect_str: String) -> Rectangular {
    let split: Vec<u32> = rect_str.split("x").map(|x: &str|x.parse().unwrap()).collect();
    Rectangular{height: split[0], length: split[1], width: split[2]}
}

fn calculate_ribbon(rect: &Rectangular) -> u32 {
    let bow_size = rect.width*rect.length*rect.height;
    let mut dimensions = [rect.height, rect.length, rect.width];
    dimensions.sort(); // no sorted :(
    let perimeter = 2*dimensions[0] + 2*dimensions[1];
    bow_size + perimeter
}

fn main(){
    let mut parsed_rectangulars: Vec<Rectangular> = Vec::new();
    if let Ok(lines) = read_lines("./input/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {
                //areas.push(calculate_area(parse_line(value))
                parsed_rectangulars.push(parse_line(value));
            }
        }
    }
    println!("total wrapping paper area: {}", parsed_rectangulars.iter().map(|x|calculate_area(x)).sum::<u32>());
    println!("total riboon length: {}", parsed_rectangulars.iter().map(|x|calculate_ribbon(x)).sum::<u32>());
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_calculate_area(){
        assert_eq!(calculate_area(&Rectangular{height: 2,length: 3, width: 4}), 58);
        assert_eq!(calculate_area(&Rectangular{height: 1,length: 1, width: 10}), 43);
    }

    #[test]
    fn test_parse_line(){
        assert_eq!(parse_line(String::from("1x1x10")).width, 10);
    }

    #[test]
    fn test_calculate_ribbon(){
        assert_eq!(calculate_ribbon(&Rectangular{height: 2,length: 3, width: 4}), 34);
        assert_eq!(calculate_ribbon(&Rectangular{height: 1,length: 1, width: 10}), 14);
    }
}
