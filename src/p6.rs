use crate::utils;

pub fn run(extra_star: bool) {
    let lines = utils::read_lines("inputs/6.txt");

    let mut counts = 0;

    println!("{:?}", parse_groups(&lines));
    for group in parse_groups(&lines) {
        if extra_star {
            counts += group.everyone_yes();
        } else {
            counts += group.anyone_yes();
        }
    }

    println!("{}", counts);
}

#[derive(Debug)]
struct Group {
    answers: [u16; 27],
    size: usize
}

impl Group {
    fn everyone_yes(&self) -> u16 {
        let mut yeses = 0;
        for answer in &self.answers {
            if *answer as usize == self.size {
                yeses += 1;
            }
        }

        yeses
    }

    fn anyone_yes(&self) -> u16 {
        let mut yeses = 0;
        for answer in &self.answers {
            if *answer > 0 {
                yeses += 1;
            }
        }

        yeses
    }
}

fn parse_groups(lines: &[String]) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    let mut answers: [u16; 27] = [0; 27];
    let mut group_size: usize = 0;

    for line in lines {
        if line.is_empty() {
            groups.push(Group {
                answers,
                size: group_size
            });
            answers = [0; 27];
            group_size = 0;
            continue;
        }

        group_size += 1;
        for c in line.chars() {
            let indx = c as u16 - 'a' as u16;
            answers[indx as usize] += 1;
        }
    }

    groups
}