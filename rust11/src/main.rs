use std::collections::HashMap;

fn paths(start: &str, edges: &HashMap<&str, Vec<&str>>) -> usize {
    let mut queue = Vec::new();
    queue.push(start);

    let mut count = 0;
    loop {
        let mut next_queue = Vec::new();
        while let Some(current) = queue.pop() {
            for edge in edges.get(current).unwrap() {
                if *edge == "out" {
                    count += 1;
                } else {
                    next_queue.push(*edge);
                }
            }
        }
        if next_queue.is_empty() {
            break;
        }
        queue = next_queue;
    }

    count
}

#[derive(Debug, Clone)]
struct SplitCount {
    visited_neither: u64,
    visited_dac_only: u64,
    visited_fft_only: u64,
    visited_both: u64,
}

fn paths2(start: &str, edges: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut queue = HashMap::new();
    queue.insert(
        start,
        SplitCount {
            visited_neither: 1,
            visited_dac_only: 0,
            visited_fft_only: 0,
            visited_both: 0,
        },
    );

    let mut count = 0;
    loop {
        let mut next_queue = HashMap::new();
        for (current, run_count) in queue.into_iter() {
            if current == "out" {
                continue;
            }
            for edge in edges.get(current).unwrap() {
                let ent = next_queue.entry(*edge).or_insert(SplitCount {
                    visited_neither: 0,
                    visited_dac_only: 0,
                    visited_fft_only: 0,
                    visited_both: 0,
                });
                match *edge {
                    "out" => count += run_count.visited_both,
                    "fft" => {
                        ent.visited_fft_only +=
                            run_count.visited_fft_only + run_count.visited_neither;
                        ent.visited_both += run_count.visited_both + run_count.visited_dac_only;
                    }
                    "dac" => {
                        ent.visited_dac_only += run_count.visited_neither;
                        ent.visited_both += run_count.visited_both + run_count.visited_fft_only;
                    }
                    _ => {
                        ent.visited_neither += run_count.visited_neither;
                        ent.visited_dac_only += run_count.visited_dac_only;
                        ent.visited_fft_only += run_count.visited_fft_only;
                        ent.visited_both += run_count.visited_both;
                    }
                }
            }
        }
        if next_queue.is_empty() {
            break;
        }
        queue = next_queue;
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Can't read input");

    let edges: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|l| {
            let (from, to) = l.split_once(':').expect("Can't find delimiter");

            (from, to.trim().split(' ').collect())
        })
        .collect();

    let part1 = paths("you", &edges);
    dbg!(part1);
    let part2 = paths2("svr", &edges);
    dbg!(part2);
}
