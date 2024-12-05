use crate::file::{self, load_content};

const PATH: &str = "src/inputs/test";
const WORD: &str = "XMAS";
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn recurse_help(
    vec: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    word_index: usize,
    visited: &mut Vec<Vec<bool>>,
) -> i32 {
    if vec[i][j] != WORD.chars().nth(word_index).unwrap() || visited[i][j] {
        return 0;
    }

    if word_index == WORD.len() - 1 {
        return 1;
    }

    visited[i][j] = true;

    let mut total = 0;

    for (di, dj) in DIRECTIONS.iter() {
        let new_i = i as i32 + di;
        let new_j = j as i32 + dj;

        if new_i >= 0
            && new_i < vec.len() as i32
            && new_j >= 0
            && new_j < vec[0].len() as i32
            && visited[new_i as usize][new_j as usize] != true
        {
            total += recurse_help(vec, new_i as usize, new_j as usize, word_index + 1, visited);
        }
    }

    visited[i][j] = false;
    total
}

fn find_recurse(outer: usize, inner: usize, vec: &mut Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    let mut visited = vec![vec![false; inner]; outer];
    for i in 0..outer {
        for j in 0..inner {
            if vec[i][j] == 'X' {
                total += recurse_help(vec, i, j, 0, &mut visited);
            }
        }
    }
    total
}

#[allow(unused_variables, unused_mut, unused_assignments)]
pub fn main() -> std::io::Result<()> {
    let mut input = load_content(PATH)?;
    let mut in_vec: Vec<Vec<char>> = Vec::new();
    let (mut total_1, mut total_2) = (0, 0);

    for line in input.lines() {
        let mut ln_vec: Vec<char> = line.chars().collect();
        in_vec.push(ln_vec);
    }

    let (outer_len, inner_len) = (in_vec.len(), in_vec[0].len());

    total_1 = find_recurse(outer_len, inner_len, &mut in_vec);

    file::print_challenges(4, total_1, total_2);
    Ok(())
}
