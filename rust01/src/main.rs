const POSITIONS: isize = 100;

fn main() {
    let input = std::fs::read_to_string("day01_large.txt").expect("Can't read input file");

    let mut dial_pos = 50isize;
    let mut prev_dial_pos;
    let mut zeros = 0u32;
    let mut passes = 0u32;

    for line in input.lines() {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let value = chars.collect::<String>().parse::<isize>().unwrap();

        prev_dial_pos = dial_pos;
        if dir == 'R' {
            dial_pos += value;
        } else {
            dial_pos -= value;
        }

        if dial_pos < 0 {
            let mut revs = dial_pos.abs() / POSITIONS;
            passes += revs as u32;
            if prev_dial_pos > 0 {
                passes += 1;
            }

            if dial_pos.abs() % POSITIONS != 0 {
                revs += 1;
            }
            dial_pos += revs * POSITIONS;
        } else if dial_pos >= POSITIONS {
            passes += (dial_pos / POSITIONS) as u32;
            dial_pos %= POSITIONS;
        } else if dial_pos == 0 {
            passes += 1;
        }

        if dial_pos == 0 {
            zeros += 1;
        }
    }

    dbg!(zeros);
    dbg!(passes);
}
