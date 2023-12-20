use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(PartialEq, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop(PulseType),
    Conjunction(FxHashMap<usize, PulseType>),
    SingleInputConjunction(PulseType),
}

impl From<char> for ModuleType {
    fn from(ch: char) -> Self {
        match ch {
            'b' => ModuleType::Broadcaster,
            '%' => ModuleType::FlipFlop(PulseType::Low),
            '&' => ModuleType::Conjunction(FxHashMap::default()),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum PulseType {
    Low,
    High,
}

impl std::ops::Not for PulseType {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            PulseType::Low => PulseType::High,
            PulseType::High => PulseType::Low,
        }
    }
}

#[tracing::instrument(skip(input), fields(day = 20))]
pub fn solve(input: &str) -> String {
    run(input, true)
}

pub fn run(input: &str, has_rx: bool) -> String {
    let modules = input
        .lines()
        .map(|line| {
            let mut line_split = line.trim().split(" -> ");
            let type_and_id_str = line_split.next().unwrap();
            let mod_type = ModuleType::from(type_and_id_str[0..1].chars().next().unwrap());
            let id = &type_and_id_str[(if mod_type == ModuleType::Broadcaster {
                0
            } else {
                1
            })..];
            let targets = line_split.next().unwrap().split(", ").collect_vec();
            (mod_type, id, targets)
        })
        .collect_vec();

    let mut id_to_index: FxHashMap<&str, usize> = modules
        .iter()
        .enumerate()
        .map(|(index, (_, id, _))| (*id, index))
        .collect();
    if has_rx {
        id_to_index.insert("rx", usize::MAX);
    }

    let mut modules = modules
        .into_iter()
        .map(|(mod_type, _, targets)| {
            (
                mod_type,
                targets
                    .into_iter()
                    .map(|target| id_to_index.get(&target).copied())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let mut all_sources = vec![vec![]; modules.len()];
    modules
        .iter()
        .enumerate()
        .for_each(|(index, (_, targets))| {
            targets.iter().for_each(|target| {
                if let Some(target) = target {
                    if *target != usize::MAX {
                        all_sources[*target].push(index);
                    }
                }
            });
        });

    modules
        .iter_mut()
        .enumerate()
        .filter(|(_, (mod_type, _))| matches!(mod_type, ModuleType::Conjunction(_)))
        .for_each(|(index, (mod_type, _))| {
            *mod_type = match mod_type {
                ModuleType::Conjunction(_) => {
                    let sources = &all_sources[index];
                    if sources.len() == 1 {
                        ModuleType::SingleInputConjunction(PulseType::Low)
                    } else {
                        ModuleType::Conjunction(
                            all_sources[index]
                                .iter()
                                .map(|&source| (source, PulseType::Low))
                                .collect(),
                        )
                    }
                }
                _ => panic!(),
            };
        });

    tracing::debug!("modules indexed {:?}", modules);

    let mut rx_watches = if has_rx {
        // rx is pointed at by one Conjunction
        let targets_rx = modules
            .iter()
            .enumerate()
            .find(|(_, (_, targets))| targets.contains(&Some(usize::MAX)))
            .map(|(index, _)| index)
            .unwrap();
        assert!(matches!(modules[targets_rx].0, ModuleType::Conjunction(_)));

        let sources = modules
            .iter()
            .enumerate()
            .filter(|(_, (_, targets))| targets.contains(&Some(targets_rx)))
            .map(|(index, _)| (index, None))
            .collect_vec();
        assert!(sources.iter().all(|&(source, _)| matches!(
            modules[source].0,
            ModuleType::SingleInputConjunction(_)
        )));
        sources
    } else {
        vec![]
    };

    let mut total_low = 0;
    let mut total_high = 0;
    let mut run = 1;

    loop {
        let mut next_pulses = vec![(
            usize::MAX,
            modules
                .iter()
                .enumerate()
                .find_map(|(index, (mod_type, _))| {
                    if *mod_type == ModuleType::Broadcaster {
                        Some(index)
                    } else {
                        None
                    }
                }),
            PulseType::Low,
        )];

        while !next_pulses.is_empty() {
            let mut last_pulses: Vec<(usize, Option<usize>, PulseType)> = vec![];
            std::mem::swap(&mut next_pulses, &mut last_pulses);

            if run <= 1000 {
                let (low, high) = last_pulses
                    .iter()
                    .fold((0, 0), |(low, high), (_, _, pulse)| {
                        (low + !*pulse as usize, high + *pulse as usize)
                    });
                total_low += low;
                total_high += high;
            }

            for (from_index, to_index, pulse) in last_pulses {
                if let Some(to_index) = to_index {
                    if let Some((mod_type, targets)) = modules.get_mut(to_index) {
                        let maybe_send = match mod_type {
                            ModuleType::Broadcaster => Some(pulse),
                            ModuleType::FlipFlop(val) => {
                                if pulse == PulseType::Low {
                                    *val = !*val;
                                    Some(*val)
                                } else {
                                    None
                                }
                            }
                            ModuleType::Conjunction(sources) => {
                                sources.insert(from_index, pulse);
                                if sources.iter().all(|(_, stored)| *stored == PulseType::High) {
                                    Some(PulseType::Low)
                                } else {
                                    Some(PulseType::High)
                                }
                            }
                            ModuleType::SingleInputConjunction(val) => {
                                if pulse == PulseType::Low {
                                    if let Some((_, wrap)) = rx_watches
                                        .iter_mut()
                                        .filter(|(_, wrap)| wrap.is_none())
                                        .find(|(index, _)| *index == to_index)
                                    {
                                        *wrap = Some(run);
                                    }
                                }
                                *val = pulse;
                                Some(!*val)
                            }
                        };

                        if let Some(send) = maybe_send {
                            next_pulses
                                .extend(targets.iter().map(|&target| (to_index, target, send)));
                        }
                    }
                }
            }
        }

        if run >= 1000 && (!has_rx || rx_watches.iter().all(|(_, wrap)| wrap.is_some())) {
            break;
        }
        run += 1;
    }

    let part_a = total_low * total_high;
    if has_rx {
        format!(
            "{}/{:?}",
            part_a,
            crate::utils::lcm(&rx_watches
                .into_iter()
                .map(|(_, wrap)| wrap.unwrap())
                .collect_vec())
        )
    } else {
        format!("{}", part_a)
    }
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        run(
            "broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output",
            false,
        ),
        "11687500".into(),
    )
}
