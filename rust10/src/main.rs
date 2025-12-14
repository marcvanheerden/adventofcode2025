use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    str::FromStr,
};

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
    length: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMachineError;

impl FromStr for Machine {
    type Err = ParseMachineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let button_start = s.chars().position(|c| c == '(').unwrap();
        let (part1, part2) = s.split_at(button_start);

        let joltage_start = part2.chars().position(|c| c == '{').unwrap();
        let (part2, part3) = part2.split_at(joltage_start);

        let lights: Vec<bool> = part1
            .trim()
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect();

        let length = lights.len();

        let buttons = part2
            .trim()
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap()
            .split(") (")
            .map(|s| s.split(',').map(|s| s.parse::<usize>().unwrap()).collect())
            .collect();

        let joltage = part3
            .trim()
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Ok(Machine {
            lights,
            buttons,
            joltage,
            length,
        })
    }
}

#[derive(Debug, Clone)]
struct LightJob {
    button_to_press: usize,
    lights: Vec<bool>,
    count: usize,
}

#[derive(Debug, Clone)]
struct JoltageJob {
    button_to_press: usize,
    joltage: Vec<usize>,
    count: usize,
    distance: usize,
}

impl Machine {
    fn fewest_presses_for_lights(&self) -> usize {
        let mut queue: VecDeque<LightJob> = (0..self.buttons.len())
            .map(|b| LightJob {
                button_to_press: b,
                lights: (0..self.length).map(|_| false).collect(),
                count: 0,
            })
            .collect();

        while !queue.is_empty() {
            let mut current = queue.pop_front().unwrap();

            // press button
            self.buttons[current.button_to_press]
                .iter()
                .for_each(|&idx| current.lights[idx] = !current.lights[idx]);

            current.count += 1;

            // check if it matches
            if self
                .lights
                .iter()
                .zip(current.lights.iter())
                .all(|(l, r)| l == r)
            {
                return current.count;
            }

            // otherwise create a job for each next button press
            (0..self.buttons.len())
                // no point in pressing same button twice in a row
                .filter(|&b| b != current.button_to_press)
                .for_each(|b| {
                    queue.push_back(LightJob {
                        button_to_press: b,
                        lights: current.lights.clone(),
                        count: current.count,
                    })
                });
        }
        unreachable!()
    }

    fn cost(target: &[f32], solution: &[f32], buttons: &[Vec<f32>]) -> f32 {
        let mut solution_value: Vec<f32> = (0..target.len()).map(|_| 0f32).collect();

        for (button, solution) in solution.iter().enumerate() {
            for (idx, solval) in solution_value.iter_mut().enumerate() {
                *solval += buttons[button][idx] * solution;
            }
        }

        let distance = target
            .iter()
            .zip(solution_value)
            .map(|(&a, b)| (a - b) * (a - b))
            .sum::<f32>();

        let integerness = solution.iter().map(|x| (x - x.round()).abs()).sum::<f32>();
        let presses = solution.iter().map(|x| x.abs()).sum::<f32>();
        let negatives: f32 = solution
            .iter()
            .filter(|x| PartialOrd::partial_cmp(x, &&0.0) == Some(Ordering::Less))
            .map(|x| x.abs())
            .sum();

        1.0 * distance + 0.5 * integerness + 0.5 * presses + 1.0 * negatives
    }

    fn fewest_presses_for_joltage(&self, learning_rate: f32) -> usize {
        let target: Vec<_> = self.joltage.iter().map(|&j| j as f32).collect();
        let mut solution: Vec<_> = (0..self.buttons.len()).map(|_| 0f32).collect();
        let buttons: Vec<Vec<f32>> = (0..self.buttons.len())
            .map(|b| {
                (0..self.length)
                    .map(|idx| {
                        if self.buttons[b].contains(&idx) {
                            1.0
                        } else {
                            0.0
                        }
                    })
                    .collect()
            })
            .collect();

        let starting_cost = Self::cost(&target, &solution, &buttons);

        let mut cost: Vec<_> = (0..self.buttons.len())
            .map(|idx| {
                let mut solution = solution.clone();
                solution[idx] += 1.0;
                Self::cost(&target, &solution, &buttons)
            })
            .collect();

        let mut dydx: Vec<_> = cost.iter().map(|c| c - starting_cost).collect();

        dbg!(starting_cost);
        for _ in 0..200 {
            dbg!(&cost);
            dbg!(&dydx);
            dbg!(&solution);
            let new_solution: Vec<_> = solution
                .iter()
                .zip(dydx.iter())
                .zip(cost.iter())
                .map(|((x, dydx), y)| x - learning_rate * y / dydx)
                .collect();

            let new_cost: Vec<f32> = (0..self.buttons.len())
                .map(|idx| {
                    let mut solution = solution.clone();
                    solution[idx] = new_solution[idx];
                    Self::cost(&target, &solution, &buttons)
                })
                .collect();

            dydx = new_cost
                .iter()
                .zip(cost.iter())
                .map(|(new, old)| new - old)
                .zip(new_solution.iter())
                .zip(solution.iter())
                .map(|((dy, new_x), old_x)| dy / (new_x - old_x))
                .collect();

            cost = new_cost;
            solution = new_solution;
        }

        solution.iter().map(|x| x.round() as usize).sum()
    }

    fn fewest_presses_for_joltage_old(&self) -> usize {
        let mut seen_joltages = HashMap::new();

        let mut queue: VecDeque<JoltageJob> = (0..self.buttons.len())
            .map(|b| JoltageJob {
                button_to_press: b,
                joltage: (0..self.length).map(|_| 0).collect(),
                count: 0,
                distance: self.joltage.iter().sum::<usize>(),
            })
            .collect();

        loop {
            let mut new_queue = Vec::new();
            while !queue.is_empty() {
                let mut current = queue.pop_front().unwrap();

                // press button
                self.buttons[current.button_to_press]
                    .iter()
                    .for_each(|&idx| current.joltage[idx] += 1);

                current.count += 1;

                // abort if already gone to far
                if self
                    .joltage
                    .iter()
                    .zip(current.joltage.iter())
                    .any(|(l, r)| l < r)
                {
                    continue;
                }

                // check if it matches
                if self
                    .joltage
                    .iter()
                    .zip(current.joltage.iter())
                    .all(|(l, r)| l == r)
                {
                    dbg!("done");
                    return current.count;
                }

                if let Some(presses) = seen_joltages.get(&current.joltage)
                    && *presses <= current.count
                {
                    continue;
                }
                seen_joltages.insert(current.joltage.clone(), current.count);

                let distance = self
                    .joltage
                    .iter()
                    .zip(current.joltage.iter())
                    .map(|(&j1, &j2)| j1.abs_diff(j2))
                    .max()
                    .unwrap();

                (0..self.buttons.len()).for_each(|b| {
                    new_queue.push(JoltageJob {
                        button_to_press: b,
                        joltage: current.joltage.clone(),
                        count: current.count,
                        distance,
                    })
                });
            }

            if new_queue.is_empty() {
                panic!("No solution")
            }
            new_queue.sort_unstable_by_key(|j| j.distance);
            let new_q_len = new_queue.len();
            dbg!(new_q_len);
            queue = new_queue
                .into_iter()
                .take(std::cmp::min(new_q_len, 1000000))
                .collect();
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Can't read file");
    let machines: Vec<_> = input
        .lines()
        .map(|l| Machine::from_str(l).unwrap())
        .collect();

    let part1: usize = machines.iter().map(|m| m.fewest_presses_for_lights()).sum();
    dbg!(part1);
    let part2: usize = machines
        .iter()
        .map(|m| m.fewest_presses_for_joltage_old())
        .sum();
    dbg!(part2);
}
