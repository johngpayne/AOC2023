#[tracing::instrument(skip(input), fields(day=2))]
pub fn solve(input: &str) -> String {
    let games = input
        .lines()
        .map(|line| {
            let mut line_split = line.split(": ");
            let game_index = line_split.next().unwrap().trim()[5..]
                .parse::<u32>()
                .unwrap();
            let game_turns = line_split
                .next()
                .unwrap()
                .split("; ")
                .map(|turn| {
                    let mut amounts = [0, 0, 0];
                    turn.split(", ").for_each(|part| {
                        let mut part_split = part.split(' ');
                        let part_num = part_split.next().unwrap().parse::<u32>().unwrap();
                        let part_type = part_split.next().unwrap();
                        amounts[["red", "green", "blue"]
                            .iter()
                            .position(|&cube_type| cube_type == part_type)
                            .unwrap()] = part_num;
                    });
                    amounts
                })
                .collect::<Vec<_>>();
            tracing::debug!("{} {:?}", game_index, game_turns);
            (game_index, game_turns)
        })
        .collect::<Vec<_>>();

    let part_a = games
        .iter()
        .filter(|(_, turns)| {
            turns.iter().all(|turn| {
                turn.iter()
                    .zip([12, 13, 14])
                    .all(|(&turn_amount, type_max)| turn_amount <= type_max)
            })
        })
        .map(|(index, _)| index)
        .sum::<u32>();

    let part_b = games
        .iter()
        .map(|(_, turns)| {
            turns.iter().fold([0, 0, 0], |agg, turn| {
                agg.iter()
                    .zip(turn)
                    .map(|(&a, &b)| u32::max(a, b))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
        })
        .map(|maxes| maxes.into_iter().product::<u32>())
        .sum::<u32>();

    format!("{}/{}", part_a, part_b)
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ),
        "8/2286".into(),
    )
}
