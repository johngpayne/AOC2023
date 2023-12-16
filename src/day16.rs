use glam::{ivec2, IVec2};
use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day = 16))]
pub fn solve(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec();
    tracing::debug!("{:?}", map);

    for row in map.iter() {
        tracing::debug!("{}", row.iter().collect::<String>());
    }

    let part_a = send_beam(&map, IVec2::ZERO, 0);

    let part_b = part_b(&map);

    format!("{}/{}", part_a, part_b)
}

fn part_b(map: &[Vec<char>]) -> usize {
    let mut max_count = 0;
    for y in 0..map.len() as i32 {
        max_count = max_count.max(send_beam(map, ivec2(0, y), 0));
        max_count = max_count.max(send_beam(map, ivec2(map[0].len() as i32 - 1, y), 2));
    }
    for x in 0..map[0].len() as i32 {
        max_count = max_count.max(send_beam(map, ivec2(x, 0), 1));
        max_count = max_count.max(send_beam(map, ivec2(x, map.len() as i32 - 1), 3));
    }
    max_count
}

fn send_beam(map: &[Vec<char>], pos: IVec2, dir_index: u8) -> usize {
    const DIRS: [IVec2; 4] = [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)];

    let size = ivec2(map[0].len() as i32, map.len() as i32);
    let mut visited_grid = vec![0u8; (size.x * size.y) as usize];
    let mut beams = vec![(pos, dir_index)];
    while !beams.is_empty() {
        let mut next_beams = Vec::with_capacity(beams.len() * 2);
        for (pos, dir_index) in beams.into_iter() {
            if let Some(&ch) = map
                .get(pos.y as usize)
                .map(|row| row.get(pos.x as usize))
                .unwrap_or(None)
            {
                if visited_grid[(pos.y * size.x + pos.x) as usize] & (1 << dir_index) == 0 {
                    visited_grid[(pos.y * size.x + pos.x) as usize] |= 1 << dir_index;
                    let mut add = |new_dir_index| {
                        next_beams.push((pos + DIRS[new_dir_index as usize], new_dir_index));
                    };
                    if ch == '|' && (dir_index & 1) == 0 {
                        add(1u8);
                        add(3u8);
                    } else if ch == '-' && (dir_index & 1) == 1 {
                        add(0u8);
                        add(2u8);
                    } else if ch == '/' {
                        add([3, 2, 1, 0][dir_index as usize]);
                    } else if ch == '\\' {
                        add([1, 0, 3, 2][dir_index as usize]);
                    } else {
                        add(dir_index);
                    }
                }
            }
        }
        beams = next_beams;
    }
    visited_grid.iter().filter(|&&v| v > 0).count()
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....",
        ),
        "46/51".into(),
    )
}
