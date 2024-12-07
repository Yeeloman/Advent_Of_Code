use crate::file;
use std::collections::HashSet;
use std::io::{Error, ErrorKind};

const DAY: i32 = 6;
const PATH: &str = "src/inputs/in_6";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    orientation: Direction,
    position: (usize, usize),
    starting_pos: (usize, usize),
    visited: HashSet<(usize, usize)>,
    state_tracker: HashSet<(usize, usize, Direction)>,
}

impl Guard {
    fn new(start_char: &char, i: usize, j: usize) -> Self {
        let orientation = match start_char {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("Invalid start"),
        };

        let mut visited = HashSet::new();
        visited.insert((i, j));

        Guard {
            orientation,
            position: (i, j),
            starting_pos: (i, j),
            visited,
            state_tracker: HashSet::from([(i, j, orientation)]),
        }
    }

    fn walk(&mut self, next: (usize, usize)) {
        self.state_tracker.insert((self.position.0, self.position.1, self.orientation));
        self.position = (next.0, next.1);
        self.visited.insert(self.position);
    }

    fn is_stuck(&self) -> bool {
        let current_state = (self.position.0, self.position.1, self.orientation);
        self.state_tracker.contains(&current_state)
    }

    fn turn(&mut self) {
        self.orientation = match self.orientation {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn path_count(&self) -> usize {
        self.visited.len()
    }
}

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let mut answer_2 = 0;
    let mut input = file::load_content(PATH)?;

    let (mut grd, mut map) = init_setup(input);
    let grd_2 = grd.clone();
    part_1::main(&mut grd, &map);

    let answer_2 = part_2::main(&grd_2, &mut map);

    let answer_1 = grd.path_count() as i32;
    file::print_challenges(DAY, answer_1, answer_2);
    Ok(())
}

fn init_setup(input: String) -> (Guard, Vec<Vec<char>>) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut guard: Option<Guard> = None;

    for (i, line) in input.lines().enumerate() {
        let tmp: Vec<char> = line.chars().collect();
        for (j, c) in tmp.iter().enumerate() {
            if *c != '.' && *c != '#' {
                guard = Some(Guard::new(&c, i, j));
            }
        }
        map.push(tmp);
    }

    let guard = guard
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "No valid guard found"))
        .unwrap();

    (guard, map)
}

mod part_1 {
    use crate::week_1::day_6::Guard;

    #[allow(dead_code)]
    pub fn main(grd: &mut Guard, map: &Vec<Vec<char>>) {
        let mut in_patrol: bool = true;
        let hashtag_char = '#';

        while in_patrol {
            let crnt_pos = grd.position;
            let mv_dir = grd.orientation.vector();
            let (new_i, new_j) = (crnt_pos.0 as i32 + mv_dir.0, crnt_pos.1 as i32 + mv_dir.1);
            if new_i < 0
                || new_i > map.len() as i32 - 1
                || new_j < 0
                || new_j > map[0].len() as i32 - 1
            {
                in_patrol = false;
                continue;
            }

            if map[new_i as usize][new_j as usize] == hashtag_char {
                grd.turn();
            } else {
                grd.walk((new_i as usize, new_j as usize));
            }
        }
    }
}

mod part_2 {
    use crate::week_1::day_6::Guard;

    #[allow(dead_code, unused_mut, unused_variables)]
    pub fn main(grd: &Guard, map: &mut Vec<Vec<char>>) -> i32 {
        let mut in_patrol: bool;
        let hashtag_char = '#';
        let point_char = '.';
        let zero_char = '0';
        let mut possible_loops = 0;

        for outer in 0..map.len() {
            for inner in 0..map[0].len() {
                in_patrol = true;
                let mut grd_clone = grd.clone();

                if map[outer][inner] == hashtag_char || grd_clone.starting_pos == (outer, inner) {
                    continue;
                }

                map[outer][inner] = zero_char;
                while in_patrol {
                    let crnt_pos = grd_clone.position;
                    let mv_dir = grd_clone.orientation.vector();
                    let (new_i, new_j) =
                        (crnt_pos.0 as i32 + mv_dir.0, crnt_pos.1 as i32 + mv_dir.1);

                    if new_i < 0
                        || new_i > map.len() as i32 - 1
                        || new_j < 0
                        || new_j > map[0].len() as i32 - 1
                    {
                        in_patrol = false;
                        continue;
                    }

                    if map[new_i as usize][new_j as usize] == hashtag_char
                        || map[new_i as usize][new_j as usize] == zero_char
                    {
                        grd_clone.turn();
                    } else {
                        grd_clone.walk((new_i as usize, new_j as usize));
                    }
                    if grd_clone.is_stuck() {
                        possible_loops += 1;
                        break;
                    }
                }
                map[outer][inner] = point_char;
            }
        }
        possible_loops
    }
}
