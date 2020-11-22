use crate::point::Point;
use std::collections::HashSet;

pub fn day3(input: &[String]) -> (i64, i64) {
    (part1(input), part2(input))
}

fn part1(input: &[String]) -> i64 {
    let paths = get_paths(input);
    let intersections = paths[0].intersection(&paths[1]);
    intersections.map(|p| p.x.abs() + p.y.abs()).min().unwrap()
}

fn part2(input: &[String]) -> i64 {
    let paths = get_paths(input);
    let intersections = paths[0].intersection(&paths[1]);

    let smallest_steps = intersections
        .map(|p| {
            let p1 = paths[0].get(p).unwrap();
            let p2 = paths[1].get(p).unwrap();
            p1.steps + p2.steps
        })
        .min()
        .unwrap();
    smallest_steps
}

fn get_paths(input: &[String]) -> Vec<HashSet<Point>> {
    input
        .iter()
        .map(|i| {
            let instructions = i.split(",").collect::<Vec<_>>();
            let routes = instructions.iter().map(parse_route).collect::<Vec<Route>>();
            let mut current_point = Point::default();

            let points = routes
                .iter()
                .flat_map(|r| {
                    let mut points: Vec<Point> = Vec::new();
                    let (dx, dy) = match r.direction {
                        Direction::Right => (1, 0),
                        Direction::Left => (-1, 0),
                        Direction::Up => (0, 1),
                        Direction::Down => (0, -1),
                    };

                    for _ in 0..r.steps {
                        let p = Point {
                            x: current_point.x + dx,
                            y: current_point.y + dy,
                            steps: current_point.steps + 1,
                        };
                        points.push(p);
                        current_point = p;
                    }
                    points
                })
                .collect::<HashSet<Point>>();

            points
        })
        .collect::<Vec<HashSet<Point>>>()
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
    
