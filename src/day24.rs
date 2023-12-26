use std::ops::RangeInclusive;

use glam::{I64Vec2, I64Vec3};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
struct I128Vec2 {
    x: i128,
    y: i128,
}

const fn i128vec2(x: i128, y: i128) -> I128Vec2 {
    I128Vec2 { x, y }
}

impl From<I64Vec2> for I128Vec2 {
    fn from(value: I64Vec2) -> Self {
        i128vec2(value.x as i128, value.y as i128)
    }
}

impl std::ops::Sub<I128Vec2> for I128Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl I128Vec2 {
    fn dot(&self, rhs: Self) -> i128 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct I128Vec3 {
    x: i128,
    y: i128,
    z: i128,
}

const fn i128vec3(x: i128, y: i128, z: i128) -> I128Vec3 {
    I128Vec3 { x, y, z }
}

impl From<I64Vec3> for I128Vec3 {
    fn from(value: I64Vec3) -> Self {
        i128vec3(value.x as i128, value.y as i128, value.z as i128)
    }
}

impl std::ops::Index<usize> for I128Vec3 {
    type Output = i128;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl I128Vec3 {
    fn truncate(&self) -> I128Vec2 {
        i128vec2(self.x, self.y)
    }
}

#[tracing::instrument(skip(input), fields(day = 24))]
pub fn solve(input: &str) -> String {
    let lines = get_lines(input);
    format!("{}/{}", part_a(&lines, &(200000000000000..=400000000000000)), part_b(&lines))
}

#[derive(Debug)]
struct Line<T> {
    pos: T,
    vel: T,
}

impl<T: Copy> From<[T; 2]> for Line<T> {
    fn from(value: [T; 2]) -> Self {
        Line {
            pos: value[0],
            vel: value[1],
        }
    }
}

fn get_lines(input: &str) -> Vec<Line<I128Vec3>> {
    input
        .lines()
        .map(|line| {
            let mut line_split = line.split('@');
            [0, 1]
                .map(|_| {
                    let parts = line_split
                        .next()
                        .unwrap()
                        .split(',')
                        .map(|p| p.trim().parse::<i128>().unwrap())
                        .collect_vec();
                    i128vec3(parts[0], parts[1], parts[2])
                })
                .into()
        })
        .collect_vec()
}

fn flatten(lines: &[Line<I128Vec3>]) -> Vec<Line<I128Vec2>> {
    lines
        .iter()
        .map(|line| Line {
            pos: line.pos.truncate(),
            vel: line.vel.truncate(),
        })
        .collect_vec()
}

fn intersection_2d(l1: &Line<I128Vec2>, l2: &Line<I128Vec2>, range: &RangeInclusive<i128>) -> bool {
    let div = l1.vel.x * l2.vel.y - l1.vel.y * l2.vel.x;
    if div == 0 {
        false
    } else {
        let t1 = l2.pos.x * (l2.pos.y + l2.vel.y) - l2.pos.y * (l2.pos.x + l2.vel.x);
        let t2 = l1.pos.x * (l1.pos.y + l1.vel.y) - l1.pos.y * (l1.pos.x + l1.vel.x);
        let p = i128vec2(
            (l1.vel.x * t1 - l2.vel.x * t2) / div,
            (l1.vel.y * t1 - l2.vel.y * t2) / div,
        );
        if (p - l1.pos).dot(l1.vel) <= 0 || (p - l2.pos).dot(l2.vel) <= 0 {
            return false;
        }
        range.contains(&p.x) && range.contains(&p.y)
    }
}

fn part_a(lines: &[Line<I128Vec3>], range: &RangeInclusive<i128>) -> usize {
    let lines = flatten(lines);
    let mut pairs = 0;
    for i1 in 0..lines.len() {
        for i2 in (i1 + 1)..lines.len() {
            if intersection_2d(&lines[i1], &lines[i2], range) {
                pairs += 1;
            }
        }
    }
    pairs
}

fn part_b(lines: &[Line<I128Vec3>]) -> i128 {

    /*
       Got a lot of this from: https://www.reddit.com/r/adventofcode/comments/18q40he/2023_day_24_part_2_a_straightforward_nonsolver/
       
       Our collision path X/Y/Z DX/DY/Z
       will hit all hails x/y/z dx/dy/dz
       at time t...

       therefore for each axis (x for example)

       X + t DX = x + t dx
       t = (X - x) / (dx - DX)

       taking x and y axis we can therefore say

       (X - x) / (dx - DX) = (Y - y) / (dy - DY)

       can re-arrange (to remove divides) as

       (X - x)(dy - DY) = (Y - y)(dx - DX)

       and expand out from there to

       Y DX - X DY = x dy - y dx + Y dx + y DX - x DY - X dy

       because left side is same for any hail we can now swap other x'/y'/dx'/dy' into it

       (dy'-dy) X + (dx-dx') Y + (y-y') DX + (x'-x) DY = x' dy' - y' dx' - x dy + y dx

       or

       -(dy-dy') X + (dx-dx') Y + (y-y') DX + -(x-x') DY = (y dx - x dy) - (y' dx' - x' dy')
    */

    // Based on a lua solution https://github.com/cideM/aoc2023/blob/main/d24/main.lua
    // But modified to swap row down if zero in the pivot position
    // which is required for test data but not real data...
    fn guassian_elimination(mut matrix: Vec<Vec<f64>>) -> Vec<f64> {

        let rows = matrix.len();
        let cols = matrix[0].len() - 1;

        for k in 0..rows {
            for i in (k + 1)..rows {
                if matrix[k][k] == 0.0 {
                    matrix.swap(k, k + 1);
                }
                let factor = matrix[i][k] / matrix[k][k];
                for j in k..(cols + 1) {
                    matrix[i][j] -= factor * matrix[k][j];
                }
            }
        }

        let mut solution = vec![0.0; rows];
        for i in (0..rows).rev() {
            let mut sum = 0.0;
            #[allow(clippy::needless_range_loop)]
            for j in (i + 1)..cols {
                sum += matrix[i][j] * solution[j]
            }
            solution[i] = (matrix[i][cols] - sum) / matrix[i][i];
        }

        solution
    }

    fn get_matrix(lines: &[Line<I128Vec3>], x_axis: usize, y_axis: usize) -> Vec<Vec<f64>> {
        let parts = lines
            .iter()
            .take(5)
            .map(|line| {
                [
                    -line.vel[y_axis],
                    line.vel[x_axis],
                    line.pos[y_axis],
                    -line.pos[x_axis],
                    line.pos[y_axis] * line.vel[x_axis] - line.pos[x_axis] * line.vel[y_axis],
                ]
            })
            .collect_vec();

        parts
            .iter()
            .take(4)
            .map(|part| {
                part.iter()
                    .zip(parts[4].iter())
                    .map(|(a, b)| (a - b) as f64)
                    .collect_vec()
            })
            .collect_vec()
    }

    let (x, y, _, _) = guassian_elimination(get_matrix(lines, 0, 1))
        .into_iter()
        .collect_tuple()
        .unwrap();
    let (z, _, _, _) = guassian_elimination(get_matrix(lines, 2, 1))
        .into_iter()
        .collect_tuple()
        .unwrap();
    tracing::debug!("results of elimination {}/{}/{}", x.round() as i128, y.round() as i128, z.round() as i128);
    
    (x + y + z) as i128
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    let lines = get_lines(
        "19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3",
    );

    (
        format!("{}/{}", part_a(&lines, &(7..=27)), part_b(&lines)),
        "2/47".into(),
    )
}
