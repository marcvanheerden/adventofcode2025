fn remove_overlap(range1: (u64, u64), range2: (u64, u64)) -> ((u64, u64), Option<(u64, u64)>) {
    if (range1.0 > range2.1) || (range2.0 > range1.1) {
        return (range1, Some(range2));
    }

    if (range1.0 <= range2.0) && (range1.1 >= range2.1) {
        // range1 contains range2
        return (range1, None);
    }

    if (range2.0 <= range1.0) && (range2.1 >= range1.1) {
        // range2 contains range1
        return (range2, None);
    }

    (
        (
            std::cmp::min(range1.0, range2.0),
            std::cmp::max(range1.1, range2.1),
        ),
        None,
    )
}

fn main() {
    let input = std::fs::read_to_string("day05_large.txt").expect("Can't read file");
    let (ranges, ingreds) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(u64, u64)> = ranges
        .lines()
        .map(|l: &str| {
            let (left, right) = l.split_once('-').unwrap();
            (
                left.parse::<u64>().expect("Can't parse interval"),
                right.parse::<u64>().expect("Can't parse interval"),
            )
        })
        .collect();

    let ingreds: Vec<_> = ingreds.lines().map(|l| l.parse::<u64>().unwrap()).collect();
    let mut n_fresh = 0;

    'outer: for ingred in ingreds.into_iter() {
        for (lower, upper) in ranges.iter() {
            if (ingred >= *lower) && (ingred <= *upper) {
                n_fresh += 1;
                continue 'outer;
            }
        }
    }

    dbg!(n_fresh);

    let mut changes = Vec::new();
    'outer: loop {
        for (no, change) in changes {
            if let Some(changed_range) = change {
                ranges[no] = changed_range;
            } else {
                ranges.remove(no);
            }
        }
        changes = Vec::new();
        for (range_no1, range1) in ranges.iter().enumerate() {
            for (range_no2, range2) in ranges.iter().enumerate().skip(range_no1 + 1) {
                let (new_range1, new_range2) = remove_overlap(*range1, *range2);

                if new_range1 != *range1 {
                    changes.push((range_no1, Some(new_range1)));
                }

                if new_range2 != Some(*range2) {
                    changes.push((range_no2, new_range2))
                }

                if !changes.is_empty() {
                    continue 'outer;
                }
            }
        }
        break;
    }

    let part2: u64 = ranges.iter().map(|(start, end)| end - start + 1).sum();
    dbg!(part2);
}
