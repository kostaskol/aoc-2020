use crate::utils;

pub fn run(extra: bool) {
    let lines = utils::read_lines("inputs/1.txt");
    let mut input: Vec<i32>  = lines.iter()
        .map(|i| i.parse().expect(&format!("{} is not a number. Duh!", i)))
        .collect();
    input.sort();

    if extra {
        run_two_stars(input);
    } else {
        run_one_star(input);
    }
}

fn run_one_star(input: Vec<i32>) {

    let mut i1 = 0;
    let mut i2 = input.len() - 1;

    loop {
        let v1 = input[i1];
        let v2 = input[i2];
        let sum = v1 + v2;

        if sum == 2020 {
            println!("{}", v1 * v2);
            return;
        } else if sum > 2020 {
            i2 -= 1;
        } else {
            i1 += 1;
        }
    }
}

fn run_two_stars(input: Vec<i32>) {
    for i1 in 0..input.len() {
        let mut i2 = i1 + 1;
        let mut i3 = input.len() - 1;

        loop {
            let v1 = input[i1];
            let v2 = input[i2];
            let v3 = input[i3];

            let sum = v1 + v2 + v3;

            if sum == 2020 {
                println!("{}", v1 * v2 * v3);
                return;
            } else if sum > 2020 && i3 > 0 {
                i3 -= 1;
            } else if sum < 2020 && i2 < input.len() - 1 {
                i2 += 1;
            } else {
                break;
            }
        }
    }
}
