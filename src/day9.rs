use std::collections::HashMap;
use std::collections::VecDeque;

use std::convert::TryFrom;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[derive(Clone, Copy)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for Mode {
    type Error = String;

    fn try_from(i: i64) -> Result<Mode, String> {
        match i {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(format!("unknown mode: {}", i)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Instruction {
    One(Mode, Mode, Mode),
    Two(Mode, Mode, Mode),
    Three(Mode),
    Four(Mode),
    Five(Mode, Mode),
    Six(Mode, Mode),
    Seven(Mode, Mode, Mode),
    Eight(Mode, Mode, Mode),
    Nine(Mode),
    Halt,
}

impl TryFrom<i64> for Instruction {
    type Error = String;

    fn try_from(i: i64) -> Result<Instruction, String> {
        let a = (i / 10000) % 10;
        let b = (i / 1000) % 10;
        let c = (i / 100) % 10;
        let d = (i / 10) % 10;
        let e = i % 10;

        match (a, b, c, d, e) {
            (0, 0, 0, 9, 9) => Ok(Instruction::Halt),
            (m3, m2, m1, 0, 1) => Ok(Instruction::One(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
                Mode::try_from(m3).unwrap(),
            )),
            (m3, m2, m1, 0, 2) => Ok(Instruction::Two(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
                Mode::try_from(m3).unwrap(),                
            )),
            (_, _, m, 0, 3) => Ok(Instruction::Three(
                Mode::try_from(m).unwrap(),
            )),
            (_, _, m1, 0, 4) => Ok(Instruction::Four(Mode::try_from(m1).unwrap())),
            (_, m2, m1, 0, 5) => Ok(Instruction::Five(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
            )),
            (_, m2, m1, 0, 6) => Ok(Instruction::Six(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
            )),
            (m3, m2, m1, 0, 7) => Ok(Instruction::Seven(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
                Mode::try_from(m3).unwrap(),
            )),
            (m3, m2, m1, 0, 8) => Ok(Instruction::Eight(
                Mode::try_from(m1).unwrap(),
                Mode::try_from(m2).unwrap(),
                Mode::try_from(m3).unwrap(),
            )),
            (_, _, m, 0, 9) => Ok(Instruction::Nine(Mode::try_from(m).unwrap())),
            _ => Err(format!("unknown instruction {}", i)),
        }
    }
}

struct Intcode {
    memory: HashMap<usize, i64>,
    pc: usize,
    relative_base: i64,
    halted: bool,
}

impl Intcode {
    fn new(memory: &[i64]) -> Intcode {
        Intcode {
            memory: memory.iter().enumerate().map(|(k, v)| (k, *v)).collect(),
            pc: 0,
            relative_base: 0,
            halted: false,
        }
    }

    fn run(&mut self, bus: &mut VecDeque<i64>) {
        loop {
            match Instruction::try_from(*self.memory.get(&self.pc).unwrap()).unwrap() {
                Instruction::Halt => {
                    self.halted = true;
                    break;
                }
                Instruction::One(m1, m2, m3) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());
                    let p3 = self.address(m3, *self.memory.get(&(self.pc + 3)).unwrap());
                    self.memory.insert(p3 as usize, p1 + p2);
                    self.pc += 4;
                }
                Instruction::Two(m1, m2, m3) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());
                    let p3 = self.address(m3, *self.memory.get(&(self.pc + 3)).unwrap());
                    self.memory.insert(p3 as usize, p1 * p2);
                    self.pc += 4;
                }
                Instruction::Three(m) => {
                    let p1 = *self.memory.get(&(self.pc + 1)).unwrap();
                    self.memory.insert(self.address(m, p1) as usize, bus.pop_front().unwrap());
                    self.pc += 2;
                }
                Instruction::Four(m) => {
                    let p1 = self.value(m, *self.memory.get(&(self.pc + 1)).unwrap());
                    bus.push_back(p1);
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
                Instruction::Seven(m1, m2, m3) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());
                    let p3 = self.address(m3, *self.memory.get(&(self.pc + 3)).unwrap());

                    if p1 < p2 {
                        self.memory.insert(p3 as usize, 1);
                    } else {
                        self.memory.insert(p3 as usize, 0);
                    }

                    self.pc += 4;
                }
                Instruction::Eight(m1, m2, m3) => {
                    let p1 = self.value(m1, *self.memory.get(&(self.pc + 1)).unwrap());
                    let p2 = self.value(m2, *self.memory.get(&(self.pc + 2)).unwrap());
                    let p3 = self.address(m3, *self.memory.get(&(self.pc + 3)).unwrap());

                    if p1 == p2 {
                        self.memory.insert(p3 as usize, 1);
                    } else {
                        self.memory.insert(p3 as usize, 0);
                    }

                    self.pc += 4;
                }
                Instruction::Nine(m) => {
                    self.relative_base += self.value(m, *self.memory.get(&(self.pc + 1)).unwrap());
                    self.pc += 2;
                }
            }
        }
    }

    fn value(&self, mode: Mode, value: i64) -> i64 {
        match mode {
            Mode::Position => *self.memory.get(&(value as usize)).unwrap_or(&0),
            Mode::Immediate => value,
            Mode::Relative => *self
                .memory
                .get(&((value + self.relative_base) as usize))
                .unwrap_or(&0),
        }
    }

    fn address(&self, mode: Mode, addr: i64) -> i64 {
        match mode {
            Mode::Relative => addr + self.relative_base,
            _ => addr,
        }
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let mut computer = Intcode::new(input);
    let mut bus = VecDeque::new();
    bus.push_back(1);
    computer.run(&mut bus);
    bus.pop_front().unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[i64]) -> i64 {
    let mut computer = Intcode::new(input);
    let mut bus = VecDeque::new();
    bus.push_back(2);
    computer.run(&mut bus);
    bus.pop_front().unwrap()
}
