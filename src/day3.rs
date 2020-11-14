use std::{collections::HashSet, hash::Hash};

pub fn day3(input: &[String]) -> (i64, i64) {
    (part1(input), 0)
}

fn part1(input: &[String]) -> i64 {
    let paths = input
        .iter()
        .map(|i| {
            println!("{}", i);
            let instructions = i.split(",").collect::<Vec<_>>();
            println!("{:?}", instructions);

            let routes = instructions.iter().map(parse_route).collect::<Vec<Route>>();
            println!("{:?}", routes);

            let mut current_point = Point {
                x: 0,
                y: 0,
                steps: 0,
            };

            let points = routes
                .iter()
                .flat_map(|r| match r.direction {
                    Direction::Right => {
                        let mut list = vec![];
                        for _ in 0..r.steps {
                            let p = Point {
                                x: current_point.x + 1,
                                y: current_point.y,
                                steps: current_point.steps + 1,
                            };
                            list.push(p);
                            current_point = p;
                        }
                        list
                    }
                    Direction::Left => {
                        let mut list = vec![];
                        for _ in 0..r.steps {
                            let p = Point {
                                x: current_point.x - 1,
                                y: current_point.y,
                                steps: current_point.steps + 1,
                            };
                            list.push(p);
                            current_point = p;
                        }
                        list
                    }
                    Direction::Up => {
                        let mut list = vec![];
                        for _ in 0..r.steps {
                            let p = Point {
                                x: current_point.x,
                                y: current_point.y + 1,
                                steps: current_point.steps + 1,
                            };
                            list.push(p);
                            current_point = p;
                        }
                        list
                    }
                    Direction::Down => {
                        let mut list = vec![];
                        for _ in 0..r.steps {
                            let p = Point {
                                x: current_point.x,
                                y: current_point.y - 1,
                                steps: current_point.steps + 1,
                            };
                            list.push(p);
                            current_point = p;
                        }
                        list
                    }
                })
                .collect::<HashSet<Point>>();

            points
        })
        .collect::<Vec<HashSet<Point>>>();

    let intersection = paths[0].intersection(&paths[1]);
    intersection.map(|p| p.x.abs() + p.y.abs()).min().unwrap()
}
#[derive(Debug, Copy, Clone, Eq)]
struct Point {
    x: i64,
    y: i64,
    steps: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug)]
struct Route {
    direction: Direction,
    steps: usize,
}

fn parse_route(instruction: &&str) -> Route {
    let dir = instruction.chars().next().unwrap();

    let direction = match dir {
        'R' => Direction::Right,
        'L' => Direction::Left,
        'U' => Direction::Up,
        'D' => Direction::Down,
        _ => panic!("You fucked up"),
    };

    let steps = instruction[1..].parse::<usize>().unwrap();

    Route { direction, steps }
}
