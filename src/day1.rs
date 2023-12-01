pub fn solve(input: &str) -> String {
    format!("{}/{}", part_a(input), part_b(input))
}

pub fn test() -> (bool, String) {
    let part_a_test_input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    let part_a = part_a(part_a_test_input);

    let part_b_test_input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    let part_b = part_b(part_b_test_input);

    let result = format!("{}/{}", part_a, part_b);
    (result == "142/281", result)
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
        .map(|line| {
            let mut line = line;
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
