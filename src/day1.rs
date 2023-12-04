use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day=1))]
pub fn solve(input: &str) -> String {
    format!("{}/{}", part_a(input), part_b(input))
}

#[tracing::instrument(fields(day=1))]
pub fn test() -> (String, String) {
    (
        format!(
            "{}/{}",
            part_a(
                "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet",
            ),
            part_b(
                "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen",
            )
        ),
        "142/281".into(),
    )
}

#[tracing::instrument(skip(input))]
fn part_a(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let nums = line
                .chars()
                .filter(|ch| ch.is_ascii_digit())
                .map(|ch| ch.to_digit(10).unwrap())
                .collect_vec();
            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum()
}

#[tracing::instrument(skip(input))]
fn part_b(input: &str) -> u32 {
    input
        .lines()
        .map(|mut line| {
            tracing::debug!("line {}", line);
            let mut nums: Vec<u32> = vec![];
            while !line.is_empty() {
                let digit_strs = [
                    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ];
                for digit in 1..10 {
                    if line.starts_with(char::from_digit(digit, 10).unwrap())
                        || line.starts_with(digit_strs[digit as usize])
                    {
                        nums.push(digit);
                    }
                }
                line = &line[1..];
            }
            tracing::debug!("nums {:?}", nums);
            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum()
}
