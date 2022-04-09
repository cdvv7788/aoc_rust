use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Rectangular{
    height: u32,
    length: u32,
    width : u32,
}

fn calculate_area(rect: Rectangular) -> u32{
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

fn main(){
    let mut areas: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines("./input/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {
                areas.push(calculate_area(parse_line(value)));
            }
        }
    }
    println!("{:?}", areas.iter().sum::<u32>());
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_calculate_area(){
        assert_eq!(calculate_area(Rectangular{height: 2,length: 3,width: 4}), 58);
        assert_eq!(calculate_area(Rectangular{height: 1,length: 1,width: 10}), 43);
    }

    #[test]
    fn test_parse_line(){
        assert_eq!(parse_line(String::from("1x1x10")).width, 10);
    }
}
