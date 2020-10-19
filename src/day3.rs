pub fn day3(input: &[String]) -> (i64, i64) {
    (part1(input), part2(input))
}

fn part1(_input: &[String]) -> i64 {
    0
}

fn part2(_input: &[String]) -> i64 {
    0
}

fn calc_shortest_distance(input: Vec<&str>) -> i64 {
    let _points: Vec<Wire> = input.iter().map(|ref s| parse_to_wire(s)).collect();
    0
}
#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

fn get_points(start: (i32, i32), input: &[WirePath]) -> Vec<Point> {
    input
        .iter()
        .fold(Vec::new(), |mut arr: Vec<Point>, i: &WirePath| {
            let mut last_position = if !arr.is_empty() {
                arr.last().unwrap()
            } else {
                arr.push(Point { x: 0, y: 0 });
                arr.last().unwrap()
            };

            match i.direction {
                Direction::Right => {
                    for _ in 0..i.steps {
                        let p = Point {
                            x: last_position.x + 1,
                            y: last_position.y,
                        };

                        arr.push(p);
                        println!("Crap: {:?}", arr);
                        last_position = arr.last().unwrap();
                    }
                }

                Direction::Left => {}
                Direction::Up => {}
                Direction::Down => {}
            }
            arr
        })
}

#[derive(Debug, PartialEq)]
struct WirePath {
    direction: Direction,
    steps: usize,
}

struct Wire {
    paths: Vec<WirePath>,
}

fn parse_to_wire(input: &str) -> Wire {
    let wire_paths: Vec<WirePath> = input
        .split(',')
        .map(|s| s.trim())
        .map(parse_to_wire_path)
        .collect();
    Wire { paths: wire_paths }
}

fn parse_to_wire_path(input: &str) -> WirePath {
    let dir = input[0..1].parse().unwrap_or('g');
    let direction = match dir {
        'L' => Direction::Left,
        'R' => Direction::Right,
        'U' => Direction::Up,
        'D' => Direction::Down,
        _ => panic!("Can't parse direction: {}", dir),
    };

    let steps: usize = match input[1..].parse() {
        Ok(s) => s,
        Err(e) => panic!("{}: {}", e, input[1..].to_string()),
    };

    WirePath { direction, steps }
}

#[cfg(test)]
mod tests {
    mod distance {
        use crate::day3::calc_shortest_distance;
        #[test]
        fn test_calculate_distance() {
            let input = vec![
                "R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83",
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51,U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            ];

            let distance = calc_shortest_distance(input);
            assert_eq!(distance, 135);
        }
    }

    mod points {
        use crate::day3::{get_points, Direction, Point, WirePath};

        #[test]
        fn test_get_points() {
            let input = &[WirePath {
                direction: Direction::Right,
                steps: 1,
            }];
            let start = (0, 0);
            let points = get_points(start, input);
            assert_eq!(points, vec![Point { x: 0, y: 0 }, Point { x: 1, y: 0 },])
        }
    }

    mod wire {
        use crate::day3::parse_to_wire;
        use crate::day3::{Direction, WirePath};

        #[test]
        fn test_parse_wire() {
            let input = "R75, L43";
            let wire = parse_to_wire(input);
            assert_eq!(wire.paths.len(), 2);
            assert_eq!(
                wire.paths[0],
                WirePath {
                    direction: Direction::Right,
                    steps: 75
                }
            );
            assert_eq!(
                wire.paths[1],
                WirePath {
                    direction: Direction::Left,
                    steps: 43
                }
            );
        }
    }
}
