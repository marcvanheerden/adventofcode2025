#[derive(Debug, Clone)]
struct Space {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Can't read file");

    let sections: Vec<_> = input.split("\n\n").collect();

    let presents: Vec<_> = sections
        .iter()
        .take(sections.len() - 1)
        .map(|l| l.chars().filter(|c| *c == '#').count())
        .collect();

    let spaces: Vec<_> = sections
        .last()
        .unwrap()
        .lines()
        .map(|l| {
            let (siz, pres) = l.split_once(':').unwrap();
            let (width, height) = siz.split_once('x').unwrap();
            let (width, height) = (width.parse::<usize>().unwrap(), height.parse().unwrap());
            let presents = pres
                .trim()
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            Space {
                width,
                height,
                presents,
            }
        })
        .collect();

    dbg!(&presents);
    let mut possible = 0;
    for space in spaces.iter() {
        dbg!(&space);
        let space_needed: usize = space
            .presents
            .iter()
            .enumerate()
            .map(|(idx, p)| p * presents[idx])
            .sum();

        if space_needed <= space.width * space.height {
            possible += 1;
        }
    }

    dbg!(possible);
}
