use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day = 6))]
pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let mut read_nums = || {
        lines
            .next()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse::<f64>().unwrap())
            .collect_vec()
    };
    let (times, distances) = (read_nums(), read_nums());
    tracing::debug!("times {:?} distances {:?}", times, distances);

    let calc = |time: f64, distance: f64| {
        let sqrt_dis = f64::sqrt(time * time - 4.0 * distance);
        let min = f64::floor(1.0 + 0.5 * (time - sqrt_dis));
        let max = f64::ceil(-1.0 + 0.5 * (time + sqrt_dis));
        1.0 + (max - min)
    };

    let part_a = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| calc(time, distance))
        .product::<f64>();

    let fold_nums = |nums: Vec<f64>| {
        nums.iter()
            .fold(0f64, |agg, &v| agg * 10f64.powf(f64::log10(v).ceil()) + v)
    };
    let part_b = calc(fold_nums(times), fold_nums(distances));

    format!("{}/{}", part_a, part_b)
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "Time:      7  15   30
        Distance:  9  40  200",
        ),
        "288/71503".into(),
    )
}
