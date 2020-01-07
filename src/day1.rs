pub type Mass = i32;
pub type Fuel = i32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Mass> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Mass]) -> Fuel {
    input.iter().map(|m| (m / 3) - 2).sum()
}

fn total_module_fuel(m: Mass) -> Fuel {
    let f: Fuel = (m / 3) - 2;
    if f < 0 {
        0
    } else {
        f + total_module_fuel(f)
    }
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Mass]) -> Fuel {
    input.iter().map(|&m| total_module_fuel(m)).sum()
}
