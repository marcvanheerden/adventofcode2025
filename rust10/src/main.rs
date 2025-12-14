use std::{collections::VecDeque, str::FromStr};

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

    fn fewest_presses_for_joltage(&self) -> usize {
        // Used significant assistance from Claude Code
        // Solve as a system of linear Diophantine equations using Gaussian elimination
        // Use integer arithmetic with LCM/GCD to avoid floating point errors

        let n_rows = self.length;
        let n_cols = self.buttons.len();

        // Build augmented matrix [A | b] using i64 for exact arithmetic
        // Each row also has a denominator to handle fractions exactly
        let mut matrix: Vec<Vec<i64>> = vec![vec![0; n_cols + 1]; n_rows];
        let mut denoms: Vec<i64> = vec![1; n_rows]; // denominator for each row

        for (button_idx, button) in self.buttons.iter().enumerate() {
            for &pos in button {
                matrix[pos][button_idx] = 1;
            }
        }
        for (pos, &target) in self.joltage.iter().enumerate() {
            matrix[pos][n_cols] = target as i64;
        }

        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 { a.abs() } else { gcd(b, a % b) }
        }

        // Gaussian elimination to get Row Echelon Form
        let mut pivot_row = 0;
        let mut pivot_info: Vec<(usize, usize)> = Vec::new();

        for col in 0..n_cols {
            if pivot_row >= n_rows {
                break;
            }

            // Find a row with non-zero entry in this column
            let mut found_row = None;
            for row in pivot_row..n_rows {
                if matrix[row][col] != 0 {
                    found_row = Some(row);
                    break;
                }
            }

            let Some(swap_row) = found_row else {
                continue; // Skip this column (free variable)
            };

            // Swap rows
            matrix.swap(pivot_row, swap_row);
            denoms.swap(pivot_row, swap_row);
            pivot_info.push((pivot_row, col));

            // Eliminate in all other rows (for RREF)
            for row in 0..n_rows {
                if row == pivot_row || matrix[row][col] == 0 {
                    continue;
                }

                // Row[row] = Row[row] * pivot_val - Row[pivot_row] * matrix[row][col]
                // This keeps everything in integers
                let pivot_val = matrix[pivot_row][col];
                let row_val = matrix[row][col];

                // Multiply row by pivot_val, then subtract pivot_row * row_val
                for c in 0..=n_cols {
                    matrix[row][c] = matrix[row][c] * pivot_val - matrix[pivot_row][c] * row_val;
                }
                denoms[row] *= pivot_val;

                // Reduce by GCD to prevent overflow
                let mut row_gcd = denoms[row].abs();
                for c in 0..=n_cols {
                    row_gcd = gcd(row_gcd, matrix[row][c]);
                }
                if row_gcd > 1 {
                    for c in 0..=n_cols {
                        matrix[row][c] /= row_gcd;
                    }
                    denoms[row] /= row_gcd;
                }
            }

            pivot_row += 1;
        }

        // Identify free variables
        let pivot_cols: Vec<usize> = pivot_info.iter().map(|&(_, c)| c).collect();
        let free_vars: Vec<usize> = (0..n_cols).filter(|c| !pivot_cols.contains(c)).collect();

        if free_vars.is_empty() {
            // No free variables - unique solution
            // solution[pivot_col] = matrix[row][n_cols] / matrix[row][pivot_col]
            let mut solution = vec![0i64; n_cols];
            for &(row, col) in &pivot_info {
                let pivot_val = matrix[row][col];
                let const_val = matrix[row][n_cols];
                assert!(const_val % pivot_val == 0, "Non-integer solution");
                solution[col] = const_val / pivot_val;
            }
            return solution.iter().map(|&x| x as usize).sum();
        }

        // With free variables, enumerate to find minimum
        // Express pivot vars in terms of free vars:
        // solution[pivot] = (const - sum(coeff[free]*free_val)) / pivot_coeff

        let n_free = free_vars.len();

        // For each pivot, store: const_val, pivot_coeff, and coeffs for each free var
        struct PivotExpr {
            col: usize,
            const_val: i64,
            pivot_coeff: i64,
            free_coeffs: Vec<i64>,
        }

        let pivot_exprs: Vec<PivotExpr> = pivot_info
            .iter()
            .map(|&(row, col)| {
                let free_coeffs: Vec<i64> = free_vars.iter().map(|&fc| -matrix[row][fc]).collect();
                PivotExpr {
                    col,
                    const_val: matrix[row][n_cols],
                    pivot_coeff: matrix[row][col],
                    free_coeffs,
                }
            })
            .collect();

        // Enumerate free variable values
        let mut best_total = i64::MAX;
        let mut best_solution: Option<Vec<i64>> = None;

        // Estimate upper bounds for free vars
        let max_free: Vec<i64> = (0..n_free).map(|_| 500i64).collect();

        fn enumerate_int(
            idx: usize,
            current: &mut Vec<i64>,
            max_free: &[i64],
            pivot_exprs: &[PivotExpr],
            free_vars: &[usize],
            n_cols: usize,
            best_total: &mut i64,
            best_solution: &mut Option<Vec<i64>>,
        ) {
            if idx == current.len() {
                // Compute solution and check validity
                let mut solution = vec![0i64; n_cols];

                // Set free variables
                for (j, &free_col) in free_vars.iter().enumerate() {
                    solution[free_col] = current[j];
                }

                // Compute pivot variables
                let mut valid = true;
                for expr in pivot_exprs {
                    let mut num = expr.const_val;
                    for (j, &free_val) in current.iter().enumerate() {
                        num += expr.free_coeffs[j] * free_val;
                    }
                    if num % expr.pivot_coeff != 0 {
                        valid = false;
                        break;
                    }
                    let val = num / expr.pivot_coeff;
                    if val < 0 {
                        valid = false;
                        break;
                    }
                    solution[expr.col] = val;
                }

                if valid {
                    let total: i64 = solution.iter().sum();
                    if total < *best_total {
                        *best_total = total;
                        *best_solution = Some(solution);
                    }
                }
                return;
            }

            for v in 0..=max_free[idx] {
                current[idx] = v;
                enumerate_int(
                    idx + 1,
                    current,
                    max_free,
                    pivot_exprs,
                    free_vars,
                    n_cols,
                    best_total,
                    best_solution,
                );
            }
        }

        let mut current = vec![0i64; n_free];
        enumerate_int(
            0,
            &mut current,
            &max_free,
            &pivot_exprs,
            &free_vars,
            n_cols,
            &mut best_total,
            &mut best_solution,
        );

        let solution = best_solution.expect("No valid integer solution found");

        // Verify
        for (pos, &target) in self.joltage.iter().enumerate() {
            let computed: i64 = self
                .buttons
                .iter()
                .enumerate()
                .filter(|(_, btn)| btn.contains(&pos))
                .map(|(b, _)| solution[b])
                .sum();
            assert_eq!(
                computed, target as i64,
                "Verification failed: pos={}, computed={}, target={}",
                pos, computed, target
            );
        }

        solution.iter().map(|&x| x as usize).sum()
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
        .map(|m| m.fewest_presses_for_joltage())
        .sum();
    dbg!(part2);
}
