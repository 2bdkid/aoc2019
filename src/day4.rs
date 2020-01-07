#[aoc_generator(day4)]
pub fn input_generator(_: &str) -> (i32, i32) {
    (359282, 820401)
}

#[aoc(day4, part1)]
pub fn solve_part1(bounds: &(i32, i32)) -> i32 {
    let (min, max) = bounds;

    let adjacent_digits = |s: &str| {
        let a = s.chars();
        let mut b = s.chars();
        b.next();

        for (c, d) in a.zip(b) {
            if c == d {
                return true;
            }
        }

        false
    };

    (*min..*max)
        .map(|i| i.to_string())
        .filter(|s| s.chars().is_sorted())
        .filter(|s| adjacent_digits(s))
        .count() as i32
}

#[aoc(day4, part2)]
pub fn solve_part2(bounds: &(i32, i32)) -> i32 {
    let (min, max) = bounds;

    let adjacent_digits = |s: &str| {
        let mut iter = s.chars();

        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();
        let d = iter.next().unwrap();
        let e = iter.next().unwrap();
        let f = iter.next().unwrap();

        if (a == b) && (b != c) {
            return true;
        }
        if (b == c) && (a != b) && (c != d) {
            return true;
        }
        if (c == d) && (b != c) && (d != e) {
            return true;
        }

        if (d == e) && (c != d) && (e != f) {
            return true;
        }

        if (e == f) && (d != e) {
            return true;
        }

        false
    };

    (*min..*max)
        .map(|i| i.to_string())
        .filter(|s| s.chars().is_sorted())
        .filter(|s| adjacent_digits(s))
        .count() as i32
}
