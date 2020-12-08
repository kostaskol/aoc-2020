use crate::utils;

pub fn run(extra_star: bool) {
  let lines = utils::read_lines("inputs/8.txt");

  let mut console = parse_code(&lines);

  if extra_star {
      println!("{}", console.boot_extra());
  } else {
      println!("{}", console.boot().unwrap());
  }
}

#[derive(Debug,Clone)]
enum OpType {
  Acc(i64),
  Jump(i64),
  Noop(i64)
}

#[derive(Debug,Clone)]
struct Instruction {
  op: OpType,
  executed: bool
}

#[derive(Debug,Clone)]
struct Console {
  boot_code: Vec<Instruction>,
  acc: i64,
  pc: i64
}

impl Console {
  fn new(boot_code: Vec<Instruction>) -> Self {
    Self {
      boot_code,
      acc: 0i64,
      pc: 0i64
    }
  }

  fn boot_extra(&self) -> i64 {
      let mut pc = -1i64;
      while pc != self.boot_code.len() as i64 {
          pc += 1;
          let mut cp: Self = self.clone();
          cp.swap_instr(pc);
          match cp.boot()  {
            Some(_) => continue, // No dice :'(
            None => return cp.acc // Hooray
          }
      }

      panic!("No answer?");
  }

  fn boot(&mut self) -> Option<i64> {
    while self.pc != self.boot_code.len() as i64 {
      if self.execute() {
        // We have reached an already executed instruction
        return Some(self.acc);
      }

      self.pc += 1;
    }

    None
  }

  fn execute(&mut self) -> bool {
    // If the instruction has already been executed, return true (and not execute it)
    if self.boot_code[self.pc as usize].check_and_execute() {
      return true
    }

    match &self.boot_code[self.pc as usize].op {
      OpType::Acc(num) => self.acc += num,
      OpType::Jump(num) => {
          self.pc += num - 1;
      },
      OpType::Noop(_) => ()
    };

    false
  }

  fn swap_instr(&mut self, indx: i64) {
      match self.boot_code[indx as usize].op {
          OpType::Jump(_) => self.boot_code[indx as usize].op = OpType::Noop(0),
          OpType::Noop(num) => self.boot_code[indx as usize].op = OpType::Jump(num),
          OpType::Acc(_) => ()
      }
  }
}

impl Instruction {
  fn new(op: OpType) -> Self {
    Self {
      op,
      executed: false
    }
  }

  fn check_and_execute(&mut self) -> bool {
      let old = self.executed;
      self.executed = true;
      old
  }
}

fn parse_code(lines: &[String]) -> Console {
  let mut instructions: Vec<Instruction> = Vec::new();

  for line in lines {
    if line.is_empty() {
      continue;
    }

    let parts: Vec<&str> = line.split(' ').collect();
    let (op, num) = (parts[0], parts[1].parse::<i64>().unwrap());
    let op_type = match op {
      "acc" => OpType::Acc(num),
      "jmp" => OpType::Jump(num),
      "nop" => OpType::Noop(num),
      &_ => panic!("invalid op {}", op)
    };

    instructions.push(Instruction::new(op_type));
  }

  Console::new(instructions)
}
