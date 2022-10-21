/// Now, you need to figure out how to pilot this thing.
///
/// It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:
///
/// forward X increases the horizontal position by X units.
/// down X increases the depth by X units.
/// up X decreases the depth by X units.
/// Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.
///
/// The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:
///
/// forward 5
/// down 5
/// forward 8
/// up 3
/// down 8
/// forward 2
/// Your horizontal position and depth both start at 0. The steps above would then modify them as follows:
///
/// forward 5 adds 5 to your horizontal position, a total of 5.
/// down 5 adds 5 to your depth, resulting in a value of 5.
/// forward 8 adds 8 to your horizontal position, a total of 13.
/// up 3 decreases your depth by 3, resulting in a value of 2.
/// down 8 adds 8 to your depth, resulting in a value of 10.
/// forward 2 adds 2 to your horizontal position, a total of 15.
/// After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
///
/// Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?


use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::default::Default;
use std::str::FromStr;
use std::fmt;


type DirectionResult<T> = Result<T, ValueError>;


#[derive(Debug, Clone, PartialEq)]
struct ValueError;


impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Got an invalid value for Direction!")
    }
}


#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down
}

impl FromStr for Direction {
    type Err = ValueError;

    fn from_str(s: &str) -> DirectionResult<Self> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(ValueError)
        }
    }
}


#[derive(Default)]
struct Position {
    x: i32,
    depth: i32
}

impl Position {

    pub fn up(&mut self, val: i32) {
        self.depth -= val;
    }

    pub fn down(&mut self, val: i32) {
        self.depth += val;
    }

    pub fn forward(&mut self, val: i32) {
        self.x += val;
    }
}

pub fn main() -> io::Result<()> {
    let mut sub: Position = Default::default();

    let fh = File::open("data/day2.txt")?;

    let reader = BufReader::new(fh);

    for line in reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.trim().is_empty())
    {

        let mut chunk = line.split_whitespace();

        let (dir, amt) = (chunk.next(), chunk.next());

        let amt = amt
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let d = Direction::from_str(
            dir.unwrap()
        );

        match d {
            Ok(Direction::Up) => sub.up(amt),
            Ok(Direction::Down) => sub.down(amt),
            Ok(Direction::Forward) => sub.forward(amt),
            _ => panic!("Got a weird result!")
        };
    }

    println!("Coords for sub are:");
    println!("x: {:?}, depth: {:?}", sub.x, sub.depth);
    println!("Product is: {:?}", sub.x * sub.depth);
    Ok(())
}


#[cfg(test)]
#[test]
fn test_sub_default() {
    let sub: Position = Default::default();
    assert_eq!((sub.x, sub.depth), (0, 0));
}


#[cfg(test)]
#[test]
fn test_sub_directions() {
    let mut sub: Position = Default::default();

    sub.forward(1);
    sub.up(1);
    sub.down(1);

    assert_eq!((sub.x, sub.depth), (1, 0));
}



#[cfg(test)]
#[test]
fn test_direction_parsing() {
    for (value, expected) in vec![
        ("up", Direction::Up),
        ("down", Direction::Down),
        ("forward", Direction::Forward)
    ].iter() {
        let direction = Direction::from_str(value).unwrap();
        assert_eq!(&direction, expected);
    }
}

#[cfg(test)]
#[test]
fn test_amt_parsing() {
    assert!(true);
}
