use std::collections::HashSet;
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

fn count_shiny_gold(bags: &HashSet<Bag>) -> u16 {
  // let mut targets = vec!["shiny gold bag"];
  // let mut total = 0u16;
  // let mut visited: HashSet<String> = HashSet::new();

  // // Hacky and awful but actually gives us the bag we want
  // // Please fix :(
  // let mut targets = vec![bags.get(&Bag { color: "shiny gold bag".to_string(), contents: None})];

  // println!("{:?}", bags);
  // println!("{:?}", targets);

  count_shiny_helper("shiny gold bag", bags) - 1


  // while let Some(curr_target) = targets.pop() {
  //   let curr_target = curr_target.unwrap();
  //   let contents = curr_target.contents;
  //   if contents.is_none() {
  //     continue;
  //   }

  //   total += 
  // }
  // total
}

fn count_shiny_helper(target: &str, bags: &HashSet<Bag>) -> u16 {
  let target_bag = bags.get(&Bag { color: String::from(target), contents: None });

  println!("Target: {:?}", target_bag);
  match &target_bag.unwrap().contents {
    Some(contents) => {
      let mut overall = 0u16;

      for bag in contents {
        let h = count_shiny_helper(&bag.name, bags);
        println!("{} * {} ({})", bag.num, bag.name, h);
        overall += bag.num as u16 * h
      }

      println!("{} returning {}", target, overall);
      overall + 1
    },
    None => {
      println!("{} returning 1", target);
      1
    }
  }
}

fn find_shiny_gold(bags: &HashSet<Bag>) -> u16{
  let mut targets = vec!["shiny gold bag"];
  let mut total = 0u16;

  let mut visited: HashSet<String> = HashSet::new();

  while let Some(curr_target) = targets.pop() {
    println!("Current target: {}", curr_target);

    for bag in bags {
      println!("Bag: {:?}", bag);

      if bag.contains(&curr_target).is_some() && !visited.contains(&bag.color) {
        println!("It's a match!!");
        visited.insert(bag.color.clone());
        targets.push(&bag.color);
        total += 1;
      }
    }
  }

  total
}

#[derive(Debug)]
struct Bag {
  color: String,
  contents: Option<Vec<Content>>
}

#[derive(Debug)]
struct Content {
  num: u8,
  name: String
}


impl PartialEq for Bag {
  fn eq(&self, other: &Self) -> bool {
    self.color == other.color
  }
}

impl std::hash::Hash for Bag {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.color.hash(state);
  }
}

impl Eq for Bag {}


impl Bag {
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

fn parse_bags(lines: &[String]) -> HashSet<Bag> {
  let mut bags: HashSet<Bag> = HashSet::new();

  for line in lines {
    let init_parts: Vec<&str> = line.split(" contain ").collect();

    let bag_name = init_parts[0].replace("bags", "bag");
    let contents = parse_contents(init_parts[1]);

    bags.insert(Bag { color: bag_name.to_string(), contents });
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

  let re = Regex::new(r"^(\d+)\s+(.*)$").unwrap();

  for part in content_parts {
    match re.captures(part) {
      Some(x) => {
        let num = x.get(1).unwrap().as_str().parse().unwrap();
        let name = x.get(2).unwrap().as_str();
        contents.push(Content { num, name: String::from(name) });
      },
      None => panic!("could not match regex for {}", part)
    }
  }

  Some(contents)
}