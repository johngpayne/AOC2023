use std::collections::VecDeque;

use glam::{ivec2, IVec2};
use itertools::Itertools;
use rustc_hash::FxHashMap;

const DIRS: [IVec2; 4] = [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)];

type NodeEntry = (IVec2, usize);
type Node = [Option<NodeEntry>; 4];

fn inverse_dir(dir_index: u8) -> u8 {
    (dir_index + 2) % 4
}

struct Map {
    grid: Vec<u8>,
    size: IVec2,
    start: IVec2,
    end: IVec2,
}

impl Map {
    fn new(value: &Vec<Vec<char>>, ignore_arrows: bool) -> Self {
        let size = ivec2(value[0].len() as i32, value.len() as i32);
        let get_grid_ch = |pos: IVec2| value[pos.y as usize][pos.x as usize];
        let get_valid_dirs = |pos: IVec2, size: IVec2| -> u8 {
            let ch = get_grid_ch(pos);
            if ch == '#' {
                0
            } else if pos.y == 0 {
                1 << 1
            } else if pos.y == size.y - 1 {
                1 << 3
            } else if ch == '.' || ignore_arrows {
                let mut ret = 0;
                for (dir_index, dir) in DIRS.iter().enumerate() {
                    if get_grid_ch(pos + *dir) != '#' {
                        ret |= 1 << dir_index;
                    }
                }
                ret
            } else {
                for (dir_index, test_ch) in ['>', 'v', '<', '^'].into_iter().enumerate() {
                    if ch == test_ch {
                        return 1 << dir_index;
                    }
                }
                panic!();
            }
        };

        let grid = (0..size.y)
            .flat_map(|y| {
                (0..size.x)
                    .map(|x| get_valid_dirs(ivec2(x, y), size))
                    .collect_vec()
            })
            .collect_vec();

        Map {
            size,
            start: ivec2(
                grid.iter()
                    .take(size.x as usize)
                    .position(|n| *n != 0)
                    .unwrap() as i32,
                0,
            ),
            end: ivec2(
                grid.iter()
                    .skip(((size.y - 1) * size.x) as usize)
                    .position(|n| *n != 0)
                    .unwrap() as i32,
                size.y - 1,
            ),
            grid,
        }
    }
}

impl Map {
    fn get(&self, pos: IVec2) -> u8 {
        self.grid[(pos.y * self.size.x + pos.x) as usize]
    }
    fn can_move(&self, pos: IVec2, dir_index: u8) -> bool {
        (self.get(pos) & (1 << dir_index)) != 0
    }

    fn longest_route(&self) -> usize {
        // get pos -> [maybe (to_pos, len)]
        let mut nodes = FxHashMap::<IVec2, Node>::default();
        let mut new_positions: Vec<IVec2> = vec![self.start];
        while !new_positions.is_empty() {
            let mut old_positions = vec![];
            std::mem::swap(&mut new_positions, &mut old_positions);

            for pos in old_positions {
                nodes.entry(pos).or_insert_with(|| {
                    let node = [0, 1, 2, 3].map(|dir_index| {
                        if !self.can_move(pos, dir_index) {
                            None
                        } else {
                            let mut current_index = dir_index;
                            let mut pos = pos;
                            let mut len = 0;
                            let mut ret = None;
                            loop {
                                pos += DIRS[current_index as usize];
                                len += 1;
                                if pos == self.end
                                    || pos == self.start
                                    || self.get(pos).count_ones() > 2
                                {
                                    ret = Some((pos, len));
                                    break;
                                }
                                if self.get(pos).count_ones() == 1
                                    && !self.can_move(pos, current_index)
                                {
                                    break;
                                }
                                current_index = (0..4)
                                    .find(|&test_index| {
                                        inverse_dir(test_index) != current_index
                                            && self.can_move(pos, test_index)
                                    })
                                    .unwrap();
                            }
                            ret
                        }
                    });
                    for to_pos in node.iter().filter_map(|maybe| maybe.map(|(pos, _)| pos)) {
                        new_positions.push(to_pos);
                    }
                    node
                });
            }
        }
        assert!(nodes.len() < 64);

        // get pos -> index in nodes map
        let pos_to_index = nodes
            .iter()
            .enumerate()
            .map(|(index, (pos, _))| (*pos, index as u8))
            .collect::<FxHashMap<_, _>>();

        // get [[maybe (index, len)>]]
        let index_nodes = nodes
            .values()
            .map(|node_entries| {
                node_entries.map(|entry| {
                    entry.map(|(to_pos, len)| (pos_to_index.get(&to_pos).unwrap(), len))
                })
            })
            .collect_vec();

        let end_index = *pos_to_index.get(&self.end).unwrap();
        let mut wave = VecDeque::from([(*pos_to_index.get(&self.start).unwrap(), 0usize, 0u64)]);
        let mut results = vec![];
        while !wave.is_empty() {
            let (index, len, mut visited) = wave.pop_front().unwrap();

            if index == end_index {
                results.push(len);
            } else {
                visited |= 1 << index;
                for &(to_index, to_len) in index_nodes[index as usize].iter().flatten() {
                    if (visited & 1 << to_index) == 0 {
                        wave.push_back((*to_index, len + to_len, visited));
                    }
                }
            }
        }
        results.into_iter().max().unwrap()
    }
}

#[tracing::instrument(skip(input), fields(day = 23))]
pub fn solve(input: &str) -> String {
    let grid_ch = input
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec();
    format!(
        "{}/{}",
        Map::new(&grid_ch, false).longest_route(),
        Map::new(&grid_ch, true).longest_route()
    )
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "#.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#",
        ),
        "94/154".into(),
    )
}
