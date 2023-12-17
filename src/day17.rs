use glam::{ivec2, IVec2};
use itertools::Itertools;
use std::collections::VecDeque;

#[tracing::instrument(skip(input), fields(day = 17))]
pub fn solve(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
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

fn expand_routes<const MIN: u32, const MAX: u32>(map: &Map) -> u32 {
    let mut routes = [Route {
        pos: IVec2::ZERO,
        cost: 0,
        index: usize::MAX,
    }]
    .into_iter()
    .collect::<VecDeque<_>>();
    let mut best_costs = [0, 1].map(|_| vec![u32::MAX; (map.size.x * map.size.y) as usize]);
    best_costs[1][0] = 0;

    let target = map.size - IVec2::ONE;

    while !routes.is_empty() {
        let route = routes.pop_front().unwrap();
        if route.pos != target {
            let best_cost =
                best_costs[route.index & 1][(route.pos.y * map.size.x + route.pos.x) as usize];
            if route.cost == best_cost {
                if route.index == usize::MAX {
                    [0, 1]
                } else {
                    [(route.index + 1) % 4, (route.index + 3) % 4]
                }
                .into_iter()
                .for_each(|index| {
                    let mut route = Route { index, ..route };
                    for dist in 1..=MAX {
                        route.pos += DIRS[index];
                        if let Some(cost) = map.get(route.pos) {
                            route.cost += cost;
                            if dist >= MIN {
                                let best_cost = &mut best_costs[route.index & 1]
                                    [(route.pos.y * map.size.x + route.pos.x) as usize];
                                if route.cost < *best_cost {
                                    *best_cost = route.cost;
                                    routes.push_back(route);
                                }
                            }
                        } else {
                            break;
                        }
                    }
                });
            }
        }
    }
    best_costs
        .iter()
        .map(|best_costs| best_costs[(target.y * map.size.x + target.x) as usize])
        .min()
        .unwrap()
}

#[derive(Debug, Copy, Clone)]
struct Route {
    pos: IVec2,
    cost: u32,
    index: usize,
}

const DIRS: [IVec2; 4] = [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)];

struct Map {
    grid: Vec<Vec<u32>>,
    size: IVec2,
}

impl Map {
    fn get(&self, pos: IVec2) -> Option<u32> {
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
