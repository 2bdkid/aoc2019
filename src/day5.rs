use std::collections::HashMap;
use std::convert::From;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[derive(Clone, Copy)]
pub enum Mode {
    Position,
    Immediate,
}

impl From<i32> for Mode {
    fn from(i: i32) -> Mode {
        match i {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!(format!("unknown mode: {}", i)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Instruction {
    One(Mode, Mode),
    Two(Mode, Mode),
    Three,
    Four(Mode),
    Five(Mode, Mode),
    Six(Mode, Mode),
    Seven(Mode, Mode),
    Eight(Mode, Mode),
    Halt,
}

impl From<i32> for Instruction {
    fn from(i: i32) -> Instruction {
        let a = (i / 10000) % 10;
        let b = (i / 1000) % 10;
        let c = (i / 100) % 10;
        let d = (i / 10) % 10;
        let e = i % 10;

        match (a, b, c, d, e) {
            (0, 0, 0, 9, 9) => Instruction::Halt,
            (_, m2, m1, 0, 1) => Instruction::One(Mode::from(m1), Mode::from(m2)),
            (_, m2, m1, 0, 2) => Instruction::Two(Mode::from(m1), Mode::from(m2)),
            (_, _, _, 0, 3) => Instruction::Three,
            (_, _, m1, 0, 4) => Instruction::Four(Mode::from(m1)),
            (_, m2, m1, 0, 5) => Instruction::Five(Mode::from(m1), Mode::from(m2)),
            (_, m2, m1, 0, 6) => Instruction::Six(Mode::from(m1), Mode::from(m2)),
            (_, m2, m1, 0, 7) => Instruction::Seven(Mode::from(m1), Mode::from(m2)),
            (_, m2, m1, 0, 8) => Instruction::Eight(Mode::from(m1), Mode::from(m2)),            
            _ => panic!(format!("unknown instruction {}", i)),
        }
    }
}

struct Intcode {
    memory: HashMap<usize, i32>,
    pc: usize,
    diagnostic: Option<i32>,
}

impl Intcode {
    fn new(memory: &Vec<i32>) -> Intcode {
        Intcode {
            memory: memory.iter().enumerate().map(|(k, v)| (k, *v)).collect(),
            pc: 0,
            diagnostic: None,
        }
    }

    fn run(&mut self, input: i32) {
        loop {
            match Instruction::from(*self.memory.get(&self.pc).unwrap()) {
                Instruction::Halt => break,
                Instruction::One(m1, m2) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());
                    let p3 = *self.memory.get(&(self.pc + 3)).unwrap();
                    self.memory.insert(p3 as usize, p1 + p2);
                    self.pc += 4;
                },
                Instruction::Two(m1, m2) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());
                    let p3 = *self.memory.get(&(self.pc + 3)).unwrap();
                    self.memory.insert(p3 as usize, p1 * p2);
                    self.pc += 4;
                },
                Instruction::Three => {
                    let p1 = *self.memory.get(&(self.pc + 1)).unwrap();
                    self.memory.insert(p1 as usize, input);
                    self.pc += 2;
                },
                Instruction::Four(m) => {
                    let p1 = self.value(m, *self.memory.get(&(self.pc + 1)).unwrap());
                    println!("{}", p1);
                    self.diagnostic = Some(p1);
                    self.pc += 2;
                },
                Instruction::Five(m1, m2) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());

                    if p1 != 0 {
                        self.pc = p2 as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                Instruction::Six(m1, m2) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());

                    if p1 == 0 {
                        self.pc = p2 as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                Instruction::Seven(m1, m2) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());
                    let p3 = *self.memory.get(&(self.pc + 3)).unwrap();

                    if p1 < p2 {
                        self.memory.insert(p3 as usize, 1);
                    } else {
                        self.memory.insert(p3 as usize, 0);
                    }

                    self.pc += 4;
                },
                Instruction::Eight(m1, m2) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());
                    let p3 = *self.memory.get(&(self.pc + 3)).unwrap();

                    if p1 == p2 {
                        self.memory.insert(p3 as usize, 1);
                    } else {
                        self.memory.insert(p3 as usize, 0);
                    }

                    self.pc += 4;
                },
            }
        }
    }

    fn value(&self, mode: Mode, value: i32) -> i32 {
        match mode {
            Mode::Position => *self.memory.get(&(value as usize)).unwrap(),
            Mode::Immediate => value,
        }
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<i32>) -> Option<i32> {
    let mut computer = Intcode::new(input);
    computer.run(1);
    computer.diagnostic
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<i32>) -> Option<i32> {
    let mut computer = Intcode::new(input);
    computer.run(5);
    computer.diagnostic
}
