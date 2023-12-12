use itertools::Itertools;
use rustc_hash::FxHashMap;

#[tracing::instrument(skip(input), fields(day = 12))]
pub fn solve(input: &str) -> String {
    let (part_a, part_b) = input.lines().fold((0, 0), |agg, line| {
        let mut split_space = line.trim().split(' ');
        let chars = split_space
            .next()
            .unwrap()
            .chars()
            .map(|ch| ch as u8)
            .collect_vec();
        let counts = split_space
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();
        let part_a = calc(&chars, &counts);
        let mut chars_mult = vec![];
        let mut counts_mult = vec![];
        for i in 0..5 {
            if i > 0 {
                chars_mult.push(b'?');
            }
            chars_mult.extend_from_slice(&chars);
            counts_mult.extend_from_slice(&counts);
        }
        
        let part_b = calc(&chars_mult, &counts_mult);
        (agg.0 + part_a, agg.1 + part_b)
    });

    format!("{}/{}", part_a, part_b)
}

fn calc(chars: &[u8], counts: &[usize]) -> usize {
    let mut cache: FxHashMap<[usize; 2], usize> = FxHashMap::default();
    inner_calc(chars, counts, &mut cache)
}

fn inner_calc(chars: &[u8], counts: &[usize], cache: &mut FxHashMap<[usize; 2], usize>) -> usize {
    let cache_key = [chars.len(), counts.len()];
    if let Some(cached_value) = cache.get(&cache_key) {
        *cached_value
    } else {
        let is_dot = |ch: &u8| *ch == b'.' || *ch == b'?';
        let is_dash = |ch: &u8| *ch == b'#' || *ch == b'?';
        let value = if !counts.is_empty() {
            (0..=(1 + chars.len() - counts.iter().sum::<usize>() - counts.len()))
                .map(|dot_count| {
                    if chars[0..dot_count].iter().all(is_dot)
                        && chars[dot_count..(dot_count + counts[0])]
                            .iter()
                            .all(is_dash)
                        && (counts.len() == 1 || is_dot(&chars[dot_count + counts[0]]))
                    {
                        inner_calc(
                            &chars
                                [(dot_count + counts[0] + if counts.len() > 1 { 1 } else { 0 })..],
                            &counts[1..],
                            cache,
                        )
                    } else {
                        0
                    }
                })
                .sum()
        } else if chars.iter().all(is_dot) {
            1
        } else {
            0
        };
        cache.insert(cache_key, value);
        value
    }
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1",
        ),
        "21/525152".into(),
    )
}
