use std::collections::HashSet;

fn get_rolls(line_no: usize, line: &str) -> Vec<(usize, usize)> {
    line.chars()
        .enumerate()
        .filter_map(|(col_no, chr)| {
            if chr == '@' {
                Some((line_no, col_no))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Can't read input file");

    let mut rolls: HashSet<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(line_no, line)| get_rolls(line_no, line))
        .collect();

    let initial_rollcount = rolls.len();

    loop {
        let mut to_remove = Vec::new();
        for roll in rolls.iter() {
            let neighbours: HashSet<&(usize, usize)> = rolls
                .iter()
                .filter(|(line, col)| roll.0.abs_diff(*line) <= 1 && roll.1.abs_diff(*col) <= 1)
                .collect();

            // neighbours includes self hence <= 4 instead of < 4
            if neighbours.len() <= 4 {
                to_remove.push(*roll);
            }
        }
        let removable_count = to_remove.len();
        dbg!(removable_count);
        for roll in to_remove.iter() {
            rolls.remove(roll);
        }
        if removable_count == 0 {
            break;
        }
    }
    dbg!(initial_rollcount - rolls.len());
}
