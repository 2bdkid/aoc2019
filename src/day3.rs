use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Path {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, n) = s.split_at(1);
        if let Ok(n) = n.parse() {
            match d {
                "U" => Ok(Path::Up(n)),
                "D" => Ok(Path::Down(n)),
                "L" => Ok(Path::Left(n)),
                "R" => Ok(Path::Right(n)),
                _ => Err(format!("Path parse error - direction \"{}\"", d)),
            }
        } else {
            Err(String::from("Path parse error - invalid size"))
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<Path>, Vec<Path>) {
    let mut g: Vec<Vec<Path>> = input
        .lines()
        .map(|l| {
            l.trim()
                .split(',')
                .map(|p| Path::from_str(p).unwrap())
                .collect()
        })
        .collect();
    (g.remove(0), g.remove(0))
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

fn intersection(a: &Vec<Point>, b: &Vec<Point>) -> Vec<Point> {
    a.clone().into_iter().filter(|x| b.contains(x)).collect()
}

fn path_points(path: &Vec<Path>) -> Vec<Point> {
    let mut curr = Point { x: 0, y: 0 };
    let mut points = vec![Point { x: 0, y: 0 }];

    for p in path {
        match p {
            Path::Up(n) => {
                for _ in 0..*n {
                    curr.y += 1;
                    points.push(Point {
                        x: curr.x,
                        y: curr.y,
                    });
                }
            }
            Path::Down(n) => {
                for _ in 0..*n {
                    curr.y -= 1;
                    points.push(Point {
                        x: curr.x,
                        y: curr.y,
                    });
                }
            }
            Path::Left(n) => {
                for _ in 0..*n {
                    curr.x -= 1;
                    points.push(Point {
                        x: curr.x,
                        y: curr.y,
                    });
                }
            }
            Path::Right(n) => {
                for _ in 0..*n {
                    curr.x += 1;
                    points.push(Point {
                        x: curr.x,
                        y: curr.y,
                    });
                }
            }
        }
    }

    points
}

#[aoc(day3, part1)]
pub fn solve_day1(input: &(Vec<Path>, Vec<Path>)) -> i32 {
    let (path1, path2) = input;
    let (path1, path2) = (path_points(&path1), path_points(&path2));
    let intersection = intersection(&path1, &path2);

    intersection
        .into_iter()
        .filter(|p| *p != Point { x: 0, y: 0 })
        .map(|Point { x, y }| x + y)
        .min()
        .unwrap()
}

fn steps_to(pt: Point, path: &Vec<Point>) -> i32 {
    path.iter().position(|p| *p == pt).unwrap() as i32
}

#[aoc(day3, part2)]
pub fn solve_day2(input: &(Vec<Path>, Vec<Path>)) -> i32 {
    let (path1, path2) = input;
    let (path1, path2) = (path_points(&path1), path_points(&path2));
    let intersections = intersection(&path1, &path2);

    intersections
        .iter()
        .filter(|p| **p != Point { x: 0, y: 0 })
        .map(|i| steps_to(*i, &path1) + steps_to(*i, &path2))
        .min()
        .unwrap()
}
