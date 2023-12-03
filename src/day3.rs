use glam::{ivec2, IVec2};
use rustc_hash::FxHashMap;

struct Num {
    value: u32,
    cursor: IVec2,
}

impl Num {
    // cursor is coord of the char just after the number
    fn new(value: u32, cursor: IVec2) -> Self {
        Num { value, cursor }
    }
    // x of start of number
    fn start_x(&self) -> i32 {
        self.cursor.x - (1 + self.value.checked_ilog10().unwrap_or(0) as i32)
    }
    // x to the right of the number
    fn end_x(&self) -> i32 {
        self.cursor.x
    }
}

struct Symbol {
    ch: char,
    near_nums: Vec<u32>,
}

#[tracing::instrument(skip(input), fields(day = 3))]
pub fn solve(input: &str) -> String {
    let mut symbols: FxHashMap<IVec2, Symbol> = FxHashMap::default();
    let mut nums: Vec<Num> = vec![];
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
                    let cursor = ivec2(x as i32, y as i32);
                    if let Some(acc_value) = acc {
                        nums.push(Num::new(acc_value, cursor));
                        acc = None;
                    }
                    if ch != '.' {
                        symbols.insert(
                            cursor,
                            Symbol {
                                ch,
                                near_nums: vec![],
                            },
                        );
                    }
                }
            });
            if let Some(acc_value) = acc {
                nums.push(Num::new(acc_value, ivec2(line.len() as i32, y as i32)));
            }
        });

    // put all nums into the symbol's vec
    nums.iter().for_each(|num| {
        (num.start_x() - 1..num.end_x() + 1).for_each(|x| {
            (num.cursor.y - 1..=num.cursor.y + 1).for_each(|y| {
                if let Some(Symbol { ref mut near_nums, .. }) = symbols.get_mut(&ivec2(x, y)) {
                    near_nums.push(num.value);
                }
            })
        })
    });

    let part_a = symbols.values().map(|symbol| symbol.near_nums.iter().sum::<u32>())
        .sum::<u32>();

    let part_b = symbols
        .values()
        .filter(|symbol| symbol.ch == '*' && symbol.near_nums.len() == 2)
        .map(|symbol| symbol.near_nums.iter().product::<u32>())
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
