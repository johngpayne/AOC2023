use std::ops::RangeInclusive;

use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug, Copy, Clone)]
enum Test {
    Greater(usize, u32),
    Less(usize, u32),
    Always,
}

impl Test {
    fn reverse(&self) -> Self {
        match self {
            Test::Greater(index, val) => Test::Less(*index, val + 1),
            Test::Less(index, val) => Test::Greater(*index, val - 1),
            Test::Always => panic!(),
        }
    }
}

#[tracing::instrument(skip(input), fields(day = 19))]
pub fn solve(input: &str) -> String {
    let mut input_split = input.split("\n\n");

    let insts: FxHashMap<&str, Vec<(Test, &str)>> = input_split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.trim();
            let mut line_split = line[0..(line.len() - 1)].split('{');
            let name = line_split.next().unwrap();
            let mut commands = line_split
                .next()
                .unwrap()
                .split(',')
                .map(|command_str| {
                    if !command_str.contains(':') {
                        (Test::Always, command_str)
                    } else {
                        let mut command_split = command_str.split(':');
                        let test = command_split.next().unwrap();
                        let var_ch = test[0..1].chars().next().unwrap();
                        let var = ['x', 'm', 'a', 's']
                            .into_iter()
                            .position(|ch| ch == var_ch)
                            .unwrap();

                        let val = test[2..].parse::<u32>().unwrap();
                        let target = command_split.next().unwrap();
                        let op = test[1..2].chars().next().unwrap();
                        (
                            match op {
                                '>' => Test::Greater(var, val),
                                '<' => Test::Less(var, val),
                                _ => panic!(),
                            },
                            target,
                        )
                    }
                })
                .collect_vec();

            // simplify commands
            while commands.len() >= 2
                && commands[commands.len() - 2].1 == commands[commands.len() - 1].1
            {
                commands.remove(commands.len() - 2);
            }

            (name, commands)
        })
        .collect();

    let values = input_split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let line = line.trim();
            let mut line_split = line[1..line.len() - 1].split(',');

            [0, 1, 2, 3].map(move |_| {
                let part = line_split.next().unwrap();
                part[2..].parse::<u32>().unwrap()
            })
        })
        .collect_vec();

    format!("{}/{}", part_a(&insts, &values), part_b(&insts))
}

fn part_a(insts: &FxHashMap<&str, Vec<(Test, &str)>>, values: &[[u32; 4]]) -> u32 {
    values
        .iter()
        .map(|value| {
            let mut pos = "in";
            loop {
                if pos == "A" {
                    return value.iter().sum::<u32>();
                } else if pos == "R" {
                    return 0;
                } else {
                    pos = insts
                        .get(&pos)
                        .unwrap()
                        .iter()
                        .find(|command| match command.0 {
                            Test::Always => true,
                            Test::Greater(index, val) => value[index] > val,
                            Test::Less(index, val) => value[index] < val,
                        })
                        .unwrap()
                        .1;
                }
            }
        })
        .sum::<u32>()
}

type Cache<'a> = FxHashMap<&'a str, Vec<[RangeInclusive<u32>; 4]>>;

fn part_b(insts: &FxHashMap<&str, Vec<(Test, &str)>>) -> u64 {
    let mut cache: Cache = Cache::default();
    cache_routes_to(insts, "A", &mut cache);
    cache.get("A").unwrap().iter().map(|result| {
        result
            .iter()
            .map(|range| (1 + range.end() - range.start()) as u64)
            .product::<u64>()
    }).sum::<u64>()
}

fn filter(results: &mut [RangeInclusive<u32>; 4], test: &Test) {
    match test {
        Test::Greater(index, val) => {
            results[*index] = *results[*index].start().max(&(val + 1))..=*results[*index].end()
        }
        Test::Less(index, val) => {
            results[*index] = *results[*index].start()..=*results[*index].end().min(&(val - 1))
        }
        Test::Always => {}
    }
}

fn cache_routes_to<'a>(
    insts: &FxHashMap<&'a str, Vec<(Test, &str)>>,
    to: &'a str,
    cache: &mut Cache<'a>,
) {
    if !cache.contains_key(to) {
        let mut results = vec![];
        if to == "in" {
            results.push([0, 1, 2, 3].map(|_| 1..=4000));
        } else {
            for (&name, commands) in insts.iter() {
                for (index, (test, goto)) in commands.iter().enumerate() {
                    if goto == &to {
                        cache_routes_to(insts, name, cache);
                        for sub_route in cache.get(name).unwrap() {
                            let mut sub_route = sub_route.clone();
                            for (prev_test, _) in commands.iter().take(index) {
                                filter(&mut sub_route, &prev_test.reverse());
                            }
                            filter(&mut sub_route, test);
                            results.push(sub_route);
                        }
                    }
                }
            }
        }
        cache.insert(to, results.clone());
    }
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve(
            "px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}",
        ),
        "19114/167409079868000".into(),
    )
}
