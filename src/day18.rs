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
            let dist2 = i32::from_str_radix(&hex[0..5], 16).unwrap();
            let dir2 =
                ['R', 'D', 'L', 'U'][hex.chars().nth(5).unwrap().to_digit(10).unwrap() as usize];

            [(dir, dist), (dir2, dist2)]
        })
        .collect_vec();

    let mut lines = lines;
    lines.push(lines[0]);
    lines.push(lines[1]);

    let area = [0, 1].map(|index| {
        let mut pos = ivec2(0, 0);
        let mut edges: Vec<IVec2> = vec![];

        for window in lines.windows(3) {
            let ((prev_dir, _), (dir, len), (next_dir, _)) =
                (window[0][index], window[1][index], window[2][index]);
            match dir {
                'D' | 'U' => {
                    let dir_scale = if dir == 'D' { 1 } else { -1 };
                    (1..len).for_each(|index| edges.push(pos + dir_scale * index * IVec2::Y));
                    pos += dir_scale * len * IVec2::Y;
                }
                'R' | 'L' => {
                    let (match_vert, dir_scale) = if dir == 'R' { ('U', 1) } else { ('D', -1) };
                    if prev_dir == match_vert {
                        edges.push(pos);
                    }
                    pos += dir_scale * len * IVec2::X;
                    if next_dir != match_vert {
                        edges.push(pos);
                    }
                }
                _ => panic!(),
            }
        }
        let t = std::time::Instant::now();
        edges.sort_by(|e1, e2| e1.y.cmp(&e2.y).then(e1.x.cmp(&e2.x)));
        let d = std::time::Instant::now() - t;
        tracing::debug!("sort took {}ms", d.as_millis());

        let mut area = 0;
        for edge_pair in edges.chunks(2) {
            area += (1 + edge_pair[1].x - edge_pair[0].x) as usize
        }
        area
    });

    format!("{}/{}", area[0], area[1])
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
