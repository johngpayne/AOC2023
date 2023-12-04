use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day = 4))]
pub fn solve(input: &str) -> String {
    let (part_a, part_b, _) = input.lines().fold(
        (0u32, 0u32, Vec::<(u32, u32)>::default()),
        |(part_a, part_b, mut extra_cards), line| {
            let mut line_split = line.split(": ");
            let mut card_parts = line_split.nth(1).unwrap().split("| ");
            let mut get_nums = || {
                card_parts
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
            };
            let winning_nums = get_nums();
            let held_nums = get_nums().collect_vec();
            let matches = winning_nums
                .filter(|winning_num| held_nums.contains(winning_num))
                .count() as u32;

            let total_copies = extra_cards.iter().map(|(copies, _)| copies).sum::<u32>() + 1;
            extra_cards = extra_cards
                .iter()
                .map(|&(copies, num)| (copies, num - 1))
                .filter(|&(_, num)| num > 0)
                .collect();
            if matches > 0 {
                extra_cards.push((total_copies, matches));
            }
            (
                part_a + (1 << matches) / 2,
                part_b + total_copies,
                extra_cards,
            )
        },
    );

    format!("{}/{}", part_a, part_b)
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ),
        "13/30".into(),
    )
}
