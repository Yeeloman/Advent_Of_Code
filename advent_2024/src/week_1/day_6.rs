use crate::file;
use std::collections::HashSet;
use std::io::{Error, ErrorKind};

const DAY: i32 = 6;
const PATH: &str = "src/inputs/test";

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

#[allow(dead_code)]
#[derive(Debug)]
struct Guard {
    orientation: Direction,
    position: (usize, usize),
    visited: HashSet<(usize, usize)>,
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
            visited,
        }
    }

    fn walk(&mut self, next: (usize, usize)) {
        let (di, dj) = self.orientation.vector();

        let new_i = (next.0 as i32 + di) as usize;
        let new_j = (next.1 as i32 + dj) as usize;
        self.position = (new_i, new_j);
        self.visited.insert(self.position);
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
    let mut in_patrol: bool = true;
    let mut answer_2 = 0;
    let mut input = file::load_content(PATH)?;

    let (mut grd, map) = init_setup(input);
    while in_patrol {
        let crnt_pos = grd.position;
        let mv_dir = grd.orientation.vector();
        let (new_i, new_j) = (crnt_pos.0 as i32+ mv_dir.0, crnt_pos.1 as i32+ mv_dir.1);
        if new_i < 0 || new_i > map.len() as i32 - 1|| new_j < 0 || new_j > map[0].len() as i32 - 1 {
            in_patrol = false;
            grd.walk((new_i as usize, new_j as usize));
            continue;
        }

        if map[new_i as usize][new_j as usize] == "#".chars().next().unwrap() {
            println!("guard turns");
            grd.turn();
        } else {
            println!("guard position {:?}", grd.position);
            println!("quard orientation {:?}", grd.orientation);
            println!("guard vec {:?}", grd.orientation.vector());
            println!("quard path count {:?}", grd.path_count());

            grd.walk((new_i as usize, new_j as usize));
            println!("guard walks");
            println!("guard position {:?}", grd.position);
            println!("quard orientation {:?}", grd.orientation);
            println!("guard vec {:?}", grd.orientation.vector());
            println!("quard path count {:?}", grd.path_count());
        }
    }

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
    #[allow(dead_code)]
    pub fn main() {
        todo!()
    }
}

mod part_2 {
    #[allow(dead_code)]
    pub fn main() {
        todo!()
    }
}
