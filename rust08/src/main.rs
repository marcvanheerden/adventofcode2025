use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Junction {
    x: u64,
    y: u64,
    z: u64,
}

impl Junction {
    fn dist(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) * self.x.abs_diff(other.x)
            + self.y.abs_diff(other.y) * self.y.abs_diff(other.y)
            + self.z.abs_diff(other.z) * self.z.abs_diff(other.z)
    }
}

const CONNECTIONS: usize = 1000;

fn main() {
    let input = std::fs::read_to_string("day08_large.txt").expect("Can't read file");

    let juncs: Vec<Junction> = input
        .lines()
        .map(|l| {
            let mut splits = l.split(',').map(|s| s.parse::<u64>().unwrap());
            Junction {
                x: splits.next().unwrap(),
                y: splits.next().unwrap(),
                z: splits.next().unwrap(),
            }
        })
        .collect();

    let mut distances = Vec::new();

    for (idx1, junc1) in juncs.iter().enumerate() {
        for (idx2, junc2) in juncs.iter().enumerate().skip(idx1 + 1) {
            distances.push((idx1, idx2, junc1.dist(junc2)));
        }
    }

    distances.sort_unstable_by_key(|x| x.2);
    let mut circuits: Vec<Option<u16>> = (0..juncs.len()).map(|_| None).collect();
    let mut available_circuit_no = 0;

    for (conn_count, (idx1, idx2, _)) in distances.iter().enumerate() {
        match (circuits[*idx1], circuits[*idx2]) {
            (None, None) => {
                circuits[*idx1] = Some(available_circuit_no);
                circuits[*idx2] = Some(available_circuit_no);
                available_circuit_no += 1;
            }
            (Some(circ), None) => {
                circuits[*idx2] = Some(circ);
            }
            (None, Some(circ)) => {
                circuits[*idx1] = Some(circ);
            }
            (Some(circ1), Some(circ2)) => {
                if circ1 != circ2 {
                    circuits
                        .iter_mut()
                        .filter(|c| **c == Some(circ2))
                        .for_each(|c| *c = Some(circ1));
                }
            }
        }

        if conn_count >= CONNECTIONS - 1 {
            let distinct_circuits: HashSet<_> = circuits.iter().collect();

            if conn_count == CONNECTIONS - 1 {
                let mut circuit_sizes: Vec<_> = distinct_circuits
                    .iter()
                    .filter(|c| c.is_some())
                    .map(|circ| circuits.iter().filter(|c| c == circ).count())
                    .collect();

                circuit_sizes.sort_unstable();
                let part1: usize = circuit_sizes.into_iter().rev().take(3).product();

                println!("Part 1: {}", part1);
            }
            if distinct_circuits.len() == 1 {
                dbg!(conn_count);
                println!("Part 2: {}", juncs[*idx1].x * juncs[*idx2].x);
                break;
            }
        }
    }
}
