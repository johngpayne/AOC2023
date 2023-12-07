use itertools::Itertools;
use std::ops::Range;

#[derive(Debug)]
struct Hand {
    bid: u32,
    score: u32,
}

#[tracing::instrument(skip(input), fields(day = 7))]
pub fn solve(input: &str) -> String {
    let hands = input
        .lines()
        .map(|line| {
            let line = line.trim_start();
            let cards: [char; 5] = line.trim_start()[0..5]
                .chars()
                .collect_vec()
                .try_into()
                .unwrap();
            let bid = line[6..].parse::<u32>().unwrap();
            (cards, bid)
        })
        .collect_vec();

    format!("{}/{}", part_a(&hands), part_b(&hands))
}

fn part_a(hands: &[([char; 5], u32)]) -> u32 {
    let card_types = "23456789TJQKA".chars().collect_vec().try_into().unwrap();
    let mut hands = hands
        .iter()
        .map(|&(cards, bid)| {
            let card_indices = get_card_indices(&cards, &card_types);
            let counts = get_card_counts(&card_indices, 0..card_types.len());
            let score = get_hand_score(&card_indices, &counts);
            Hand { bid, score }
        })
        .collect_vec();

    sort_by_score_and_sum(&mut hands)
}

fn part_b(hands: &[([char; 5], u32)]) -> u32 {
    let card_types = "J23456789TQKA".chars().collect_vec().try_into().unwrap();
    let mut hands = hands
        .iter()
        .map(|&(cards, bid)| {
            let card_indices = get_card_indices(&cards, &card_types);
            let mut counts = get_card_counts(&card_indices, 1..card_types.len());
            let num_jokers = card_indices.iter().filter(|&&card| card == 0).count();
            if !counts.is_empty() {
                counts[0] += num_jokers;
            } else if num_jokers > 0 {
                counts.push((num_jokers + 1).min(5));
            }
            let score = get_hand_score(&card_indices, &counts);
            Hand { bid, score }
        })
        .collect_vec();

    sort_by_score_and_sum(&mut hands)
}

fn get_card_indices(cards: &[char; 5], card_types: &[char; 13]) -> [usize; 5] {
    cards
        .iter()
        .map(|&ch| card_types.iter().position(|&x| x == ch).unwrap())
        .collect_vec()
        .try_into()
        .unwrap()
}

fn get_card_counts(card_indices: &[usize; 5], card_range: Range<usize>) -> Vec<usize> {
    let mut counts = card_range
        .map(|card_index| {
            card_indices
                .iter()
                .filter(move |&&card| card_index == card)
                .count()
        })
        .filter(|&card_count| card_count > 1)
        .collect_vec();
    counts.sort_by_key(|&card_count| usize::MAX - card_count);
    counts
}

fn get_hand_score(card_indices: &[usize; 5], counts: &Vec<usize>) -> u32 {
    ((counts.len() + 2 * *counts.first().unwrap_or(&0)) as u32) * 13_u32.pow(5)
        + card_indices
            .iter()
            .fold(0u32, |agg, &card_index| agg * 13 + (card_index as u32))
}

fn sort_by_score_and_sum(hands: &mut [Hand]) -> u32 {
    hands.sort_by_key(|hand| hand.score);
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (1 + index as u32) * hand.bid)
        .sum::<u32>()
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483",
        ),
        "6440/5905".into(),
    )
}
