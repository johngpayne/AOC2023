use std::ops::RangeInclusive;

use glam::{ivec2, IVec2};
use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day = 17))]
pub fn solve(input: &str) -> String {
    let map = Map(input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec());
    let target = ivec2(map.0[0].len() as i32 - 1, map.0.len() as i32 - 1);

    format!("{}/{}", run(&map, target, 1..=3), run(&map, target, 4..=10))
}

fn run(map: &Map, target: IVec2, range: RangeInclusive<u32>) -> u32 {
    let mut routes = Routes::new(map);
    while routes.expand_routes(map, target, &range) {}
    routes.best_costs
            .iter()
            .map(|best_costs| best_costs[target.y as usize][target.x as usize])
            .min()
            .unwrap()
}

#[derive(Debug, Clone)]
struct Route {
    pos: IVec2,
    cost: u32,
    index: usize,
}

impl Default for Route {
    fn default() -> Self {
        Route {
            pos: IVec2::ZERO,
            cost: 0,
            index: usize::MAX,
        }
    }
}

#[derive(Debug)]
struct Routes {
    routes: Vec<Route>,
    best_costs: [Vec<Vec<u32>>; 4],
}

const DIRS: [IVec2; 4] = [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)];

impl Routes {
    fn new(map: &Map) -> Self {
        Routes {
            routes: [Route::default()].into_iter().collect(),
            best_costs: (0..4)
                .map(|_| vec![vec![u32::MAX; map.0[0].len()]; map.0.len()])
                .collect_vec()
                .try_into()
                .unwrap(),
        }
    }
    fn add_route(&mut self, route: &Route) {
        let best_cost =
            &mut self.best_costs[route.index][route.pos.y as usize][route.pos.x as usize];
        if *best_cost > route.cost {
            *best_cost = route.cost;
            self.routes.push(route.clone());
        }
    }
    fn expand_routes(&mut self, map: &Map, target: IVec2, range: &RangeInclusive<u32>) -> bool {
        let mut prev_routes = Vec::default();
        std::mem::swap(&mut prev_routes, &mut self.routes);
        for route in prev_routes {
            if route.index == usize::MAX {
                (0..4).for_each(|index| {
                    self.move_along_route(&route, index, map, range);
                });
            } else if route.pos != target {
                [
                    (route.index + 1) % 4,
                    (route.index + 3) % 4,
                ]
                .into_iter()
                .for_each(|index| {
                    self.move_along_route(&route, index, map, range);
                });
            }
        }
        !self.routes.is_empty()
    }
    fn move_along_route(&mut self, route: &Route, index: usize, map: &Map, range: &RangeInclusive<u32>) {
        let mut expanded_route = Route {
            index,
            ..route.clone()
        };
        for dist in 1..=*range.end() {
            if let Some(cost) = map.get(expanded_route.pos + DIRS[index]) {
                expanded_route.pos += DIRS[index];
                expanded_route.cost += cost;
                if dist >= *range.start() {
                    self.add_route(&expanded_route);
                }
            } else {
                return;
            }
        }
    }
}

struct Map(Vec<Vec<u32>>);

impl Map {
    fn get(&self, pos: IVec2) -> Option<&u32> {
        self.0
            .get(pos.y as usize)
            .map(|row| row.get(pos.x as usize))
            .unwrap_or_default()
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
