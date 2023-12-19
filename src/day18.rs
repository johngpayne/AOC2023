use glam::{ivec2, IVec2};
use itertools::Itertools;
use rustc_hash::FxHashMap;

struct XAndLen {
    x: i32,
    len: i32,
}

impl XAndLen {
    fn next(&self, gap: i32) -> Option<Self> {
        if self.len > gap {
            Some(XAndLen {
                x: self.x,
                len: self.len - gap,
            })
        } else {
            None
        }
    }
}

struct PosAndLen {
    pos: IVec2,
    len: i32,
}

impl From<IVec2> for PosAndLen {
    fn from(pos: IVec2) -> Self {
        PosAndLen { pos, len: 1 }
    }
}

#[derive(Clone, Copy)]
struct DirAndLen {
    dir: IVec2,
    len: i32,
}

#[tracing::instrument(skip(input), fields(day = 18))]
pub fn solve(input: &str) -> String {
    let dirs = [
        ('R', IVec2::X),
        ('L', -IVec2::X),
        ('D', IVec2::Y),
        ('U', -IVec2::Y),
    ]
    .into_iter()
    .collect::<FxHashMap<char, IVec2>>();

    let lines = input
        .lines()
        .map(|line| {
            let mut line_split = line.trim().split(' ');
            let part_a = DirAndLen {
                dir: dirs[&line_split.next().unwrap().chars().next().unwrap()],
                len: line_split.next().unwrap().parse::<i32>().unwrap(),
            };
            let hex = &line_split.next().unwrap()[2..8];
            let part_b = DirAndLen {
                dir: [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y]
                    [hex.chars().nth(5).unwrap().to_digit(10).unwrap() as usize],
                len: i32::from_str_radix(&hex[0..5], 16).unwrap(),
            };
            [part_a, part_b]
        })
        .collect_vec();

    let mut lines = lines;
    lines.push(lines[0]);
    lines.push(lines[1]);

    let areas = [0, 1].map(|index| {
        let mut pos = ivec2(0, 0);
        let mut edges: Vec<PosAndLen> = vec![];
        for window in lines.windows(3) {
            let (prev, line, next) = (window[0][index], window[1][index], window[2][index]);
            if line.dir.y != 0 {
                edges.push(PosAndLen {
                    pos: pos + IVec2::Y * if line.dir.y > 0 { 1 } else { 1 - line.len },
                    len: line.len - 1,
                });
                pos += line.len * line.dir;
            } else {
                if prev.dir.y != line.dir.x {
                    edges.push(pos.into());
                }
                pos += line.len * line.dir;
                if next.dir.y == line.dir.x {
                    edges.push(pos.into());
                }
            }
        }
        edges.sort_by_key(|edge| edge.pos.y);

        let mut tracked_edges: Vec<XAndLen> = vec![];
        let mut edge_index = 0;
        let mut area = 0;
        let mut y = edges[0].pos.y;
        loop {
            // add edges matching y
            let mut added = false;
            while edge_index < edges.len() && edges[edge_index].pos.y == y {
                tracked_edges.push(XAndLen {
                    x: edges[edge_index].pos.x,
                    len: edges[edge_index].len,
                });
                edge_index += 1;
                added = true;
            }
            // sort if added
            if added {
                tracked_edges.sort_by_key(|x_and_len| x_and_len.x);
            }
            if tracked_edges.is_empty() {
                break;
            }
            // see if can skip
            let gap_until_next_edge = tracked_edges
                .iter()
                .map(|x_and_len| x_and_len.len)
                .min()
                .unwrap()
                .min(if edge_index < edges.len() {
                    edges[edge_index].pos.y - y
                } else {
                    1
                });
            // add area
            area += (gap_until_next_edge as usize)
                * tracked_edges
                    .chunks(2)
                    .map(|pair| (1 + pair[1].x - pair[0].x) as usize)
                    .sum::<usize>();
            // remove any redundant edges
            tracked_edges = tracked_edges
                .into_iter()
                .filter_map(|x_and_len| x_and_len.next(gap_until_next_edge))
                .collect_vec();
            y += gap_until_next_edge;
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
