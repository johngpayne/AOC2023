use std::{collections::VecDeque, hash::Hasher};

use glam::{ivec2, IVec2};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHasher};

#[tracing::instrument(skip(input), fields(day = 21))]
pub fn solve(input: &str) -> String {
    let (map_chars, start_pos) = parse(input);
    format!(
        "{}/{}",
        Map::new(&map_chars, start_pos, 1).part_a(64),
        Map::new(&map_chars, start_pos, 2).part_b(26501365)
    )
}

struct Map {
    grid: Vec<bool>,
    size: i32,
    start_pos: IVec2,
}

fn parse(input: &str) -> (Vec<Vec<char>>, IVec2) {
    let map_chars = input
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec();
    let start_pos = map_chars
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(move |(x, ch)| {
                if *ch == 'S' {
                    Some(ivec2(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();
    (map_chars, start_pos)
}

type Starts = VecDeque<(u64, usize, IVec2, IVec2)>;
type Cache = FxHashMap<u64, Page>;
type CacheItem = <Cache as IntoIterator>::Item;

impl Map {
    fn new(map_chars: &Vec<Vec<char>>, start_pos: IVec2, mult: i32) -> Self {
        let mut size = ivec2(map_chars[0].len() as i32, map_chars.len() as i32);
        let original_size = size;
        size *= mult;
        assert!(size.x == size.y);
        let grid = (0..size.y)
            .flat_map(|y| {
                (0..size.x)
                    .map(|x| {
                        map_chars[(y % original_size.y) as usize][(x % original_size.x) as usize]
                            != '#'
                    })
                    .collect_vec()
            })
            .collect_vec();

        Map {
            grid,
            size: size.x,
            start_pos,
        }
    }

    fn inside(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.size && pos.y < self.size
    }
    fn get(&self, pos: IVec2) -> bool {
        self.inside(pos) && self.grid[(pos.y * self.size + pos.x) as usize]
    }
    fn part_a(&self, steps: usize) -> usize {
        let page = Page::new(self, &[(0, self.start_pos)], steps & 1);
        page.score(steps, steps & 1)
    }

    fn part_b(&self, steps: usize) -> usize {
        let mut starts: Starts = [(0, steps, IVec2::ZERO, IVec2::ZERO)]
            .into_iter()
            .collect::<VecDeque<_>>();

        let mut cached_pages: Cache = [(0, Page::new(self, &[(0, self.start_pos)], steps & 1))]
            .into_iter()
            .collect::<FxHashMap<_, _>>();

        let mut total_score = 0;
        while !starts.is_empty() {
            let (page_hash, page_steps, page_pos, from_dir) = starts.pop_front().unwrap();

            let page = cached_pages.get(&page_hash).unwrap();
            total_score += page.score(page_steps, steps & 1);

            let mut new_pages: Vec<CacheItem> = vec![];

            let mut expand_generic = |pos, dir, steps_left| {
                let new_pos = pos + dir;
                let (min_in_dir, border_hash, border) = page.get_border(dir);
                if steps_left >= *min_in_dir {
                    if !cached_pages.contains_key(border_hash) {
                        new_pages.push((*border_hash, Page::new(self, border, steps & 1)));
                    }
                    let new_start = (*border_hash, steps_left - min_in_dir, new_pos, dir);
                    starts.push_back(new_start);
                }
            };

            let mut expand_repeat = |mut pos, dir: IVec2, mut steps_left, total_score: &mut usize| {
                let (min_in_dir, border_hash, _) = page.get_border(dir);
                if page_hash == *border_hash {
                    while steps_left >= *min_in_dir {
                        steps_left -= min_in_dir;
                        *total_score += page.score(steps_left, steps & 1);
                        pos += dir;
                    }
                } else {
                    expand_generic(pos, dir, steps_left);
                }
            };

            if from_dir == IVec2::ZERO {
                for &dir in DIRS.iter() {
                    expand_generic(page_pos, dir, page_steps);
                }
            } else if from_dir.y != 0 {
                expand_repeat(page_pos, IVec2::X, page_steps, &mut total_score);
                expand_repeat(page_pos, -IVec2::X, page_steps, &mut total_score);
                expand_generic(page_pos, from_dir, page_steps);
            } else {
                expand_repeat(page_pos, from_dir, page_steps, &mut total_score);
            }

            cached_pages.extend(new_pages.into_iter());
        }

        total_score
    }
}

type StepAndPos = (usize, IVec2);
type Border = (usize, u64, Vec<StepAndPos>);

struct Page {
    min_steps_to_pos: Vec<Option<usize>>,
    max_steps: usize,
    score_at_max_steps: usize,
    borders: [Border; 4],
}

const DIRS: [IVec2; 4] = [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)];

impl Page {
    fn get_border(&self, dir: IVec2) -> &Border {
        &self.borders[DIRS.iter().position(|&test_dir| test_dir == dir).unwrap()]
    }
    fn score(&self, steps: usize, odd: usize) -> usize {
        if steps >= self.max_steps {
            self.score_at_max_steps
        } else {
            self.min_steps_to_pos
                .iter()
                .filter(|&&min_steps| match min_steps {
                    None => false,
                    Some(min_steps) => min_steps <= steps && (min_steps % 2) == odd,
                })
                .count()
        }
    }
    fn new(map: &Map, starts: &[StepAndPos], odd: usize) -> Self {
        let mut min_steps_to_pos = vec![None; (map.size * map.size) as usize];

        let set = |min_steps_to_pos: &mut Vec<Option<usize>>, pos: IVec2, step: usize| {
            min_steps_to_pos[(pos.y * map.size + pos.x) as usize] = Some(step);
        };
        let get = |min_steps_to_pos: &Vec<Option<usize>>, pos: IVec2| {
            min_steps_to_pos[(pos.y * map.size + pos.x) as usize]
        };

        let get_indexed_coord = |index: i32, dir: IVec2| match dir {
            IVec2 { x: -1, y: 0 } => ivec2(0, index),
            IVec2 { x: 1, y: 0 } => ivec2(map.size - 1, index),
            IVec2 { x: 0, y: -1 } => ivec2(index, 0),
            IVec2 { x: 0, y: 1 } => ivec2(index, map.size - 1),
            _ => panic!(),
        };

        let get_indexed = |min_steps_to_pos: &Vec<Option<usize>>, index: i32, dir: IVec2| {
            get(min_steps_to_pos, get_indexed_coord(index, dir))
        };

        let hash_border = |border: &[StepAndPos], dir: IVec2| {
            let mut hasher = FxHasher::default();
            hasher.write_i32(dir.x);
            hasher.write_i32(dir.y);
            for (steps, _) in border.iter() {
                hasher.write_usize(*steps);
            }
            hasher.finish()
        };

        let mut next_wave = vec![];
        for &(offset, start) in starts.iter() {
            set(&mut min_steps_to_pos, start, offset);
            next_wave.push(start);
        }

        while !next_wave.is_empty() {
            let mut prev_wave = vec![];
            std::mem::swap(&mut next_wave, &mut prev_wave);

            for prev_pos in prev_wave {
                let step = get(&min_steps_to_pos, prev_pos).unwrap() + 1;
                for &dir in DIRS.iter() {
                    let pos = prev_pos + dir;
                    if map.get(pos)
                        && match get(&min_steps_to_pos, pos) {
                            None => true,
                            Some(s) => s > step,
                        }
                    {
                        set(&mut min_steps_to_pos, pos, step);
                        next_wave.push(pos);
                    }
                }
            }
        }

        let max_steps = min_steps_to_pos
            .iter()
            .filter_map(|steps| *steps)
            .max()
            .unwrap();

        let score_at_max_steps = min_steps_to_pos
            .iter()
            .filter(|&&min_steps| match min_steps {
                None => false,
                Some(min_steps) => (min_steps % 2) == odd,
            })
            .count();

        let borders = DIRS.map(|dir| {
            let min_in_dir = (0..map.size).fold(usize::MAX, |min, index| {
                min.min(get_indexed(&min_steps_to_pos, index, dir).unwrap())
            }) & !1;
            let border = (0..map.size)
                .map(|index| {
                    (
                        1 + get_indexed(&min_steps_to_pos, index, dir).unwrap() - min_in_dir,
                        get_indexed_coord(index, -dir),
                    )
                })
                .collect_vec();
            (min_in_dir, hash_border(&border, dir), border)
        });

        Page {
            min_steps_to_pos,
            max_steps,
            score_at_max_steps,
            borders,
        }
    }
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    let (map_chars, start_pos) = parse(
        "...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........",
    );

    (
        format!(
            "{}/{}",
            Map::new(&map_chars, start_pos, 1).part_a(6),
            Map::new(&map_chars, start_pos, 2).part_b(5000)
        ),
        "16/16733044".into(),
    )
}
