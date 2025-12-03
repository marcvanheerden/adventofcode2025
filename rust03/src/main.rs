fn max_jolts(batteries: &str, ndigits: usize) -> usize {
    let chars: Vec<char> = batteries.chars().collect();

    assert!(chars.len() >= ndigits);
    let mut digits = Vec::with_capacity(ndigits);
    let mut space = ndigits;
    let mut start = 0usize;

    while space > 0 {
        let (next_digit, next_start) = chars
            .iter()
            .skip(start)
            .take(chars.len() - start - space + 1)
            .enumerate()
            .fold(None, |acc, (i, x)| match acc {
                None => Some((x, i)),
                Some((best, best_i)) if x > best => Some((x, i)),
                _ => acc,
            })
            .expect("Empty bank");

        digits.push(next_digit);
        space -= 1;
        start += next_start + 1;
    }

    digits
        .into_iter()
        .collect::<String>()
        .parse()
        .expect("Non numeric battery values")
}

fn main() {
    let input = std::fs::read_to_string("day03_large.txt").expect("Unable to read input file");

    let part1: usize = input.lines().map(|x| max_jolts(x, 2)).sum();
    let part2: usize = input.lines().map(|x| max_jolts(x, 12)).sum();
    dbg!(part1);
    dbg!(part2);
}
