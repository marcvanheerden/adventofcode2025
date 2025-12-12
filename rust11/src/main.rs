use std::collections::{HashMap, VecDeque};

fn paths(start: &str, edges: &HashMap<&str, Vec<&str>>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut count = 0;
    loop {
        let mut next_queue = Vec::new();
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            for edge in edges.get(current).unwrap() {
                if *edge == "out" {
                    count += 1;
                } else {
                    next_queue.push(edge);
                }
            }
        }
        if next_queue.is_empty() {
            break;
        }
        queue = next_queue.into_iter().cloned().collect();
    }

    count
}

fn paths2(start: &str, edges: &HashMap<&str, Vec<&str>>) -> usize {
    let mut queue = HashMap::new();
    // counts: visited neither, visited fft only, visited dac only, visited both
    queue.insert(start, (1, 0, 0, 0));

    let mut count = 0;
    loop {
        let mut next_queue = HashMap::new();
        for (current, run_count) in queue.into_iter() {
            if run_count.0 + run_count.1 + run_count.2 + run_count.3 == 0 {
                continue;
            }
            for edge in edges.get(current).unwrap() {
                let ent = next_queue.entry(*edge).or_insert((0, 0, 0, 0));
                match *edge {
                    "out" => count += run_count.3,
                    "fft" => {
                        ent.0 += 0;
                        ent.1 += run_count.0 + run_count.1;
                        ent.2 += 0;
                        ent.3 += run_count.2 + run_count.3;
                    }
                    "dac" => {
                        ent.0 += 0;
                        ent.1 += 0;
                        ent.2 += run_count.0 + run_count.2;
                        ent.3 += run_count.1 + run_count.3;
                    }
                    _ => {
                        ent.0 += run_count.0;
                        ent.1 += run_count.1;
                        ent.2 += run_count.2;
                        ent.3 += run_count.3;
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
