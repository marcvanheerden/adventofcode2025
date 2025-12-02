fn sum_repeats(range: &str) -> (usize, usize) {
    let (start, end) = range.split_once('-').expect("Bad input format");
    let (start, end): (usize, usize) = (start.parse().unwrap(), end.parse().unwrap());

    let mut total = 0;
    let mut total2 = 0;
    for code in start..=end {
        let digits = code.checked_ilog10().unwrap_or(0) + 1;
        let first_half = code / 10usize.pow(digits / 2);
        let second_half = code % 10usize.pow(digits / 2);

        if digits % 2 == 0 && first_half == second_half {
            total += code;
            total2 += code;
        } else if !is_valid(code, digits as usize) {
            total2 += code;
        }
    }

    (total, total2)
}

fn is_valid(code: usize, length: usize) -> bool {
    'outer: for split_length in 1..=(length / 2) {
        let mut run_code = code;
        if length.is_multiple_of(split_length) {
            let mut divisor = 10usize.pow((length - split_length) as u32);
            let expected_value = run_code / divisor;
            run_code %= divisor;
            divisor /= 10usize.pow(split_length as u32);

            while divisor > 0 {
                if (run_code / divisor) != expected_value {
                    continue 'outer;
                }
                run_code %= divisor;
                divisor /= 10usize.pow(split_length as u32);
            }

            return false;
        }
    }

    true
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Can't read input file");
    let input = input.trim();

    let (part1, part2): (Vec<usize>, Vec<usize>) = input.split(',').map(sum_repeats).unzip();
    dbg!(part1.iter().sum::<usize>());
    dbg!(part2.iter().sum::<usize>());
}
