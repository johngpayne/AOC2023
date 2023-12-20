use itertools::Itertools;
use rustc_hash::FxHashMap;

#[tracing::instrument(skip(input), fields(day = 8))]
pub fn solve(input: &str) -> String {
    let (instructions, moves) = get_data(input);
    format!("{}/{}", part_a(&instructions, &moves), part_b(&instructions, &moves))
}

fn get_data(input: &str) -> (Vec<usize>, FxHashMap<&str, [&str; 2]>) {
    let mut lines = input.lines();

    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|ch| if ch == 'L' { 0 } else { 1 })
        .collect_vec();
    tracing::debug!("{:?}", instructions);

    let moves: FxHashMap<&str, [&str; 2]> = lines
        .skip(1)
        .map(|line| {
            let line = line.trim_start();
            (&line[0..3], [&line[7..10], &line[12..15]])
        })
        .collect();
    tracing::debug!("{:?}", moves);

    (instructions, moves)
}

fn part_a(instructions: &Vec<usize>, moves: &FxHashMap<&str, [&str; 2]>) -> usize {
    run("AAA", instructions, moves)
}

fn part_b(instructions: &Vec<usize>, moves: &FxHashMap<&str, [&str; 2]>) -> usize {
    crate::utils::lcm(&moves
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| run(key, instructions, moves))
        .collect_vec())
}

fn run(start: &str, instructions: &Vec<usize>, moves: &FxHashMap<&str, [&str; 2]>) -> usize {
    let mut current = start;
    let mut index = 0;
    while !current.ends_with('Z') {
        current = moves[current][instructions[index % instructions.len()]];
        index += 1;
    }
    index
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    let (instructions, moves) = get_data(
        "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)",
    );
    let part_a = part_a(&instructions, &moves);

    let (instructions, moves) = get_data(
        "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)",
    );
    let part_b = part_b(&instructions, &moves);

    (format!("{}/{}", part_a, part_b), "6/6".into())
}
