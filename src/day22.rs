use glam::{ivec3, IVec3};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

struct Shape {
    pos: IVec3,
    size: IVec3,
}

impl From<(IVec3, IVec3)> for Shape {
    fn from(value: (IVec3, IVec3)) -> Self {
        Shape {
            pos: value.0,
            size: IVec3::ONE + value.1 - value.0,
        }
    }
}

impl Shape {
    fn for_each<F: FnMut(IVec3)>(&self, mut f: F) {
        for z in 0..self.size.z {
            self.for_each_flat(|pos| f(pos + ivec3(0, 0, z)));
        }
    }
    fn for_each_flat<F: FnMut(IVec3)>(&self, mut f: F) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                f(self.pos + ivec3(x, y, 0));
            }
        }
    }
    fn all<F: Fn(IVec3) -> bool>(&self, f: F) -> bool {
        for z in 0..self.size.z {
            for y in 0..self.size.y {
                for x in 0..self.size.x {
                    if !f(self.pos + ivec3(x, y, z)) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[tracing::instrument(skip(input), fields(day = 22))]
pub fn solve(input: &str) -> String {
    let mut shapes = input
        .lines()
        .map(|line| {
            let coords = line
                .trim()
                .split('~')
                .map(|coord_str| {
                    let xyz = coord_str
                        .split(',')
                        .map(|coord_part_str| coord_part_str.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32, i32)>()
                        .unwrap();
                    ivec3(xyz.0, xyz.1, xyz.2)
                })
                .collect_tuple::<(IVec3, IVec3)>()
                .unwrap();
            Shape::from(coords)
        })
        .collect_vec();

    shapes.sort_by_key(|shape| shape.pos.z);

    let mut grid = FxHashMap::<IVec3, usize>::default();
    for (index, shape) in shapes.iter().enumerate() {
        shape.for_each(|pos| {
            grid.insert(pos, index);
        });
    }

    for (index, shape) in shapes.iter_mut().enumerate() {
        while shape.pos.z > 1
            && shape.all(|pos| match grid.get(&(pos - IVec3::Z)) {
                None => true,
                Some(found_index) => *found_index == index,
            })
        {
            shape.for_each_flat(|pos| {
                grid.remove(&(pos + ivec3(0, 0, shape.size.z - 1)));
            });
            shape.pos -= IVec3::Z;
            shape.for_each_flat(|pos| {
                grid.insert(pos, index);
            });
        }
    }

    let aboves = shapes
        .iter()
        .map(|shape| {
            let mut above = FxHashSet::<usize>::default();
            shape.for_each_flat(|pos| {
                if let Some(&index) = grid.get(&(pos + ivec3(0, 0, shape.size.z))) {
                    above.insert(index);
                }
            });
            above
        })
        .collect_vec();

    let belows = shapes
        .iter()
        .map(|shape| {
            let mut below = FxHashSet::<usize>::default();
            shape.for_each_flat(|pos| {
                if let Some(&index) = grid.get(&(pos - IVec3::Z)) {
                    below.insert(index);
                }
            });
            below
        })
        .collect_vec();

    let part_a = (0..shapes.len())
        .filter(|&index| {
            aboves[index].is_empty() || aboves[index].iter().all(|&above| belows[above].len() > 1)
        })
        .count();

    fn get_fallers(
        fallers: &mut FxHashSet<usize>,
        index: usize,
        aboves: &Vec<FxHashSet<usize>>,
        belows: &Vec<FxHashSet<usize>>,
    ) {
        fallers.insert(index);
        let above = aboves[index]
            .iter()
            .filter(|&&above| belows[above].iter().all(|below| fallers.contains(below)))
            .copied()
            .collect_vec();
        for faller_index in above {
            get_fallers(fallers, faller_index, aboves, belows);
        }
    }

    let part_b = (0..shapes.len())
        .map(|index| {
            let mut fallers = FxHashSet::default();
            get_fallers(&mut fallers, index, &aboves, &belows);
            fallers.len() - 1
        })
        .sum::<usize>();

    format!("{}/{}", part_a, part_b)
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9",
        ),
        "5/7".into(),
    )
}
