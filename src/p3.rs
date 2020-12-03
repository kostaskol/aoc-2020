use crate::utils;

pub fn run(extra_star: bool) {
    let lines = utils::read_lines("inputs/3.txt");

    let b = Biome::new(&lines);

    let steps;

    if extra_star {
        steps = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    } else {
        steps = vec![(1, 3)];
    }

    let mut prod = 1;
    for (step_i, step_j) in steps {
        let mut i = 0;
        let mut j = 0;
        let mut total_trees = 0;

        loop {
            match b.is_tree(i, j) {
                Some(is) => {
                    if is {
                        total_trees += 1;
                    }
                },
                None => {
                    prod *= total_trees;
                    break;
                }
            }

            i += step_i;
            j += step_j;
        }
    }

    println!("{}", prod);


}

struct Biome {
    board: Vec<Vec<bool>>
}

impl Biome {
    fn new(lines: &[String]) -> Biome {
        let mut board: Vec<Vec<bool>> = Vec::new();

        for line in lines {
            if line.is_empty() {
                break;
            }

            let mut row: Vec<bool> = Vec::new();
            for c in line.chars() {
                if c == '#' {
                    row.push(true);
                } else {
                    row.push(false);
                }
            }
            board.push(row);
        }

        Biome { board }
    }

    fn is_tree(&self, i: usize, j: usize) -> Option<bool> {
        if i >= self.board.len() {
            // It means we are outside the last row so we should quit
            return None;
        }

        let real_j = j % self.board[0].len();

        Some(self.board[i][real_j])
    }
}
