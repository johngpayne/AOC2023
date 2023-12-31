use itertools::Itertools;

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

        let chars = itertools::Itertools::intersperse([&chars].repeat(5).into_iter(), &vec![b'?'])
            .flatten()
            .copied()
            .collect_vec();
        let counts = counts.repeat(5);
        let part_b = calc(&chars, &counts);
        (agg.0 + part_a, agg.1 + part_b)
    });

    format!("{}/{}", part_a, part_b)
}

const MAX_CHARS: usize = 128;
const MAX_COUNTS: usize = 32;

fn calc(chars: &[u8], counts: &[usize]) -> usize {
    assert!(chars.len() < MAX_CHARS);
    assert!(counts.len() < MAX_COUNTS);
    let mut cache = vec![None; MAX_CHARS * MAX_COUNTS];
    inner_calc(chars, counts, &mut cache)
}

fn inner_calc(chars: &[u8], counts: &[usize], cache: &mut [Option<usize>]) -> usize {
    if let Some(cached_value) = cache[MAX_COUNTS * chars.len() + counts.len()] {
        cached_value
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
        cache[MAX_COUNTS * chars.len() + counts.len()] = Some(value);
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
