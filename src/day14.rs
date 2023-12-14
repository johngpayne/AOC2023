use glam::{ivec2, IVec2};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::hash::Hasher;

#[tracing::instrument(skip(input), fields(day = 14))]
pub fn solve(input: &str) -> String {
    let mut rocks = FxHashSet::default();
    let mut rolls = vec![];
    let mut size = IVec2::ZERO;
    input.lines().enumerate().for_each(|(y, line)| {
        size.y = size.y.max(y as i32 + 1);
        line.trim().chars().enumerate().for_each(|(x, ch)| {
            size.x = size.x.max(x as i32 + 1);
            let v = ivec2(x as i32, y as i32);
            if ch == '#' {
                rocks.insert(v);
            } else if ch == 'O' {
                rolls.push(v);
            }
        })
    });

    let mut grids: [Vec<IVec2>; 4] = (0..4)
        .map(|_| vec![IVec2::ZERO; (size.x * size.y) as usize])
        .collect_vec()
        .try_into()
        .unwrap();
    for x in 0..size.x {
        let mut write = ivec2(x, 0);
        for y in 0..size.y {
            if rocks.contains(&ivec2(x, y)) {
                write = ivec2(x, y + 1);
            }
            grids[0][(y * size.x + x) as usize] = write;
        }
        let mut write = ivec2(x, size.y - 1);
        for y in (0..size.y).rev() {
            if rocks.contains(&ivec2(x, y)) {
                write = ivec2(x, y - 1);
            }
            grids[2][(y * size.x + x) as usize] = write;
        }
    }
    for y in 0..size.y {
        let mut write = ivec2(0, y);
        for x in 0..size.x {
            if rocks.contains(&ivec2(x, y)) {
                write = ivec2(x + 1, y);
            }
            grids[1][(y * size.x + x) as usize] = write;
        }
        let mut write = ivec2(size.x - 1, y);
        for x in (0..size.x).rev() {
            if rocks.contains(&ivec2(x, y)) {
                write = ivec2(x - 1, y);
            }
            grids[3][(y * size.x + x) as usize] = write;
        }
    }

    let part_a = {
        let mut rolls = rolls.clone();
        shake(&size, &mut rolls, &grids[0], IVec2::Y);
        score(&size, &rolls)
    };

    let part_b = {
        let mut rolls = rolls.clone();
        let mut hashes = FxHashMap::<u64, usize>::default();
        let mut scores = vec![];
        let mut part_b = None;
        let mut storage = vec![0u128; size.y as usize];
        while part_b.is_none() {
            for (index, grid) in grids.iter().enumerate() {
                shake(
                    &size,
                    &mut rolls,
                    grid,
                    [IVec2::Y, IVec2::X, IVec2::NEG_Y, IVec2::NEG_X][index],
                );
            }
            let hash = hash(&mut storage, &rolls);
            if let Some(start_loop) = hashes.get(&hash) {
                part_b = Some(
                    scores[start_loop
                        + ((1_000_000_000 - 1) - start_loop) % (scores.len() - start_loop)],
                );
                break;
            }
            hashes.insert(hash, scores.len());
            scores.push(score(&size, &rolls));
        }

        part_b.unwrap()
    };

    format!("{}/{}", part_a, part_b)
}

fn shake(size: &IVec2, rolls: &mut [IVec2], grid: &[IVec2], dir: IVec2) {
    let mut used = vec![0i32; (size.x * size.y).try_into().unwrap()];
    for roll in rolls.iter_mut() {
        let nearest_rock = grid[(roll.y * size.x + roll.x) as usize];
        let used_index = (nearest_rock.y * size.x + nearest_rock.x) as usize;
        let used_entry = used.get_mut(used_index).unwrap();
        let new_roll = nearest_rock + dir * *used_entry;
        *used_entry += 1;
        *roll = new_roll;
    }
}

fn score(size: &IVec2, rolls: &[IVec2]) -> i32 {
    rolls.iter().map(|roll| size.y - roll.y).sum::<i32>()
}

fn hash(storage_for_rows: &mut [u128], rolls: &[IVec2]) -> u64 {
    for roll in rolls {
        storage_for_rows[roll.y as usize] |= 1 << (roll.x as usize);
    }
    let mut hasher = FxHasher::default();
    for row in storage_for_rows.iter_mut() {
        hasher.write_u128(*row);
        *row = 0;
    }
    hasher.finish()
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
