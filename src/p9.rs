use crate::utils;

const WINDOW_SIZE: usize = 25;

pub fn run(extra_star: bool) {
  let lines = utils::read_lines("inputs/9.txt");

  let mut xmas_code = parse_xmas_code(&lines);

  if extra_star {
    println!("{}", xmas_code.decode_extra().unwrap());
  } else {
    println!("{}", xmas_code.decode().unwrap());
  }
}

struct XmasCode {
  input: Vec<u64>,
  p: usize
}

impl XmasCode {
  fn new(input: Vec<u64>) -> Self {
    Self {
      input,
      p: WINDOW_SIZE
    }
  }

  fn decode(&mut self) -> Option<u64> {
    loop {
      let window_start = self.p - WINDOW_SIZE;
      let mut window: Vec<u64> = self.input[window_start..self.p].to_vec();
      let target = self.input[self.p];

      window.sort_unstable();
      if !self.is_valid_num(target, &window) {
        return Some(target);
      }

      self.p += 1;
    }
  }

  fn decode_extra(&mut self) -> Option<u64> {
    let invalid = match self.decode() {
      Some(i) => i,
      None => return None
    };

    let mut start = 0;
    while start < self.input.len() - 1 {
      let mut sum = 0;
      let mut end = start;
      let mut max = 0;
      let mut min = u64::MAX;

      while sum < invalid {
        let val = self.input[end];
        if val > max {
          max = val;
        } 
        if val < min {
          min = val;
        }

        sum += val;

        if sum == invalid {
          return Some(min + max);
        }

        end += 1;
      }

      start += 1;
    }

    None
  }

  fn is_valid_num(&self, target: u64, window: &[u64]) -> bool {
    let mut low = 0;
    let mut high = WINDOW_SIZE - 1;

    while low != high {
      let sum = window[low] + window[high];
      if sum == target {
        return true;
      }
      if sum < target {
        low += 1;
      } else {
        high -= 1;
      }
    }

    false
  }
}

fn parse_xmas_code(lines: &[String]) -> XmasCode {
  let ilines: Vec<u64> = lines.iter().map(|e| e.parse().unwrap()).collect();

  XmasCode::new(ilines)
}