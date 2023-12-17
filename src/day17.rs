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

    let mut routes = Routes::default();
    
    let mut i = 0;
    while routes.expand_routes(&map, usize::MAX) {
        tracing::debug!("expand {} {}", i, routes.routes.len());
        i += 1;
    }

    format!("{}", routes.find_best(map.target()))
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
    fn has_better_route(&self, cost: u32, pos: IVec2, prev_dir: IVec2, prev_dir_length: u32) -> bool {
        if let Some(&best_cost) = self.best_costs.get(&(pos, prev_dir, prev_dir_length)) {
            if best_cost <= cost {
                return true;
            }
        }
        if prev_dir_length == 1 {
            false
        } else {
            self.has_better_route(cost, pos, prev_dir, prev_dir_length - 1)
        }
    }
    fn add(&mut self, route: Route, target: IVec2) { 
        if self.has_better_route(route.cost, route.pos, route.prev_dir, route.prev_dir_length) {
            return;
        }
        self.best_costs.insert((route.pos, route.prev_dir, route.prev_dir_length), route.cost);
        self.routes.push(route);
        self.routes
            .sort_by_key(|route| route.cost + route.max_cost(target));
    }
    fn find_best(&self, target: IVec2) -> u32 {
        self.best_costs.iter().filter(|((pos, _, _), _)| *pos == target).map(|(_, cost)| *cost).min().unwrap()
    }
    fn expand_routes(&mut self, map: &Map, max_take: usize) -> bool {
        let mut prev_routes = Vec::default();
        std::mem::swap(&mut prev_routes, &mut self.routes);
        for route in prev_routes.into_iter().take(max_take) {
            if route.pos != map.target() {
                const DIRS: [IVec2; 4] = [ivec2(1, 0), ivec2(0, 1), ivec2(-1, 0), ivec2(0, -1)];
                for dir in DIRS {
                    if route.can_go(dir) {
                        if let Some(cost) = map.get(route.pos + dir) {
                            self.add(route.expand(dir, cost), map.target());
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
    fn target(&self) -> IVec2 {
        ivec2(self.0[0].len() as i32 - 1, self.0.len() as i32 - 1)
    }
}

#[derive(Debug, Default, Clone)]
struct Route {
    pos: IVec2,
    cost: u32,
    prev_dir: IVec2,
    prev_dir_length: u32,
}

impl Route {
    fn can_go(&self, dir: IVec2) -> bool {
        self.prev_dir != -dir && (self.prev_dir != dir || self.prev_dir_length < 3)
    }
    fn expand(&self, dir: IVec2, cost: u32) -> Self {
        Route {
            pos: self.pos + dir,
            cost: self.cost + cost,
            prev_dir: dir,
            prev_dir_length: if self.prev_dir == dir {
                self.prev_dir_length + 1
            } else {
                1
            },
        }
    }
    fn max_cost(&self, target: IVec2) -> u32 {
        5 * ((target.x - self.pos.x).abs() + (target.y - self.pos.y).abs()) as u32
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
        "102".into(),
    )
}
