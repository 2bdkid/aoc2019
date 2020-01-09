use std::collections::HashMap;
use std::convert::TryFrom;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[derive(Clone, Copy)]
pub enum Mode {
    Position,
    Immediate,
}

impl TryFrom<i32> for Mode {
    type Error = String;

    fn try_from(i: i32) -> Result<Mode, String> {
        match i {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err(format!("unknown mode: {}", i)),
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

impl TryFrom<i32> for Instruction {
    type Error = String;

    fn try_from(i: i32) -> Result<Instruction, String> {
        let a = (i / 10000) % 10;
        let b = (i / 1000) % 10;
        let c = (i / 100) % 10;
        let d = (i / 10) % 10;
        let e = i % 10;

        match (a, b, c, d, e) {
            (0, 0, 0, 9, 9) => Ok(Instruction::Halt),
            (_, m2, m1, 0, 1) => Ok(Instruction::One(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
            )),
            (_, m2, m1, 0, 2) => Ok(Instruction::Two(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
            )),
            (_, _, _, 0, 3) => Ok(Instruction::Three),
            (_, _, m1, 0, 4) => Ok(Instruction::Four(Mode::try_from(m1).unwrap())),
            (_, m2, m1, 0, 5) => Ok(Instruction::Five(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
            )),
            (_, m2, m1, 0, 6) => Ok(Instruction::Six(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
            )),
            (_, m2, m1, 0, 7) => Ok(Instruction::Seven(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
            )),
            (_, m2, m1, 0, 8) => Ok(Instruction::Eight(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
            )),
            _ => Err(format!("unknown instruction {}", i)),
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
            match Instruction::try_from(*self.memory.get(&self.pc).unwrap()).unwrap() {
                Instruction::Halt => break,
                Instruction::One(m1, m2) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());
                    let p3 = *self.memory.get(&(self.pc + 3)).unwrap();
                    self.memory.insert(p3 as usize, p1 + p2);
                    self.pc += 4;
                }
                Instruction::Two(m1, m2) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());
                    let p3 = *self.memory.get(&(self.pc + 3)).unwrap();
                    self.memory.insert(p3 as usize, p1 * p2);
                    self.pc += 4;
                }
                Instruction::Three => {
                    let p1 = *self.memory.get(&(self.pc + 1)).unwrap();
                    self.memory.insert(p1 as usize, input);
                    self.pc += 2;
                }
                Instruction::Four(m) => {
                    let p1 = self.value(m, *self.memory.get(&(self.pc + 1)).unwrap());
                    println!("{}", p1);
                    self.diagnostic = Some(p1);
                    self.pc += 2;
                }
                Instruction::Five(m1, m2) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());

                    if p1 != 0 {
                        self.pc = p2 as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Instruction::Six(m1, m2) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());

                    if p1 == 0 {
                        self.pc = p2 as usize;
                    } else {
                        self.pc += 3;
                    }
                }
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
                }
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
                }
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
