use std::collections::{HashSet, HashMap};
use std::str::FromStr;
use regex::Regex;

use crate::utils;

pub fn run(extra_star: bool) {
  let lines = utils::read_lines("inputs/7.txt");
  let bags = parse_bags(&lines);

  let total = match extra_star {
    true => count_shiny_gold(&bags),
    false => find_shiny_gold(&bags)
  };

  println!("{}", total);
}

fn count_shiny_gold(bags: &HashMap<String, Bag>) -> u16 {
  let mut known: HashMap<String, u16> = HashMap::new();
  count_shiny_helper("shiny gold bag", bags, &mut known) - 1
}

fn count_shiny_helper(target: &str, bags: &HashMap<String, Bag>, known: &mut HashMap<String, u16>) -> u16 {
  let curr_target = bags.get(target).unwrap();

  match &curr_target.contents {
    Some(contents) => {
      let mut overall = 0u16;

      for content in contents {
        if let Some(cap) = known.get(&content.name) {
          overall += content.num as u16 * cap;
        } else {
          let h = count_shiny_helper(&content.name, bags, known);
          known.insert(content.name.clone(), h);
          overall += content.num as u16 * h
        }
      }
      overall + 1
    },
    None => 1
  }
}

fn find_shiny_gold(bags: &HashMap<String, Bag>) -> u16{
  let mut targets = vec!["shiny gold bag"];
  let mut total = 0u16;

  let mut visited: HashSet<String> = HashSet::new();

  while let Some(curr_target) = targets.pop() {
    for (color, bag) in bags {
      if bag.contains(&curr_target).is_some() && !visited.contains(color) {
        visited.insert(color.clone());
        targets.push(&color);
        total += 1;
      }
    }
  }

  total
}

fn parse_bags(lines: &[String]) -> HashMap<String, Bag> {
  let mut bags: HashMap<String, Bag> = HashMap::new();

  for line in lines {
    let init_parts: Vec<&str> = line.split(" contain ").collect();

    let bag_name = init_parts[0].replace("bags", "bag");
    let contents = parse_contents(init_parts[1]);

    bags.insert(bag_name.to_string(), Bag::new(contents));
  }

  bags
}

fn parse_contents(content_parts: &str) -> Option<Vec<Content>> {
  if content_parts == "no other bags." {
    return None;
  }

  let mut contents: Vec<Content> = Vec::new();
  let replaced = content_parts.replace('.', "").replace("bags", "bag");
  let content_parts: Vec<&str> = replaced.split(", ").collect();

  for part in content_parts {
    contents.push(Content::from_str(part).unwrap());
  }

  Some(contents)
}

#[derive(Debug)]
struct Bag {
  contents: Option<Vec<Content>>
}

#[derive(Debug)]
struct Content {
  num: u8,
  name: String
}

impl Bag {
  fn new(contents: Option<Vec<Content>>) -> Self {
    Self {
      contents
    }
  }

  fn contains(&self, cont: &str) -> Option<u8> {
    if let Some(contents) = &self.contents {
      for bag in contents {
        if bag.name == cont {
          return Some(bag.num);
        }
      }
    }

    None
  }
}

impl FromStr for Content {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let re = Regex::new(r"^(\d+)\s+(.*)$").unwrap();

    match re.captures(s) {
      Some(x) => {
        let num = x.get(1).unwrap().as_str().parse().unwrap();
        let name = x.get(2).unwrap().as_str();
        return Ok(Self { num, name: String::from(name) })
      },
      None => Err(format!("could not match regex for {}", s))
    }
  }
}
