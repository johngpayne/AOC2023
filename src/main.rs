use anyhow::{anyhow, Error};
use chrono::{Datelike, FixedOffset, Utc};
use clap::Parser;
use inventory::{collect, submit};
use itertools::Itertools;
use reqwest::{Client, Method};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
    time::{Duration, Instant},
};
use tracing_subscriber::layer::SubscriberExt;

mod utils;

struct Solution {
    day: u32,
    solve: fn(&str) -> String,
    test: fn() -> (String, String),
}

macro_rules! add_day {
    ($num: literal, $module: tt) => {
        mod $module;
        submit! {
            Solution {
                day: $num,
                solve: $module::solve,
                test: $module::test,
            }
        }
    };
}

/*
// Template

use itertools::Itertools;

#[tracing::instrument(skip(input), fields(day=XXX))]
pub fn solve(input: &str) -> String {
    let lines = input.lines().map(|line| line.trim()).collect_vec();
    tracing::debug!("input {:?}", lines);
    format!("{}", 0)
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve("TEST_DATA"),
        "?".into(),
    )
}
*/

add_day!(1, day1);
add_day!(2, day2);
add_day!(3, day3);
add_day!(4, day4);
add_day!(5, day5);
add_day!(6, day6);
add_day!(7, day7);
add_day!(8, day8);
add_day!(9, day9);
add_day!(10, day10);
add_day!(11, day11);
add_day!(12, day12);
add_day!(13, day13);
add_day!(14, day14);
add_day!(15, day15);
add_day!(16, day16);
add_day!(17, day17);
add_day!(18, day18);
add_day!(19, day19);
add_day!(20, day20);
add_day!(21, day21);
add_day!(22, day22);
add_day!(23, day23);
add_day!(24, day24);

collect!(Solution);

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: Option<u32>,
    #[arg(long, default_value = "2023")]
    year: u32,
    #[arg(long)]
    all: bool,
    #[arg(long)]
    timed: bool,
    #[arg(long)]
    debug: bool,
    #[arg(long)]
    trace: bool,
    #[arg(long)]
    test_only: bool,
    #[arg(long)]
    no_test: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(if args.debug {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .with_target(false)
        .without_time()
        .finish();

    let _guard = if args.trace {
        let (chrome_layer, guard) = tracing_chrome::ChromeLayerBuilder::new().build();
        tracing::subscriber::set_global_default(subscriber.with(chrome_layer))?;
        Some(guard)
    } else {
        tracing::subscriber::set_global_default(subscriber)?;
        None
    };

    if args.all {
        let start = Instant::now();
        let tasks = (0..25)
            .map(|day| tokio::spawn(run(1 + day, args.year, args.test_only, args.no_test)))
            .collect_vec();
        let mut outputs = vec![];
        for task in tasks {
            outputs.push(task.await.unwrap())
        }
        for (index, output) in outputs.into_iter().enumerate() {
            write_output(1 + index as u32, output, &args);
        }
        let duration = Instant::now() - start;
        if args.timed {
            tracing::info!(
                "\x1b[93mCompleted in: {}\x1b[0m",
                short_duration_to_str(duration)
            );
        }
    } else {
        let day = if let Some(day) = args.day {
            day
        } else {
            get_today()?
        };
        write_output(
            day,
            run(day, args.year, args.test_only, args.no_test).await,
            &args,
        );
    }
    Ok(())
}

fn short_duration_to_str(duration: Duration) -> String {
    if duration < 5 * Duration::from_millis(1) {
        format!("{}μs", duration.as_micros())
    } else {
        format!("{}ms", duration.as_millis())
    }
}

fn write_output(day: u32, result: Result<(String, Duration), Error>, args: &Args) {
    let prefix = format!(
        "\x1b[34mDay {day}{} \x1b[0m",
        if day < 10 { " " } else { "" }
    );
    match result {
        Ok((result, duration)) => tracing::info!(
            "{}{}{}",
            prefix,
            result,
            if args.timed {
                format!("\x1b[93m ({})\x1b[0m", short_duration_to_str(duration))
            } else {
                String::default()
            }
        ),
        Err(err) => tracing::error!("{}{}", prefix, err),
    }
}

fn get_cache_path(day: u32, year: u32) -> PathBuf {
    format!("cache/{year}/day{day}.tmp").into()
}

async fn get_data(day: u32, year: u32) -> Result<String, Error> {
    // first check cache
    if let Ok(data) = read_to_string(get_cache_path(day, year)) {
        return Ok(data);
    }

    // otherwise request, using session.txt and user-agent.txt
    let session = read_to_string("session.txt")
        .map_err(|_| anyhow!("cannot find session.txt (needed for downloading data)"))?;
    let request = Client::new()
        .request(
            Method::GET,
            format!("https://adventofcode.com/{year}/day/{day}/input"),
        )
        .header("Cookie", format!("session={}", session))
        .header("User-Agent", include_str!("../user-agent.txt"));

    let response = request.send().await?;
    let text = response.text().await?;

    // cache for next time
    let path = get_cache_path(day, year);
    create_dir_all(path.parent().unwrap())?;
    write(path, text.clone())?;

    Ok(text)
}

async fn run(
    day: u32,
    year: u32,
    test_only: bool,
    no_test: bool,
) -> Result<(String, Duration), Error> {
    // find solution
    let solution = get_solution(day)?;

    // run test
    if !no_test {
        let test_start = Instant::now();
        let (test_result, test_expected) = (solution.test)();
        let test_duration = Instant::now() - test_start;

        if test_result != test_expected {
            return Err(anyhow!(
                "failed test, got '{}' expected '{}'",
                test_result,
                test_expected,
            ));
        }
        if test_only {
            return Ok(("passed".into(), test_duration));
        }
    }

    // get real data and run
    let data = get_data(day, year).await?;
    let start = Instant::now();
    let result = (solution.solve)(&data);
    let duration = Instant::now() - start;
    Ok((result, duration))
}

fn get_today() -> Result<u32, Error> {
    let now = Utc::now().with_timezone(&FixedOffset::west_opt(18000).unwrap());
    if Datelike::month(&now) != 12 || Datelike::day(&now) > 25 {
        Err(anyhow!("Advent of Code is not running"))
    } else {
        Ok(Datelike::day(&now))
    }
}

fn get_solution(day: u32) -> Result<&'static Solution, Error> {
    for solution in inventory::iter::<Solution> {
        if solution.day == day {
            return Ok(solution);
        }
    }
    Err(anyhow!("not implemented"))
}
