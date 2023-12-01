pub fn solve(input: &str) -> String {
    format!("{}/{}", part_a(input), part_b(input))
}

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

fn part_a(input: &str) -> u32 {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums = line
                .chars()
                .filter(|ch| ch.is_ascii_digit())
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum()
}

fn part_b(input: &str) -> u32 {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|mut line| {
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
            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum()
}
