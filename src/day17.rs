use glam::{ivec2, IVec2};
use itertools::Itertools;
use std::collections::BinaryHeap;

#[tracing::instrument(skip(input), fields(day = 17))]
pub fn solve(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec();
    let size = ivec2(grid[0].len() as i32, grid.len() as i32);
    let map = Map { grid, size };

    format!(
        "{}/{}",
        expand_routes::<1, 3>(&map),
        expand_routes::<4, 10>(&map)
    )
}

#[derive(Debug, Copy, Clone)]
struct Route {
    pos: IVec2,
    cost: u32,
    index: u8,
    score: u32,
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for Route {}
impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score).map(|p| p.reverse())
    }
}
impl Ord for Route {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

fn expand_routes<const MIN: u32, const MAX: u32>(map: &Map) -> u32 {
    let target = map.size - IVec2::ONE;
    let score =
        |cost: u32, pos: IVec2| cost + ((pos.x - target.x).abs() + (pos.y - target.y).abs()) as u32;

    let mut routes = [Route {
        pos: IVec2::ZERO,
        cost: 0,
        index: u8::MAX,
        score: score(0, IVec2::ZERO),
    }]
    .into_iter()
    .collect::<BinaryHeap<_>>();

    let mut best_costs = [0, 1].map(|_| vec![u32::MAX; (map.size.x * map.size.y) as usize]);
    best_costs[1][0] = 0;

    while !routes.is_empty() {
        let route = routes.pop().unwrap();
        if route.pos == target {
            break;
        }
        let best_cost = best_costs[(route.index & 1) as usize]
            [(route.pos.y * map.size.x + route.pos.x) as usize];
        if route.cost == best_cost {
            if route.index == u8::MAX {
                [0, 1]
            } else {
                [(route.index + 1) % 4, (route.index + 3) % 4]
            }
            .into_iter()
            .for_each(|index| {
                let mut route = Route { index, ..route };
                for dist in 1..=MAX {
                    const DIRS: [IVec2; 4] = [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)];
                    route.pos += DIRS[index as usize];
                    if let Some(cost) = map.get(route.pos) {
                        route.cost += cost as u32;
                        if dist >= MIN {
                            let best_cost = &mut best_costs[(route.index & 1) as usize]
                                [(route.pos.y * map.size.x + route.pos.x) as usize];
                            if route.cost < *best_cost {
                                *best_cost = route.cost;
                                route.score = score(route.cost, route.pos);
                                routes.push(route);
                            }
                        }
                    } else {
                        break;
                    }
                }
            });
        }
    }
    best_costs
        .iter()
        .map(|best_costs| best_costs[(target.y * map.size.x + target.x) as usize])
        .min()
        .unwrap()
}

struct Map {
    grid: Vec<Vec<u8>>,
    size: IVec2,
}

impl Map {
    fn get(&self, pos: IVec2) -> Option<u8> {
        if pos.x >= 0 && pos.y >= 0 && pos.x < self.size.x && pos.y < self.size.y {
            Some(self.grid[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533",
        ),
        "102/94".into(),
    )
}
