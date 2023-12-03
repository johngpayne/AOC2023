use glam::{ivec2, IVec2};
use rustc_hash::FxHashMap;

#[tracing::instrument(skip(input), fields(day = 3))]
pub fn solve(input: &str) -> String {
    let num_digits = |num: u32| num.checked_ilog10().unwrap_or(0) as i32 + 1;
    let mut symbols: FxHashMap<IVec2, char> = FxHashMap::default();
    let mut nums: Vec<(IVec2, u32)> = vec![];
    input
        .lines()
        .map(|line| line.trim())
        .enumerate()
        .for_each(|(y, line)| {
            let mut acc: Option<u32> = None;
            line.chars().enumerate().for_each(|(x, ch)| {
                if let Some(digit) = ch.to_digit(10) {
                    acc = match acc {
                        None => Some(digit),
                        Some(acc_value) => Some(acc_value * 10 + digit),
                    };
                } else {
                    let coord = ivec2(x as i32, y as i32);
                    if let Some(acc_value) = acc {
                        nums.push((coord, acc_value));
                        acc = None;
                    }
                    if ch != '.' {
                        symbols.insert(coord, ch);
                    }
                }
            });
            if let Some(acc_value) = acc {
                nums.push((ivec2(line.len() as i32, y as i32), acc_value));
            }
        });

    let part_a = nums
        .iter()
        .filter(|(coord, num)| {
            (-1 - num_digits(*num)..1).any(|xo| {
                (-1..=1).any(|yo| symbols.contains_key(&ivec2(coord.x + xo, coord.y + yo)))
            })
        })
        .map(|(_, num)| num)
        .sum::<u32>();

    let part_b = symbols
        .iter()
        .filter(|(_, &ch)| ch == '*')
        .map(|(&gear_coord, _)| {
            nums.iter()
                .filter(|&&(num_coord, num)| {
                    let offset = num_coord - gear_coord;
                    offset.y.abs() <= 1 && offset.x <= num_digits(num) + 1 && offset.x >= 0
                })
                .map(|&(_, num)| num)
                .collect::<Vec<_>>()
        })
        .filter(|gear_nums| gear_nums.len() == 2)
        .map(|gear_nums| gear_nums.iter().product::<u32>())
        .sum::<u32>();

    format!("{}/{}", part_a, part_b)
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..",
        ),
        "4361/467835".into(),
    )
}
