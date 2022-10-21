/// Part 2:
/// Based on your calculations, the planned course doesn't seem to make any sense. You find the submarine manual and discover that the process is actually slightly more complicated.
///
/// In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:
///
/// down X increases your aim by X units.
/// up X decreases your aim by X units.
/// forward X does two things:
/// It increases your horizontal position by X units.
/// It increases your depth by your aim multiplied by X.
/// Again note that since you're on a submarine, down and up do the opposite of what you might expect: "down" means aiming in the positive direction.
///
/// Now, the above example does something different:
///
/// forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
/// down 5 adds 5 to your aim, resulting in a value of 5.
/// forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
/// up 3 decreases your aim by 3, resulting in a value of 2.
/// down 8 adds 8 to your aim, resulting in a value of 10.
/// forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.
/// After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)
///
/// Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?

use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::default::Default;


#[derive(Default)]
struct Position {
    x: i32,
    depth: i32,
    aim: i32,
}

impl Position {

    pub fn up(&mut self, val: i32) {
        self.aim -= val;
    }

    pub fn down(&mut self, val: i32) {
        self.aim += val;
    }

    pub fn forward(&mut self, val: i32) {
        self.x += val;
        self.depth += val * self.aim;
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

        match dir {
            Some("up") => sub.up(amt),
            Some("down") => sub.down(amt),
            Some("forward") => sub.forward(amt),
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

    sub.down(1);
    sub.down(1);
    sub.forward(1);

    assert_eq!((sub.x, sub.depth, sub.aim), (1, 2, 2));
}

#[cfg(test)]
#[test]
fn test_amt_parsing() {
    assert!(true);
}
