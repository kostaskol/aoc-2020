use crate::utils;

pub fn run(extra_star: bool) {
  let lines = utils::read_lines("inputs/test2.txt");

  let mut nums: Vec<u64> = lines.iter().filter_map(|n| n.parse().ok()).collect();
  nums.sort_unstable();

  println!("{:?}", nums);

  if extra_star {
    println!("{}", solve_extra(&nums));
  } else {
    println!("{}", solve(&nums));
  }
}

fn solve_extra(nums: &[u64]) -> u64 {
  let mut known = vec![None; nums.len()];

  solve_extra_helper(0, nums, &mut known[..])
}

/// Returns the number of ways to reach to the end of the array
/// from that point forward
fn solve_extra_helper(indx: usize, nums: &[u64], known: &mut[Option<u64>]) -> u64 {
  println!("Solving for index: {}", indx);
  
  if indx == nums.len() {
    return 3;
  }

  let mut steps = 0;

  println!("{:?}", known);

  if let Some(k) = known[indx] {
    println!("Found known index");
    return k;
  }

  for i in valid_steps(indx, nums) {
    println!("Valid step: {}", i);
    steps += solve_extra_helper(indx + i, nums, known);
    println!("Steps {}", steps);
  }
 
  known[indx] = Some(steps);

  steps
}

fn valid_steps(i: usize, nums: &[u64]) -> Vec<usize> {
  let mut ret: Vec<usize> = Vec::new();
  for j in 1..=3 {
    if i + j == nums.len() {
      break;
    }

    for k in 1..=3 {
      if nums[i + j] == nums[i] + k as u64 {
        ret.push(k);
      }
    }
  }

  ret
}

fn solve(nums: &[u64]) -> u64 {
  let mut ones = 0;
  // Count the diff between last and console adapter since it's always 3
  let mut threes = 1;

  if nums[0] == 1 {
    ones = 1;
  } else if nums[0] == 3 {
    threes += 1;
  }

  for indx in 1..nums.len() {
    let d = diff(nums[indx - 1], nums[indx]).unwrap();
    if d == 1 {
      ones += 1;
    } else if d == 3 {
      threes += 1;
    }
  }

  ones * threes
}

fn diff(num1: u64, num2: u64) -> Option<u64> {
  let diff = num2 - num1;
  if diff <= 0 || diff > 3 {
    return None;
  }

  Some(diff)
}