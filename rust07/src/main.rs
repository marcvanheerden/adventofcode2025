use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Can't read input file");

    let mut splitters = HashSet::new();

    let mut start = (0usize, 0usize);
    let lines = input.lines();
    let width = lines.clone().next().unwrap().chars().count();
    let height = lines.clone().count();

    for (row_no, row) in lines.enumerate() {
        for (col_no, chr) in row.chars().enumerate() {
            match chr {
                'S' => {
                    start = (row_no, col_no);
                }
                '^' => {
                    splitters.insert((row_no, col_no));
                }
                _ => {}
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 1u64));
    let mut timelines = 0;
    let mut splits = 0u32;

    loop {
        if queue.is_empty() {
            break;
        }
        let (row, col, count) = queue.pop_front().unwrap();
        if row == (height - 1) {
            timelines += count;
            continue;
        }
        let mut to_add = Vec::new();

        if splitters.contains(&(row + 1, col)) {
            if col > 0 {
                to_add.push((row + 1, col - 1));
            }
            if col < (width - 1) {
                to_add.push((row + 1, col + 1));
            }
            splits += 1;
        } else {
            to_add.push((row + 1, col));
        }

        for (new_row, new_col) in to_add.into_iter() {
            if let Some((_, _, c)) = queue
                .iter_mut()
                .find(|(x, y, _)| *x == new_row && *y == new_col)
            {
                *c += count;
            } else {
                queue.push_back((new_row, new_col, count)); // or whatever fallback you want
            }
        }
    }
    dbg!(splits);
    dbg!(timelines);
}
