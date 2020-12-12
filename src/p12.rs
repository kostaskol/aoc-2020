use regex::Regex;
use std::str::FromStr;
use crate::utils;

pub fn run(extra_star: bool) {
  let lines = utils::read_lines("inputs/test.txt");

  let ship = Ship::new(&lines);
  ship.navigate();
  println!("{:?}", ship);
}

#[derive(Debug)]
struct Ship {
  directions: Vec<Dir>
}

impl Ship {
  fn new(lines: &[String]) -> Self {
    let mut directions: Vec<Dir> = Vec::with_capacity(lines.len());

    for line in lines {
      directions.push(line.parse().unwrap());
    }

    Self { directions }
  }

  fn navigate(&self) {
    for dir in &self.directions {
      match dir {
        Dir::Forward => {
          
        }
      }
    }
  }
}

#[derive(Debug)]
enum Dir {
  North(i32),
  East(i32),
  South(i32),
  West(i32),
  Left(i32),
  Right(i32),
  Forward(i32)
}

impl FromStr for Dir {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let re = Regex::new(r"^([NWSERLF])(\d+)$").unwrap();

    match re.captures(input) {
      Some(x) => {
        let letter = x.get(1).unwrap().as_str();
        let num = x.get(2).unwrap().as_str().parse().unwrap();

        let dir = match letter {
          "N" => Dir::North(num),
          "E" => Dir::East(num),
          "S" => Dir::South(num),
          "W" => Dir::West(num),
          "R" => Dir::Right(num),
          "L" => Dir::Left(num),
          "F" => Dir::Forward(num),
          &_ => panic!("Don't know how to format {}", input)
        };

        return Ok(dir)
      },
      None => {
        panic!("Don't know how to format {}", input);
      }
    }
  }
}