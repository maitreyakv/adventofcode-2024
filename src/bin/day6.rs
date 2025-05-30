use std::collections::HashSet;

const MAX_STEPS: usize = 10_000;

fn main() {
    println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    Map::from_str(input)
        .build()
        .run()
        .unwrap()
        .guard
        .previous
        .into_iter()
        .map(|(position, _)| position)
        .collect::<HashSet<_>>()
        .len()
}

fn part_2(input: &str) -> usize {
    let map = Map::from_str(input).build();
    let placements = map
        .clone()
        .run()
        .unwrap()
        .guard
        .previous
        .into_iter()
        .map(|(position, _)| position)
        .collect::<HashSet<_>>();
    let n_threads: usize = std::thread::available_parallelism().unwrap().into();
    let handles = (0..n_threads)
        .map(|thread_idx| {
            let map = map.clone();
            let placements = placements.clone();
            std::thread::spawn(move || {
                placements
                    .iter()
                    .skip(thread_idx)
                    .step_by(n_threads)
                    .map(|position| {
                        if let Ok(map) = map.clone().place_obstacle(position.to_owned()) {
                            if map.run().is_err() { 1 } else { 0 }
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
        })
        .collect::<Vec<_>>();
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

#[derive(Clone, Debug)]
struct Map {
    guard: Guard,
    obstacles: HashSet<Position>,
    bounds: Position,
}

impl Map {
    fn from_str(input: &str) -> MapBuilder {
        input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| (i, j, c)))
            .fold(MapBuilder::new(), |mut builder, (i, j, c)| {
                match c {
                    '#' => builder.add_obstacle((i as isize, j as isize)),
                    '^' => builder.add_guard((i as isize, j as isize), Direction::Up),
                    '>' => builder.add_guard((i as isize, j as isize), Direction::Right),
                    'v' => builder.add_guard((i as isize, j as isize), Direction::Down),
                    '<' => builder.add_guard((i as isize, j as isize), Direction::Left),
                    _ => {}
                };
                builder
            })
    }

    fn place_obstacle(mut self, position: Position) -> Result<Self, Occupied> {
        if self.obstacles.contains(&position) {
            Err(Occupied)
        } else {
            self.obstacles.insert(position);
            Ok(self)
        }
    }

    fn run(mut self) -> Result<Self, Loop> {
        for _ in 0..MAX_STEPS {
            if self.is_loop() {
                return Err(Loop);
            };
            if self.is_over() {
                return Ok(self);
            }
            self = self.step();
        }
        panic!("A simulation reached MAX_STEPS!")
    }

    fn step(mut self) -> Self {
        self.guard = if self.obstacles.contains(&self.guard.next_position()) {
            self.guard.turn_right()
        } else {
            self.guard.step()
        };
        self
    }

    fn is_over(&self) -> bool {
        !self.guard.is_in_bounds(&self.bounds)
    }

    fn is_loop(&self) -> bool {
        let guard_state = (
            self.guard.position.to_owned(),
            self.guard.direction.to_owned(),
        );
        self.guard.previous.contains(&guard_state)
    }
}

#[derive(Debug)]
struct Loop;

#[derive(Debug)]
struct Occupied;

struct MapBuilder {
    guard: Option<Guard>,
    obstacles: HashSet<Position>,
    bounds: Position,
}

impl MapBuilder {
    fn new() -> Self {
        Self {
            guard: None,
            obstacles: HashSet::new(),
            bounds: (0, 0),
        }
    }

    fn build(self) -> Map {
        Map {
            guard: self.guard.unwrap(),
            obstacles: self.obstacles,
            bounds: self.bounds,
        }
    }

    fn add_obstacle(&mut self, position: Position) {
        self.obstacles.insert(position);
        self.bounds.0 = std::cmp::max(self.bounds.0, position.0);
        self.bounds.1 = std::cmp::max(self.bounds.1, position.1);
    }

    fn add_guard(&mut self, position: Position, direction: Direction) {
        self.guard = Some(Guard {
            position,
            direction,
            previous: HashSet::new(),
        })
    }
}

#[derive(Clone, Debug)]
struct Guard {
    position: Position,
    direction: Direction,
    previous: HashSet<(Position, Direction)>,
}

impl Guard {
    fn turn_right(self) -> Self {
        Self {
            direction: self.direction.turn_right(),
            ..self
        }
    }

    fn step(mut self) -> Self {
        let next = self.next_position();
        self.previous
            .insert((self.position, self.direction.clone()));
        Self {
            position: next,
            ..self
        }
    }

    fn next_position(&self) -> Position {
        let (i, j) = self.position;
        match self.direction {
            Direction::Up => (i - 1, j),
            Direction::Right => (i, j + 1),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
        }
    }

    fn is_in_bounds(&self, bounds: &Position) -> bool {
        let (i, j) = &self.position;
        let i_range = 0..=bounds.0;
        let j_range = 0..=bounds.1;
        i_range.contains(i) && j_range.contains(j)
    }
}

type Position = (isize, isize);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 4374)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 1705)
    }
}

#[allow(clippy::items_after_test_module)]
const INPUT: &str = r#"......#........#..........#.................##......................#.............#..................#............#............#..
.....#............................................#.......................#.......##.................#..#...................#.#...
......#...................#............................#.....#.........#................................................#.........
...........................................#.......#..........#..#......#.........#.#.#.#................#.##........##...........
........#.........................................#.......#.....#..........#...#....##....#..#.......##................#......#...
...................#..#.....................................................#.................................................#...
....#..................#..........#........#.............#................................................................#.......
......#..............#..................#...........#......#.....#...........................#..#.#.....................#.....#...
......#...#...............................................#.....#.........................#..........#............................
......#......#..........................................#..................................#....#..#................#..##...#.....
........................................................................................#.........................................
................#..........#..........................#.##......................#..........#......................................
.............#.............................#......#..............................#....................#...........#.....#...#.....
.......#.................#..................#..........................#...........................................#..............
.......#.....................................................#..........................................#....#.#.....#......#.....
...#......#............#........#......................#...........................#.#....#..#....................#.............##
...........................##..#..#..........#....................#................................................#......#.......
.....................#...#.......................#...#..#.....#................#...........................................#....#.
.................#................#.............#...#...........................................................#..#.#............
#.....................#....................##.......#...................#..........#..........................#...................
.#..........##......#...#......#................#.....................................#.#.....#..............#....................
...#...#..#..........................................................#..#..#......................................................
..................#.#..............#...#...............................#...#....#.#.........#.........................##..........
.......#.........................#..............................................#........#....#....#.........#....................
...........#......#......#...........#........#.............................##.................................................#..
............................#...................................#......#........................................#.................
......................................##...........................................#.............#..#...........#...#.....#.......
...................................................#.................................#............................................
..........................#..........................................#..#.....#.................#.#......#........................
..................#...........#.............................#....#.#..................#................................#..........
...................#....#..#.........................................................#....#.......................................
..#..........................................................##..................................#......................#....#.#..
#.........................#.....#...............................#........................#................#.......................
......................#......#...#..........#......#..............................#...#......#.................................#..
...............#.....#............................................................................#...............................
#.....................#........#.#........................................................#.......................#.............#.
.......#...#..........#........#...........#.................#..................#....................................##.....#.....
.#......#..................................#.......................................................................#..............
.................#.....#...........................................................#..............................................
.........#....................................^......#..................#..............................#..#.............#...#.....
.............................................................#.....#....................................#.........................
....................#.................#........#........................................................#..#............#........#
.....#..........................................................#...........................#.................#....##.............
..............#.........#.................#............................#.............#......................#.#............#......
.............#.................#.........................................#...#................#.............#.....#...............
....#.....##..#......#........#............................................#..........#.......#.................................##
..........................................................................#.......................................................
.......#..........#.............................................................................................##..#.............
............................#...............#...............................................................................#.....
....#......#...............................................................................................................#......
..................................#......................#.....#............................................#.....#........#......
...............................#..........................................#.......................................................
.....#.............#...#..............#.#............#....................................................................#.......
..............................................#...............................#......##.......#................#..........#.#.....
........#.....##...............................................#.........................#........................................
.....#.........................#.#........#.............................................#.............#..............##...........
........................................#..##.........................#............#...........#.............#....................
.....................#.#................#............................#.................#.....#..............................#.....
................#.....................................#..#....................#..................................#................
...........#...................#.................#..........................................................#.......#...........#.
...........................#..................................#......#..............#............#.........#....#................#
..................................................................................................................#...............
..............#....#.#............................................................................#.....#.........#........#......
..#...........#.................#.#......#................................#..........................#...........................#
............#...........##...............................#..................................................#.....................
..............##.#..................................................................................#.........#......#......#.....
................................................................#...............#....#....##................#.....................
.........#..#.....................#........................................................#......#................#..............
...#..........#..........#..................#.......................#........................#...#................#.........#.....
...................................#..#...............................#..........#................................................
.........................#........................................................................................................
......................#.........#....................................................##............................#.....#........
.#....#.....................#.........................................................#....#........#.............................
..........................................#....................................#........................................#.........
.................................................#.......................................#........................................
#..#...............#......................#....#.............................................#................#........##.........
.........#....#........##.........#.....#.........#...............................................#..................#....#...#...
..#............................................................................#...............................................#..
.......................................................#.........#......#..........................#.#............................
.........#.............#.................##.....#.....#.........#............#.........#..................#.....#.............#...
#..........#.........#...............................................................................................#............
.......................#....................#..................................................#..................................
.#................#.................................#........................................#...........#........................
.........#........................................................................................................................
..................................................................................#..........#.....................#..............
..#...............................#.......#.....................#......................#.....#..........................#.........
......#....#..........................................................#..........................#................................
...........................................##..................................................##..................#...#..........
...............#....#...........................#........................................#..............#....................##...
#......#......................................................#..........#.........................#.........#....................
...#...................................#......#..........................................#.........................#.............#
#.....#.........#.............#.......................................#...............................#...........................
...................#.#..........................................................................#...............#.................
.......#......##..........#....................................................#..................#..........#....................
.............................................#.##..............#......................#......#..#..................#..........#...
..##...................#...................................#.........................................#...........#................
...................................................#.....................................#.......................#...............#
..........#.......##...................#...............#...............................................................#..........
.......................#............#............##.............................................#......##.................#...#...
.........#.................................................................................#............#....................#....
.......##.........#..................................................................#................#...........................
................#.............................................................................................#...................
......................#.......#....#.................................................#......#.#...............................#...
.........#.........#.#.....##.#..................................##...................................................#...........
.#......................................................#.......#......##........#..........#.........#....#......................
........#.#......................#................................................................................................
..................#.............................#.................#..................#......................#......##.#...........
................##.............................................#....#..........##.#.#....#........................................
..............#...................#.......#............................#.#.....................................................#..
............#..............................................#....#.....................#...#.......................#.....#......#..
..............#........................................................#............................................#.............
.......#.....#.........#..............#..............................#.......................#..#................#....#..#........
.......................#......#........................................#.................#..........#.............................
...........#..................#.............................#............................................................#........
......#....#......#.....................#.....................#........#.....................#...#.....#.#...................#....
#..............................................##...#.....#...........#..........................................#................
..........#..........#.#........................................#..#....#.........................................................
..................#......................................#...#.......#............................................................
...#.............................#..................................................#...........#.................................
................................##.#.............#........................#...#...............................................#...
...#...................................#.......................#........................#............#...............#....#.#.....
..............................................................##.......#...............#.........#..................#.............
....#............................#..................................#..#..#...............#..............##.......................
#...#.................##...............................#...........#...........#..............#.#.........#........#...#..........
..........................#.....#....................#..............#.............#..........#....................#.........#.....
......#.......................................................#........#.....#..........................#...#.........#...........
...............................#............#.....#...........................#...............#........#..........................
............#..#......#...#..#.....................#...............#.........#...........................................#.....#..
.......#..#..................#............#...........#..............................................................#............
....#......#.##..#......##..........#.......#............#...............#....#....................................#..........#.#."#;
