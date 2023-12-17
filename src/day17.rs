use std::ops::RangeInclusive;

use glam::{ivec2, IVec2};
use itertools::Itertools;
use rustc_hash::FxHashMap;

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
    let mut routes = Routes::default();
    while routes.expand_routes(map, target, range.clone()) {
    }
    routes.find_best(target)
}

#[derive(Debug)]
struct Routes {
    routes: Vec<Route>,
    best_costs: FxHashMap<(IVec2, IVec2, u32), u32>,
}

impl Default for Routes {
    fn default() -> Self {
        Routes {
            routes: vec![Route::default()],
            best_costs: [((IVec2::ZERO, IVec2::ZERO, 0), 0)].into_iter().collect(),
        }
    }
}

impl Routes {
    fn has_better_route(&self, cost: u32, pos: IVec2, prev_dir: IVec2, dist: u32, min_dist: u32) -> bool {
        if let Some(&best_cost) = self.best_costs.get(&(pos, prev_dir, dist)) {
            if best_cost <= cost {
                return true;
            }
        }
        if dist == min_dist {
            false
        } else {
            self.has_better_route(cost, pos, prev_dir, dist - 1, min_dist)
        }
    }
    fn add(&mut self, route: &Route, dist: u32, min_dist: u32) {
        if self.has_better_route(route.cost, route.pos, route.prev_dir, dist, min_dist) {
            return;
        }
        self.best_costs
            .insert((route.pos, route.prev_dir, dist), route.cost);
        self.routes.push(route.clone());
    }
    fn find_best(&self, target: IVec2) -> u32 {
        self.best_costs
            .iter()
            .filter(|((pos, _, _), _)| *pos == target)
            .map(|(_, cost)| *cost)
            .min()
            .unwrap()
    }
    fn expand_routes(&mut self, map: &Map, target: IVec2, range: RangeInclusive<u32>) -> bool {
        let mut prev_routes = Vec::default();
        std::mem::swap(&mut prev_routes, &mut self.routes);
        for route in prev_routes {
            if route.pos != target {
                const DIRS: [IVec2; 4] = [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)];
                for dir in DIRS {
                    if route.prev_dir != -dir && route.prev_dir != dir {
                        let mut expanded_route = route.clone();
                        for dist in 1..=*range.end() {
                            if let Some(cost) = map.get(expanded_route.pos + dir) {
                                expanded_route.pos += dir;
                                expanded_route.cost += cost;
                                expanded_route.prev_dir = dir;
                                if dist >= *range.start() {
                                    self.add(&expanded_route, dist, *range.start());
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        !self.routes.is_empty()
    }
}

struct Map(Vec<Vec<u32>>);

impl Map {
    fn get(&self, pos: IVec2) -> Option<u32> {
        self.0
            .get(pos.y as usize)
            .map(|row| row.get(pos.x as usize).copied())
            .unwrap_or_default()
    }
}

#[derive(Debug, Default, Clone)]
struct Route {
    pos: IVec2,
    cost: u32,
    prev_dir: IVec2,
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
