use glam::{ivec2, IVec2};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHasher};
use std::{cmp::Ordering, hash::Hasher};

#[tracing::instrument(skip(input), fields(day = 14))]
pub fn solve(input: &str) -> String {
    let mut rocks = vec![];
    let mut rolls = vec![];
    let mut size = IVec2::ZERO;
    input.lines().enumerate().for_each(|(y, line)| {
        size.y = size.y.max(y as i32 + 1);
        line.trim().chars().enumerate().for_each(|(x, ch)| {
            size.x = size.x.max(x as i32 + 1);
            let v = ivec2(x as i32, y as i32);
            if ch == '#' {
                rocks.push(v);
            } else if ch == 'O' {
                rolls.push(v);
            }
        })
    });

    format!(
        "{}/{}",
        part_a(&size, &rocks, &rolls),
        part_b(&size, &rocks, &rolls)
    )
}

fn part_a(size: &IVec2, rocks: &[IVec2], rolls: &[IVec2]) -> i32 {
    let mut rocks = rocks.to_vec();
    sort(&mut rocks, 1, -1);
    let mut rolls = rolls.to_vec();
    shake(size, &rocks, &mut rolls, 1, -1);
    score(size, &rolls)
}

fn part_b(size: &IVec2, rocks: &[IVec2], rolls: &[IVec2]) -> i32 {
    let per_dir = (0..4)
        .map(|dir_index| {
            let mut rocks = rocks.to_vec();
            let dom_axis = 1 - (dir_index % 2);
            let dir = if dir_index < 2 { -1 } else { 1 };
            sort(&mut rocks, dom_axis, dir);
            (rocks, dom_axis, dir)
        })
        .collect_vec();

    let mut rolls = rolls.to_vec();
    let mut hashes = FxHashMap::<u64, usize>::default();
    let mut scores = vec![];
    loop {
        per_dir.iter().for_each(|(rocks, dom_axis, dir)| {
            shake(size, rocks, &mut rolls, *dom_axis, *dir);
        });
        let hash = hash(&rolls);
        if let Some(start_loop) = hashes.get(&hash) {
            return scores
                [start_loop + ((1_000_000_000 - 1) - start_loop) % (scores.len() - start_loop)];
        }

        hashes.insert(hash, scores.len());
        scores.push(score(size, &rolls));
    }
}

fn score(size: &IVec2, rolls: &[IVec2]) -> i32 {
    rolls.iter().map(|roll| size.y - roll.y).sum::<i32>()
}

fn hash(rolls: &[IVec2]) -> u64 {
    let mut hasher = FxHasher::default();
    rolls.iter().for_each(|roll| {
        hasher.write_u8(roll.x as u8);
        hasher.write_u8(roll.y as u8);
    });
    hasher.finish()
}

fn shake(size: &IVec2, rocks: &[IVec2], rolls: &mut [IVec2], dom_axis: usize, dir: i32) {
    sort(rolls, dom_axis, dir);
    let reset_pos = if dir < 0 { 0 } else { size[dom_axis] - 1 };
    let mut rock_index = 0;
    let mut roll_index = 0;
    let mut pos = ivec2(-1, -1);
    while roll_index < rolls.len() {
        let next_rock = rocks.get(rock_index);
        let roll = rolls.get_mut(roll_index).unwrap();
        if roll[1 - dom_axis] == pos[1 - dom_axis]
            && match next_rock {
                None => true,
                Some(next_rock) => {
                    next_rock[1 - dom_axis] > pos[1 - dom_axis]
                        || dir * next_rock[dom_axis] < dir * roll[dom_axis]
                }
            }
        {
            *roll = pos;
            pos[dom_axis] -= dir;
            roll_index += 1;
        } else if match next_rock {
            None => false,
            Some(next_rock) => next_rock[1 - dom_axis] == pos[1 - dom_axis],
        } {
            pos[dom_axis] = next_rock.unwrap()[dom_axis] - dir;
            rock_index += 1;
        } else {
            pos[1 - dom_axis] += 1;
            pos[dom_axis] = reset_pos;
        }
    }
}

fn sort(data: &mut [IVec2], dom_axis: usize, dir: i32) {
    data.sort_by(|i1, i2| {
        let cmp_non_dom_axis = i1[1 - dom_axis].partial_cmp(&i2[1 - dom_axis]).unwrap();
        if cmp_non_dom_axis != Ordering::Equal {
            cmp_non_dom_axis
        } else {
            (i2[dom_axis] * dir)
                .partial_cmp(&(i1[dom_axis] * dir))
                .unwrap()
        }
    });
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....",
        ),
        "136/64".into(),
    )
}
