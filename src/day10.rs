use glam::{ivec2, IVec2};
use itertools::Itertools;
use rustc_hash::FxHashSet;

#[derive(Debug)]
struct Route {
    pos: IVec2,
    from: IVec2,
    len: usize,
}

impl Route {
    fn grow(&mut self, map: &Map) {
        *self = Route {
            pos: self.pos
                + map
                    .can_travel_from(&self.pos)
                    .unwrap()
                    .into_iter()
                    .find(|&dir| self.from != self.pos + dir)
                    .unwrap(),
            from: self.pos,
            len: self.len + 1,
        };
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<char>>,
}

const DIRS: [IVec2; 4] = [IVec2::Y, ivec2(1, 0), IVec2::NEG_Y, IVec2::NEG_X];

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map {
            tiles: value
                .lines()
                .map(|line| line.trim_start().chars().collect_vec())
                .collect_vec(),
        }
    }
}

impl Map {
    fn get(&self, pos: &IVec2) -> Option<char> {
        self.tiles
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
            .copied()
    }

    fn part_a(&self) -> usize {
        let mut routes = self.find_starts();
        let mut update_index = 0;
        loop {
            let route = routes.get_mut(update_index).unwrap();
            route.grow(self);
            if routes[update_index].pos == routes[1 - update_index].pos {
                return routes[update_index].len;
            }
            update_index = 1 - update_index;
        }
    }

    fn part_b(&self) -> usize {
        self.find_starts()
            .iter_mut()
            .find_map(|route| {
                let mut edges = vec![route.pos];
                while self.get(&route.pos) != Some('S') {
                    route.grow(self);
                    edges.push(route.pos);
                }
                edges.push(edges[0]);

                // fill edge_set from pos,
                // return true if hit edge
                fn fill(
                    map: &Map,
                    edge_set: &mut FxHashSet<IVec2>,
                    pos: &IVec2,
                    num: &mut usize,
                ) -> bool {
                    if map.get(pos).is_none() {
                        true
                    } else if edge_set.contains(pos) {
                        false
                    } else {
                        edge_set.insert(*pos);
                        *num += 1;
                        for &dir in DIRS.iter() {
                            if fill(map, edge_set, &(*pos + dir), num) {
                                return true;
                            }
                        }
                        false
                    }
                }

                let mut num = 0;
                let mut edge_set: FxHashSet<IVec2> = edges.iter().copied().collect();
                for pos_3 in edges.windows(3) {
                    for pos_2 in pos_3.windows(2) {
                        if fill(
                            self,
                            &mut edge_set,
                            &(pos_3[1] + (pos_2[1] - pos_2[0]).perp()),
                            &mut num,
                        ) {
                            return None;
                        }
                    }
                }

                Some(num)
            })
            .unwrap()
    }

    fn find_starts(&self) -> [Route; 2] {
        let pos = (|| {
            for y in 0..self.tiles.len() as i32 {
                for x in 0..self.tiles[0].len() as i32 {
                    let pos = ivec2(x, y);
                    if self.get(&pos) == Some('S') {
                        return pos;
                    }
                }
            }
            panic!();
        })();
        DIRS.iter()
            .filter_map(|&dir| {
                if let Some(dirs) = self.can_travel_from(&(pos + dir)) {
                    if dirs.contains(&(-dir)) {
                        return Some(Route {
                            pos: pos + dir,
                            from: pos,
                            len: 1,
                        });
                    }
                }
                None
            })
            .collect_vec()
            .try_into()
            .unwrap()
    }

    fn can_travel_from(&self, pos: &IVec2) -> Option<[IVec2; 2]> {
        self.get(pos)
            .map(|ch| match ch {
                '-' => Some([IVec2::X, IVec2::NEG_X]),
                '|' => Some([IVec2::Y, IVec2::NEG_Y]),
                'F' => Some([IVec2::X, IVec2::Y]),
                'L' => Some([IVec2::X, IVec2::NEG_Y]),
                '7' => Some([IVec2::NEG_X, IVec2::Y]),
                'J' => Some([IVec2::NEG_X, IVec2::NEG_Y]),
                _ => None,
            })
            .unwrap_or(None)
    }
}

#[tracing::instrument(skip(input), fields(day = 10))]
pub fn solve(input: &str) -> String {
    let map: Map = input.into();
    format!("{}/{}", map.part_a(), map.part_b())
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    let part_a = Map::from(
        "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ",
    )
    .part_a();

    let part_b = Map::from(
        "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L",
    )
    .part_b();

    (format!("{}/{}", part_a, part_b), "8/10".into())
}
