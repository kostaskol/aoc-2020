use crate::utils;

const MAX_SEAT_NUM: u8 = 127;
const MAX_COL_NUM: u8 = 7;

pub fn run(extra_star: bool) {
    let lines = utils::read_lines("inputs/5.txt");

    let mut ids = parse_boarding_passes(&lines);

    ids.sort_unstable();

    if extra_star {
        println!("{}", deduce_seat(&ids).unwrap());
    } else {
        println!("{}", ids[ids.len() - 1]);
    }
}

fn deduce_seat(ids: &[u16]) -> Option<u16> {
    let mut indx1 = 0;
    let mut indx2 = 1;

    loop {
        if (ids[indx2] - ids[indx1]) == 2 {
            return Some(ids[indx2] - 1);
        }

        indx1 += 1;
        indx2 += 1;

        if indx2 == ids.len() {
            break;
        }
    }

    None
}

fn parse_boarding_passes(passes: &[String]) -> Vec<u16> {
    let mut ids: Vec<u16> = Vec::new();

    for pass in passes {
        if pass.is_empty() {
            continue;
        }

        let row = match binary_search(0, MAX_SEAT_NUM, &pass[0..7]) {
            Some(r) => r,
            None => panic!(format!("Something went wrong with FB {}", pass))
        };

        let col = match binary_search(0, MAX_COL_NUM, &pass[7..]) {
            Some(c) => c,
            None => panic!(format!("Something went wrong with LR {}", pass))
        };

        ids.push(row as u16 * 8 + col as u16);
    }

    ids
}

fn binary_search(start_low: u8, start_high: u8, inp: &str) -> Option<u8>{
    let mut low: u8 = start_low;
    let mut high: u8 = start_high;

    println!("Input: {}", inp);

    for c in inp.chars() {
        println!("Low: {} & High: {}", low, high);

        if high as i16 - low as i16 == 1 {
            println!("Got into if");
            let val = match c {
                'F' | 'L' => low,
                'B' | 'R' => high,
                _ => panic!("Something's wrong")
            };
            println!("Returning {}", val);
            return Some(val);
        }

        let new_val = (low as f32 + high as f32) / 2.0;
        println!("New val is {}", new_val);
        match c {
            'F' | 'L' => high = new_val.floor() as u8, 
            'B' | 'R' => low = new_val.ceil() as u8,
            _ => panic!("Please let me panic...")
        };
    }

    if low != high {
        return None;
    }

    Some(low)
}