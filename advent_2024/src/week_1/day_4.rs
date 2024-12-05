use crate::file::{self, load_content};

const PATH: &str = "src/inputs/in_4";
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

fn find_recurse(outer: usize, inner: usize, vec: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    for i in 0..outer {
        for j in 0..inner {
            if WORD.starts_with(vec[i][j]) {
                total += recurse_help(&vec, i, j, 0, None);
            }
        }
    }
    total
}

fn recurse_help(
    vec: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    word_index: usize,
    direction: Option<(i32, i32)>,
) -> i32 {
    if vec[i][j] != WORD.chars().nth(word_index).unwrap() {
        return 0;
    }

    if word_index == WORD.len() - 1 {
        return 1;
    }

    let mut total = 0;

    let mut process_direction = |di: i32, dj: i32| {
        let new_i = i as i32 + di;
        let new_j = j as i32 + dj;

        if new_i >= 0 && new_i < vec.len() as i32 && new_j >= 0 && new_j < vec[0].len() as i32 {
            total += recurse_help(
                vec,
                new_i as usize,
                new_j as usize,
                word_index + 1,
                Some((di, dj)),
            );
        }
    };

    if let Some((di, dj)) = direction {
        process_direction(di, dj);
    } else {
        for (di, dj) in DIRECTIONS.iter() {
            process_direction(*di, *dj);
        }
    }
    total
}

#[allow(unused_variables)]
pub fn main() -> std::io::Result<()> {
    let input = load_content(PATH)?;
    let mut in_vec: Vec<Vec<char>> = Vec::new();
    let (total_1, total_2): (i32, i32);

    for line in input.lines() {
        let ln_vec: Vec<char> = line.chars().collect();
        in_vec.push(ln_vec);
    }

    let (outer_len, inner_len) = (in_vec.len(), in_vec[0].len());
    total_1 = find_recurse(outer_len, inner_len, &in_vec);

    file::print_challenges(4, total_1, 0);
    Ok(())
}
