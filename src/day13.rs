use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day=13))]
pub fn solve(input: &str) -> String {
    let (part_a, part_b) = input.split("\n\n").fold((0, 0), |agg, block| {

        let lines = block.lines().map(|line| {
            line.trim().chars().map(|ch| if ch == '#' { 1 } else { 0 }).collect_vec()
        }).collect_vec();
        
        let mut rows = vec![0u64; lines.len()];
        let mut columns = vec![0u64; lines[0].len()];
        for (y, line) in lines.iter().enumerate() {
            for (x, num) in line.iter().enumerate() {
                rows[y] = rows[y] * 2 + num;
                columns[x] = columns[x] * 2 + num;
            }
        }
        (
            agg.0 + 100 * find(&rows, 0).unwrap_or(0) + find(&columns, 0).unwrap_or(0),
            agg.1 + 100 * find(&rows, 1).unwrap_or(0) + find(&columns, 1).unwrap_or(0),
        )
    });
    format!("{}/{}", part_a, part_b)
}

fn find(vals: &[u64], num_wrong_bits: u32) -> Option<usize> {
    (1..vals.len()).find(|&index| {
        (0..(index.min(vals.len() - index))).map(|cmp| {
            (vals[index - 1 - cmp] ^ vals[index + cmp]).count_ones()
        }).sum::<u32>() == num_wrong_bits
    })
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve("#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"),
        "405/400".into(),
    )
}