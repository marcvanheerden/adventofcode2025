const POSITIONS: isize = 100;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Can't read input file");

    let mut dial_pos = 50isize;
    let mut zeros = 0u32;
    let mut passes = 0u32;

    for line in input.lines() {
        let (dir, value) = line.split_at(1);
        let value: isize = value.parse().expect("Cant parse value");

        let full_rotations = value / POSITIONS;
        let partial_rotation = value % POSITIONS;

        match dir.chars().next() {
            Some('R') => {
                if (partial_rotation + dial_pos) >= POSITIONS {
                    passes += 1;
                }
                dial_pos += value;
            }
            Some('L') => {
                if (partial_rotation >= dial_pos) & (dial_pos > 0) {
                    passes += 1;
                }
                dial_pos -= value;
            }
            _ => panic!("Invalid input"),
        };

        passes += full_rotations as u32;

        dial_pos = dial_pos.rem_euclid(POSITIONS);
        if dial_pos == 0 {
            zeros += 1;
        }
    }

    dbg!(zeros);
    dbg!(passes);
}
