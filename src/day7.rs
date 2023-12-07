use itertools::Itertools;
use std::{cmp::Ordering, ops::Range};

#[derive(Debug)]
struct Hand {
    card_indices: [usize; 5],
    bid: u32,
    counts: Vec<usize>,
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
    let card_types: [char; 13] = (2..10)
        .map(|i| char::from_digit(i, 10).unwrap())
        .chain("TJQKA".chars())
        .collect_vec()
        .try_into()
        .unwrap();

    let mut hands = hands
        .iter()
        .map(|&(cards, bid)| {
            let card_indices = get_card_indices(&cards, &card_types);
            let counts = get_card_counts(&card_indices, 0..card_types.len());
            Hand {
                card_indices,
                bid,
                counts,
            }
        })
        .collect_vec();

    sort_by_score(&mut hands)
}

fn part_b(hands: &[([char; 5], u32)]) -> u32 {
    let card_types: [char; 13] = "J"
        .chars()
        .chain(
            (2..10)
                .map(|i| char::from_digit(i, 10).unwrap())
                .chain("TQKA".chars()),
        )
        .collect_vec()
        .try_into()
        .unwrap();

    let mut hands = hands
        .iter()
        .map(|&(cards, bid)| {
            let card_indices = get_card_indices(&cards, &card_types);
            // ignore count of jokers
            let mut counts = get_card_counts(&card_indices, 1..card_types.len());
            let num_jokers = card_indices.iter().filter(|&&card| card == 0).count();

            // use jokers to increase the highest
            if !counts.is_empty() {
                counts[0] += num_jokers;
            // or N+1 of a kind up to max 5
            } else if num_jokers > 0 {
                counts.push((num_jokers + 1).min(5));
            }

            Hand {
                card_indices,
                bid,
                counts,
            }
        })
        .collect_vec();

    sort_by_score(&mut hands)
}

fn get_card_indices(cards: &[char; 5], card_types: &[char; 13]) -> [usize; 5] {
    cards
        .iter()
        .map(|&ch| card_types.iter().position(|&x| x == ch).unwrap())
        .collect_vec()
        .try_into()
        .unwrap()
}

fn get_card_counts(cards: &[usize; 5], card_range: Range<usize>) -> Vec<usize> {
    let mut counts = card_range
        .map(|card_index| {
            cards
                .iter()
                .filter(move |&&card| card_index == card)
                .count()
        })
        .filter(|&card_count| card_count > 1)
        .collect_vec();
    counts.sort_by_key(|&card_count| card_count);
    counts.reverse();
    counts
}

fn sort_by_score(hands: &mut [Hand]) -> u32 {
    hands.sort_by(|h1, h2| {
        if !h1.counts.is_empty() || !h2.counts.is_empty() {
            // If one hand has 2 or more of a kind and other doesn't...
            let ord = (!h1.counts.is_empty()).cmp(&(!h2.counts.is_empty()));
            if ord != Ordering::Equal {
                return ord;
            }
            // Compare "N of a kind" in each hand
            let ord = h1.counts[0].cmp(&h2.counts[0]);
            if ord != Ordering::Equal {
                return ord;
            }
            // Compare whether or not there is a secondary (ie full house or two pair)
            let ord = (h1.counts.len() > 1).cmp(&(h2.counts.len() > 1));
            if ord != Ordering::Equal {
                return ord;
            }
        }
        for (c1, c2) in h1.card_indices.iter().zip(h2.card_indices.iter()) {
            let ord = c1.cmp(c2);
            if ord != std::cmp::Ordering::Equal {
                return ord;
            }
        }
        std::cmp::Ordering::Equal
    });

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
