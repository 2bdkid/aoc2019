use std::collections::HashMap;
use std::collections::VecDeque;

use std::convert::TryFrom;

#[aoc_generator(day7)]
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
    halted: bool,
}

impl Intcode {
    fn new(memory: &Vec<i32>) -> Intcode {
        Intcode {
            memory: memory.iter().enumerate().map(|(k, v)| (k, *v)).collect(),
            pc: 0,
            halted: false,
        }
    }

    fn run(&mut self, bus: &mut VecDeque<i32>) {
        loop {
            match Instruction::try_from(*self.memory.get(&self.pc).unwrap()).unwrap() {
                Instruction::Halt => {
                    self.halted = true;
                    break;
                }
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
                    self.memory.insert(p1 as usize, bus.pop_front().unwrap());
                    self.pc += 2;
                }
                Instruction::Four(m) => {
                    let p1 = self.value(m, *self.memory.get(&(self.pc + 1)).unwrap());
                    bus.push_back(p1);
                    self.pc += 2;
                    break; // hack: yield
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

fn thrust_level(program: &Vec<i32>, phases: (i32, i32, i32, i32, i32)) -> i32 {
    let (a, b, c, d, e) = phases;
    let mut bus = VecDeque::new();
    let mut amp_a = Intcode::new(program);
    let mut amp_b = Intcode::new(program);
    let mut amp_c = Intcode::new(program);
    let mut amp_d = Intcode::new(program);
    let mut amp_e = Intcode::new(program);

    bus.push_back(a);
    bus.push_back(0);
    amp_a.run(&mut bus);

    bus.push_front(b);
    amp_b.run(&mut bus);

    bus.push_front(c);
    amp_c.run(&mut bus);

    bus.push_front(d);
    amp_d.run(&mut bus);

    bus.push_front(e);
    amp_e.run(&mut bus);

    bus.pop_front().unwrap()
}

fn feedback_loop(program: &Vec<i32>, phases: (i32, i32, i32, i32, i32)) -> i32 {
    let (a, b, c, d, e) = phases;
    let mut bus = VecDeque::new();
    let mut amp_a = Intcode::new(program);
    let mut amp_b = Intcode::new(program);
    let mut amp_c = Intcode::new(program);
    let mut amp_d = Intcode::new(program);
    let mut amp_e = Intcode::new(program);

    bus.push_front(a);
    bus.push_back(0);
    amp_a.run(&mut bus);

    bus.push_front(b);
    amp_b.run(&mut bus);

    bus.push_front(c);
    amp_c.run(&mut bus);

    bus.push_front(d);
    amp_d.run(&mut bus);

    bus.push_front(e);
    amp_e.run(&mut bus);

    while !amp_a.halted && !amp_b.halted && !amp_c.halted && !amp_d.halted && !amp_e.halted {
        amp_a.run(&mut bus);
        amp_b.run(&mut bus);
        amp_c.run(&mut bus);
        amp_d.run(&mut bus);
        amp_e.run(&mut bus);
    }

    bus.pop_front().unwrap()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<i32>) -> Option<i32> {
    let phases = (0..=44444)
        .map(|i| {
            (
                ((i / 10000) % 5),
                ((i / 1000) % 5),
                ((i / 100) % 5),
                ((i / 10) % 5),
                (i % 5),
            )
        })
        .filter(|(a, b, c, d, e)| {
            (a != b)
                && (a != c)
                && (a != d)
                && (a != e)
                && (b != c)
                && (b != d)
                && (b != e)
                && (c != d)
                && (c != e)
                && (d != e)
        });

    phases.map(|phase| thrust_level(input, phase)).max()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<i32>) -> Option<i32> {
    let phases = (55555..=99999)
        .map(|i| {
            (
                ((i / 10000) % 5) + 5,
                ((i / 1000) % 5) + 5,
                ((i / 100) % 5) + 5,
                ((i / 10) % 5) + 5,
                (i % 5) + 5,
            )
        })
        .filter(|(a, b, c, d, e)| {
            (a != b)
                && (a != c)
                && (a != d)
                && (a != e)
                && (b != c)
                && (b != d)
                && (b != e)
                && (c != d)
                && (c != e)
                && (d != e)
        });

    phases.map(|phase| feedback_loop(input, phase)).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_a() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), Some(43210));
    }

    #[test]
    fn part1_b() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), Some(54321));
    }

    #[test]
    fn part1_c() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let input = input_generator(input);
        assert_eq!(solve_part1(&input), Some(65210));
    }

    #[test]
    fn thrust_level_calculation_a() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let input = input_generator(input);
        assert_eq!(thrust_level(&input, (4, 3, 2, 1, 0)), 43210);
    }

    #[test]
    fn thrust_level_calculation_b() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let input = input_generator(input);
        assert_eq!(thrust_level(&input, (0, 1, 2, 3, 4)), 54321);
    }

    #[test]
    fn thrust_level_calculation_c() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let input = input_generator(input);
        assert_eq!(thrust_level(&input, (1, 0, 4, 3, 2)), 65210);
    }

    #[test]
    fn feedback_loop_a() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let input = input_generator(input);
        assert_eq!(feedback_loop(&input, (9, 8, 7, 6, 5)), 139629729);
    }

    #[test]
    fn feedback_loop_b() {
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let input = input_generator(input);
        assert_eq!(feedback_loop(&input, (9, 7, 8, 5, 6)), 18216);
    }
}
