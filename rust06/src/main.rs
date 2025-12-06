fn main() {
    let input = std::fs::read_to_string("input.txt").expect("cant read file");

    let lines: Vec<&str> = input.lines().collect();

    let numbers: Vec<Vec<u64>> = lines
        .iter()
        .take(lines.len() - 1)
        .map(|l| {
            l.split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let ops: Vec<&str> = lines
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect();

    numbers.iter().for_each(|n| assert_eq!(n.len(), ops.len()));

    let mut part1 = 0;
    for (idx, op) in ops.into_iter().enumerate() {
        let mut val = match op {
            "+" => 0,
            "*" => 1,
            _ => panic!(),
        };

        for number_list in numbers.iter() {
            match op {
                "+" => {
                    val += number_list[idx];
                }
                "*" => {
                    val *= number_list[idx];
                }
                _ => panic!(),
            }
        }
        part1 += val;
    }

    dbg!(part1);
    let ops: Vec<(usize, char)> = lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_idx, op)| *op != ' ')
        .collect();

    let mut ends: Vec<usize> = ops.iter().skip(1).map(|(idx, _op)| *idx).collect();
    let max_line_length = lines.iter().map(|l| l.chars().count()).max().unwrap();
    ends.push(max_line_length);

    let mut total: u64 = 0;

    for (idx, op) in ops.into_iter() {
        let end = ends.iter().find(|x| **x > idx).unwrap();
        let mut sect_nums = Vec::with_capacity(lines.len());
        for col in idx..*end {
            let mut number = 0u64;
            for line in lines.iter().take(lines.len() - 1) {
                if let Some(digit) = line.chars().nth(col) && digit != ' ' {
                    number *= 10;
                    number += (digit as u8 - b'0') as u64;
                }
            }
            if number > 0 {
                sect_nums.push(number);
            }
        }
        
        total += match op {
            '+' => sect_nums.into_iter().sum::<u64>(),
            '*' => sect_nums.into_iter().product(),
            _ => panic!(),
        }
    }

    dbg!(total);
}
