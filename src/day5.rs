use std::ops::Range;
use itertools::{min, Itertools};

#[tracing::instrument(skip(input), fields(day = 5))]
pub fn solve(input: &str) -> String {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect_vec();
    tracing::debug!("nums {:?}", seeds);

    let mut transform_steps: Vec<Vec<(i64, Range<i64>)>> = vec![];
    lines.for_each(|line| {
        if line.is_empty() || line.ends_with(':') {
            if transform_steps
                .iter()
                .last()
                .map(|last_line| !last_line.is_empty())
                .unwrap_or(true)
            {
                transform_steps.push(vec![]);
            }
        } else if let Some((dest, start, count)) = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
        {
            transform_steps
                .iter_mut()
                .last()
                .unwrap()
                .push((dest - start, start..(start + count)));
        }
    });
    tracing::debug!("transform_steps {:?}", transform_steps);

    // sort the transforms within a step by start (so can go through in order)
    transform_steps.iter_mut().for_each(|transform_step| {
        transform_step.sort_by_key(|(_, r)| r.start);
    });

    let mut nums = seeds.clone();
    transform_steps.iter().for_each(|transforms| {
        nums.iter_mut().for_each(|num| {
            if let Some((offset, _)) = transforms.iter().find(|(_, range)| range.contains(num)) {
                *num += offset;
            }
        });
    });
    tracing::debug!("transformed nums {:?}", nums);
    let part_a = min(nums).unwrap();

    let mut seed_ranges = seeds.chunks(2).map(|r| r[0]..(r[0] + r[1])).collect_vec();
    tracing::debug!("ranges {:?}", seed_ranges);

    for transform_step in transform_steps {
        let mut next_seed_ranges = vec![];
        for mut seed_range in seed_ranges {
            for (offset, transform_range) in transform_step.iter() {
                // any seeds to left of this transform range add unchanged
                // and crop the seed range to the transform start
                if seed_range.start < transform_range.start {
                    let unchanged_range = seed_range.start..transform_range.start.min(seed_range.end);
                    seed_range.start = unchanged_range.end;
                    next_seed_ranges.push(unchanged_range);
                }
                // any intersect with the transformed range, crop the seed range 
                // to the end of the transform range
                let intersect_start = seed_range.start.max(transform_range.start);
                let intersect_end = seed_range.end.min(transform_range.end);
                if intersect_end > intersect_start {
                    seed_range.start = intersect_end;
                    next_seed_ranges.push((intersect_start + offset)..(intersect_end + offset));
                }
                // if nothing left then finish this transform step
                if seed_range.start == seed_range.end {
                    break;
                }
            }
            // if anything was left then add it unchanged
            if seed_range.end > seed_range.start {
                next_seed_ranges.push(seed_range);
            }
        }
        seed_ranges = next_seed_ranges;
    }
    let part_b = min(seed_ranges.iter().map(|r| r.start)).unwrap();

    format!("{}/{}", part_a, part_b)
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4",
        ),
        "35/46".into(),
    )
}
