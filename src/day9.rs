use glam::{ivec2, IVec2};
use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day = 9))]
pub fn solve(input: &str) -> String {
    let parts = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .fold(IVec2::ZERO, |acc, line| acc + get_next(line));
    format!("{}/{}", parts.x, parts.y)
}

fn get_next(line: Vec<i32>) -> IVec2 {
    if line.iter().all(|&val| val == 0) {
        IVec2::ZERO
    } else {
        ivec2(line[line.len() - 1], line[0])
            + ivec2(1, -1)
                * get_next(
                    line.windows(2)
                        .map(|slice| slice[1] - slice[0])
                        .collect_vec(),
                )
    }
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45",
        ),
        "114/2".into(),
    )
}
