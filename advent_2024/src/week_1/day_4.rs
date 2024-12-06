use crate::file::{self, load_content};

const PATH: &str = "src/inputs/in_4";
const WORD_PART_1: &str = "XMAS";
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
const XDIRECTIONS: [((i32, i32), (i32, i32)); 2] = [((-1, -1), (1, 1)), ((-1, 1), (1, -1))];

fn part_1(outer: usize, inner: usize, vec: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    for i in 0..outer {
        for j in 0..inner {
            if WORD_PART_1.starts_with(vec[i][j]) {
                total += part_1_helper(&vec, i, j, 0, None);
            }
        }
    }
    total
}

fn part_2(outer: usize, inner: usize, vec: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    for i in 0..outer {
        for j in 0..inner {
            if vec[i][j] == 'A' {
                total += part_2_helper(&vec, i, j);
            }
        }
    }
    total
}

fn check_bounds(i: i32, j: i32, vec: &Vec<Vec<char>>) -> bool {
    if i >= 0 && j >= 0 && i < vec.len() as i32 && j < vec[0].len() as i32 {
        return true;
    }
    false
}
#[allow(unused_variables)]
fn part_2_helper(vec: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut total = 0;

    let process_direction = |dir1: ((i32, i32), (i32, i32)), dir2: ((i32, i32), (i32, i32))| {
        let (dir1_start, dir1_end) = dir1;
        let (dir2_start, dir2_end) = dir2;

        let (new_i1_start, new_j1_start) = (i as i32 + dir1_start.0, j as i32 + dir1_start.1);
        let (new_i1_end, new_j1_end) = (i as i32 + dir1_end.0, j as i32 + dir1_end.1);

        let (new_i2_start, new_j2_start) = (i as i32 + dir2_start.0, j as i32 + dir2_start.1);
        let (new_i2_end, new_j2_end) = (i as i32 + dir2_end.0, j as i32 + dir2_end.1);

        if check_bounds(new_i1_start, new_j1_start, vec)
            && check_bounds(new_i1_end, new_j1_end, vec)
            && check_bounds(new_i2_start, new_j2_start, vec)
            && check_bounds(new_i2_end, new_j2_end, vec)
        {
            let check_first_diagonal = (vec[new_i1_start as usize][new_j1_start as usize] == 'M'
                && vec[i][j] == 'A'
                && vec[new_i1_end as usize][new_j1_end as usize] == 'S')
                || (vec[new_i1_start as usize][new_j1_start as usize] == 'S'
                    && vec[i][j] == 'A'
                    && vec[new_i1_end as usize][new_j1_end as usize] == 'M');

            let check_second_diagonal = (vec[new_i2_start as usize][new_j2_start as usize] == 'M'
                && vec[i][j] == 'A'
                && vec[new_i2_end as usize][new_j2_end as usize] == 'S')
                || (vec[new_i2_start as usize][new_j2_start as usize] == 'S'
                    && vec[i][j] == 'A'
                    && vec[new_i2_end as usize][new_j2_end as usize] == 'M');

            if check_first_diagonal && check_second_diagonal {
                return 1;
            }
        }
        0
    };

    total += process_direction(XDIRECTIONS[0], XDIRECTIONS[1]);
    total
}

fn part_1_helper(
    vec: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    word_index: usize,
    direction: Option<(i32, i32)>,
) -> i32 {
    if vec[i][j] != WORD_PART_1.chars().nth(word_index).unwrap() {
        return 0;
    }

    if word_index == WORD_PART_1.len() - 1 {
        return 1;
    }

    let mut total = 0;

    let mut process_direction = |di: i32, dj: i32| {
        let new_i = i as i32 + di;
        let new_j = j as i32 + dj;

        if new_i >= 0 && new_i < vec.len() as i32 && new_j >= 0 && new_j < vec[0].len() as i32 {
            total += part_1_helper(
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
    let (answer_1, answer_2): (i32, i32);

    for line in input.lines() {
        let ln_vec: Vec<char> = line.chars().collect();
        in_vec.push(ln_vec);
    }

    let (outer_len, inner_len) = (in_vec.len(), in_vec[0].len());
    answer_1 = part_1(outer_len, inner_len, &in_vec);
    answer_2 = part_2(outer_len, inner_len, &in_vec);

    file::print_challenges(4, answer_1, answer_2);
    Ok(())
}
