use glam::{ivec2, IVec2};
use itertools::Itertools;

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

const DIRS: [IVec2; 4] = [ivec2(0, 1), ivec2(1, 0), ivec2(0, -1), ivec2(-1, 0)];

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
    fn width(&self) -> i32 {
        self.tiles[0].len() as i32
    }
    fn height(&self) -> i32 {
        self.tiles.len() as i32
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
            .filter_map(|route| {
                let mut outer_position = vec![route.pos];
                while self.get(&route.pos) != Some('S') {
                    route.grow(self);
                    outer_position.push(route.pos);
                }
                outer_position.push(outer_position[0]);

                let mut map = Map {
                    tiles: (0..self.height())
                        .map(|y| {
                            (0..self.width())
                                .map(|x| {
                                    let pos = ivec2(x, y);
                                    if !outer_position.contains(&pos) {
                                        ' '
                                    } else {
                                        self.get(&pos).unwrap()
                                    }
                                })
                                .collect_vec()
                        })
                        .collect_vec(),
                };

                fn fill(map: &mut Map, pos: &IVec2, num: &mut usize) {
                    if map.get(pos) != Some(' ') {
                        return;
                    }
                    map.tiles[pos.y as usize][pos.x as usize] = '*';
                    *num += 1;
                    for &dir in DIRS.iter() {
                        fill(map, &(*pos + dir), num);
                    }
                }

                let mut num = 0;
                for pos_3 in outer_position.windows(3) {
                    for pos_2 in pos_3.windows(2) {
                        fill(
                            &mut map,
                            &(pos_3[1] + (pos_2[1] - pos_2[0]).perp()),
                            &mut num,
                        );
                    }
                }

                // if we got to edge we were looking the wrong way
                if map.tiles[0].iter().any(|&ch| ch == '*')
                    || map.tiles[map.tiles.len() - 1].iter().any(|&ch| ch == '*')
                    || map
                        .tiles
                        .iter()
                        .any(|row| row[0] == '*' || row[row.len() - 1] == '*')
                {
                    return None;
                }

                for y in 0..map.height() {
                    tracing::debug!("{}", map.tiles[y as usize].iter().collect::<String>());
                }
                Some(num)
            })
            .next()
            .unwrap()
    }

    fn find_starts(&self) -> [Route; 2] {
        let pos = (|| {
            for y in 0..self.height() {
                for x in 0..self.width() {
                    let pos = ivec2(x, y);
                    if self.get(&pos) == Some('S') {
                        return pos;
                    }
                }
            }
            panic!();
        })();
        DIRS.iter()
            .filter(|&&dir| {
                if let Some(dirs) = self.can_travel_from(&(pos + dir)) {
                    dirs.contains(&(-dir))
                } else {
                    false
                }
            })
            .map(|&dir| Route {
                pos: pos + dir,
                from: pos,
                len: 1,
            })
            .collect_vec()
            .try_into()
            .unwrap()
    }
    fn can_travel_from(&self, pos: &IVec2) -> Option<[IVec2; 2]> {
        self.get(pos)
            .map(|ch| match ch {
                '-' => Some([ivec2(1, 0), ivec2(-1, 0)]),
                '|' => Some([ivec2(0, 1), ivec2(0, -1)]),
                'F' => Some([ivec2(1, 0), ivec2(0, 1)]),
                'L' => Some([ivec2(1, 0), ivec2(0, -1)]),
                '7' => Some([ivec2(-1, 0), ivec2(0, 1)]),
                'J' => Some([ivec2(-1, 0), ivec2(0, -1)]),
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
