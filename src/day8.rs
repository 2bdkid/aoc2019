use std::char;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[u32]) -> i32 {
    let layer_with_least_zeroes = input
        .chunks(25 * 6)
        .min_by_key(|layer| layer.iter().filter(|px| **px == 0).count())
        .unwrap();

    let (ones, twos) = layer_with_least_zeroes
        .iter()
        .fold((0, 0), |(x, y), px| match px {
            1 => (x + 1, y),
            2 => (x, y + 1),
            _ => (x, y),
        });

    ones * twos
}

fn color<'a, I>(pixel: I) -> char
where
    I: Iterator<Item = &'a u32>,
{
    pixel
        .skip_while(|l| **l == 2)
        .next()
        .map(|c| char::from_digit(*c, 10).unwrap())
        .unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[u32]) -> String {
    let pixel = |n| input.iter().skip(n).step_by(25 * 6);
    (0..(25 * 6)).map(|px| color(pixel(px))).collect()
}
