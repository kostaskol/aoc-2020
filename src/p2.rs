use crate::utils;

pub fn run(extra: bool) {
    let lines = utils::read_lines("inputs/2.txt");

    let policies = parse_lines(lines);

    let mut valid_passwds = 0;
    
    for policy in policies {
        if extra {
            if policy.is_actually_valid() {
                valid_passwds += 1;
            }
        } else {
            if policy.is_valid() {
                valid_passwds += 1;
            }
        }
    }

    println!("{}", valid_passwds);
}

fn parse_lines(lines: Vec<String>) -> Vec<Policy> {
    let mut policies: Vec<Policy> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        let (policy, passwd) = (parts[0], parts[1]);

        policies.push(Policy::new(policy, passwd.to_string()));
    }

    policies
}

#[derive(Debug)]
struct Policy {
    min: u8,
    max: u8,
    character: char,
    passwd: String
}

impl Policy {
    fn new(unparsed: &str, passwd: String) -> Policy {
        let parts: Vec<&str> = unparsed.split(" ").collect();
        let (range, character) = (parts[0], parts[1].chars().next().unwrap());

        let parts: Vec<&str> = range.split("-").collect();
        let (min, max) = (
            parts[0].parse::<u8>().unwrap(),
            parts[1].parse::<u8>().unwrap()
        );

        Policy { min, max, character, passwd }
    }

    fn is_valid(&self) -> bool {
        let counts = self.char_occurences();

        counts >= self.min && counts <= self.max
    }

    fn is_actually_valid(&self) -> bool {
        let mut counts = 0;

        for pos in vec![self.min, self.max] {
            if self.passwd.chars().nth((pos - 1) as usize).unwrap() == self.character {
                counts += 1;
            }
        }

        counts == 1
    }

    fn char_occurences(&self) -> u8 {
        let mut count = 0;
        for c in self.passwd.chars() {
            if c == self.character {
                count += 1;
            }
        }

        count
    }
}
