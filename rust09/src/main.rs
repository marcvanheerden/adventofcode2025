use std::cmp::{max, min};

fn lines_cross(line1: &[(u64, u64)], line2: &[(u64, u64); 2]) -> bool {
    let line1_min_x = min(line1[0].0, line1[1].0);
    let line1_max_x = max(line1[0].0, line1[1].0);
    let line1_min_y = min(line1[0].1, line1[1].1);
    let line1_max_y = max(line1[0].1, line1[1].1);

    let line2_min_x = min(line2[0].0, line2[1].0);
    let line2_max_x = max(line2[0].0, line2[1].0);
    let line2_min_y = min(line2[0].1, line2[1].1);
    let line2_max_y = max(line2[0].1, line2[1].1);

    ((line1_min_x < line2_min_x && line1_max_x > line2_max_x)
        && (line2_min_y < line1_min_y && line2_max_y > line1_max_y))
        || ((line2_min_x < line1_min_x && line2_max_x > line1_max_x)
            && (line1_min_y < line2_min_y && line1_max_y > line2_max_y))
}

fn polygon_crosses_rectangle(p1: usize, p2: usize, points: &[(u64, u64)]) -> bool {
    let corners = [
        (
            min(points[p1].0, points[p2].0),
            min(points[p1].1, points[p2].1),
        ),
        (
            max(points[p1].0, points[p2].0),
            min(points[p1].1, points[p2].1),
        ),
        (
            max(points[p1].0, points[p2].0),
            max(points[p1].1, points[p2].1),
        ),
        (
            min(points[p1].0, points[p2].0),
            max(points[p1].1, points[p2].1),
        ),
    ];

    let arcs = [
        [corners[0], corners[1]],
        [corners[1], corners[2]],
        [corners[2], corners[3]],
        [corners[3], corners[0]],
    ];

    for p in points.windows(2) {
        if p.contains(&points[p1]) || p.contains(&points[p2]) {
            continue;
        }
        for arc in arcs.iter() {
            if lines_cross(p, arc) {
                return true;
            }
        }
    }

    false
}

fn has_intrusions(p1: usize, p2: usize, points: &[(u64, u64)]) -> bool {
    points.iter().any(|p| {
        let contains_x =
            p.0 > min(points[p1].0, points[p2].0) && p.0 < max(points[p1].0, points[p2].0);
        let contains_y =
            p.1 > min(points[p1].1, points[p2].1) && p.1 < max(points[p1].1, points[p2].1);
        contains_x && contains_y
    })
}

fn main() {
    let input = std::fs::read_to_string("day09_large.txt").expect("Can't open file");

    let mut points: Vec<(u64, u64)> = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect();

    let mut part1 = 0;
    for (idx, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(idx + 1) {
            let area = (point1.0.abs_diff(point2.0) + 1) * (point1.1.abs_diff(point2.1) + 1);
            if area > part1 {
                part1 = area;
            }
        }
    }

    dbg!(part1);

    points.push(points[0]);
    let mut part2 = 0u64;

    for (idx, start_p) in points.iter().enumerate() {
        for (idx2, later_p) in points.iter().enumerate().skip(idx + 2) {
            if has_intrusions(idx, idx2, &points) || polygon_crosses_rectangle(idx, idx2, &points) {
                continue;
            }

            let area = (start_p.0.abs_diff(later_p.0) + 1) * (start_p.1.abs_diff(later_p.1) + 1);
            if area > part2 {
                part2 = area;
            }
        }
    }

    dbg!(part2);
}
