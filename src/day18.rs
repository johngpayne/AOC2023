use glam::{ivec2, IVec2};
use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day = 18))]
pub fn solve(input: &str) -> String {
    let lines = input
        .lines()
        .map(|line| {
            let mut line_split = line.trim().split(' ');
            let dir = line_split.next().unwrap().chars().next().unwrap();
            let dist = line_split.next().unwrap().parse::<i32>().unwrap();

            let hex = &line_split.next().unwrap()[2..8];
            let hex_dist = i32::from_str_radix(&hex[0..5], 16).unwrap();
            let hex_dir =
                ['R', 'D', 'L', 'U'][hex.chars().nth(5).unwrap().to_digit(10).unwrap() as usize];

            [(dir, dist), (hex_dir, hex_dist)]
        })
        .collect_vec();

    let mut lines = lines;
    lines.push(lines[0]);
    lines.push(lines[1]);

    let areas = [0, 1].map(|index| {
        let mut pos = ivec2(0, 0);
        let mut edges = vec![];
        for window in lines.windows(3) {
            let ((prev_dir, _), (dir, len), (next_dir, _)) =
                (window[0][index], window[1][index], window[2][index]);
            match dir {
                'D' => {
                    edges.push((pos + IVec2::Y, len - 1));
                    pos += len * IVec2::Y;
                }
                'U' => {
                    edges.push((pos - (len - 1) * IVec2::Y, len - 1));
                    pos -= len * IVec2::Y;
                }
                'R' => {
                    if prev_dir == 'U' {
                        edges.push((pos, 1));
                    }
                    pos += len * IVec2::X;
                    if next_dir != 'U' {
                        edges.push((pos, 1));
                    }
                }
                'L' => {
                    if prev_dir == 'D' {
                        edges.push((pos, 1));
                    }
                    pos -= len * IVec2::X;
                    if next_dir != 'D' {
                        edges.push((pos, 1));
                    }
                }
                _ => panic!(),
            }
        }
        edges.sort_by_key(|edge| edge.0.y);

        let mut tracked_edges = vec![];
        let mut edge_index = 0;
        let mut area = 0;

        for y in edges[0].0.y..=(edges[edges.len() - 1].0.y) {
            // add edges matching y
            let mut added = false;
            while edge_index < edges.len() && edges[edge_index].0.y == y {
                tracked_edges.push(edges[edge_index]);
                edge_index += 1;
                added = true;
            }
            // sort if added
            if added {
                tracked_edges.sort_by_key(|(pos, _)| pos.x);
            }
            // add area
            area += tracked_edges
                .chunks(2)
                .map(|pair| (1 + pair[1].0.x - pair[0].0.x) as usize)
                .sum::<usize>();
            // remove any redundant edges
            tracked_edges = tracked_edges
                .into_iter()
                .filter_map(|(p, n)| if n > 1 { Some((p, n - 1)) } else { None })
                .collect_vec();
        }
        area
    });

    format!("{}/{}", areas[0], areas[1])
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)",
        ),
        "62/952408144115".into(),
    )
}
