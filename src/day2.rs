#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.split(',').map(|i| i.parse().unwrap()).collect()
}

pub fn computer(input_s: &[u32], a: u32, b: u32) -> u32 {
    let mut input = Vec::new();
    let mut iter = input_s.iter();
    
    input.resize_with(input_s.len(), || *iter.next().unwrap());
    
    input[1] = a;
    input[2] = b;

    let mut pos = 0usize;
    
    loop {
        let op = input[pos];
        match op {
            1 => {
                let a = input[input[pos + 1] as usize];
                let b = input[input[pos + 2] as usize];
                let c = input[pos + 3];
                input[c as usize] = a + b;
            },
            2 => {
                let a = input[input[pos + 1] as usize];
                let b = input[input[pos + 2] as usize];
                let c = input[pos + 3];
                input[c as usize] = a * b;
            },
            99 => break,
            _ => panic!("Unknown opcode")
        }

        pos += 4;
    }

    input[0]

}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    computer(input, 12, 2)
}

const DESIRED_OUTPUT: u32 = 19690720;

#[aoc(day2, part2)]
pub fn solve_part2(input: &[u32]) -> Option<u32> {
    for noun in 0..100 {
        for verb in 0..100 {
            if computer(input, noun, verb) == DESIRED_OUTPUT {
                return Some(100 * noun + verb)
            }
        }
    }
    
    None
}

