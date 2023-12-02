use anyhow::{anyhow, Error};
use chrono::{Datelike, FixedOffset, Utc};
use clap::Parser;
use futures::future::join_all;
use inventory::{collect, submit};
use reqwest::{Client, Method};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
    time::{Duration, Instant},
};

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

#[tracing::instrument(skip(input), fields(day=N))]
pub fn solve(input: &str) -> String {
    format!("SOLVE({})", input)
}

#[tracing::instrument]
pub fn test() -> (String, String) {
    (
        solve("TEST"),
        "SOLUTION".into(),
    )
}
*/

add_day!(1, day1);
add_day!(2, day2);

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
    test_only: bool,
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

    tracing::subscriber::set_global_default(subscriber)?;

    if args.all {
        let tasks = (0..25).map(|day| run(1 + day, &args)).collect::<Vec<_>>();
        let outputs = join_all(tasks).await;
        for (index, output) in outputs.into_iter().enumerate() {
            print(1 + index as u32, output, &args);
        }
    } else {
        let day = if let Some(day) = args.day {
            day
        } else {
            get_today()?
        };
        print(day, run(day, &args).await, &args);
    }
    Ok(())
}

fn print(day: u32, result: Result<(String, Duration), Error>, args: &Args) {
    let prefix = format!("\x1b[34mDay {day}{} \x1b[0m", if day < 10 { " " } else { "" });
    match result {
        Ok((result, duration)) => tracing::info!(
            "{}{}{}",
            prefix,
            result,
            if args.timed {
                if duration < Duration::from_millis(1) {
                    format!("\x1b[93m ({}μs)\x1b[0m", duration.as_micros())
                } else {
                    format!("\x1b[93m ({}ms)\x1b[0m", duration.as_millis())
                }
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
    let _ = create_dir_all(path.parent().unwrap());
    write(path, text.clone())?;

    Ok(text)
}

async fn run(day: u32, args: &Args) -> Result<(String, Duration), Error> {
    // find solution
    let solution = get_solution(day)?;

    // run test
    let test_start = Instant::now();
    let (test_result, test_expected) = (solution.test)();
    let test_duration = Instant::now() - test_start;

    if test_result != test_expected {
        return Err(anyhow!(
            "failed, got '{}' expected '{}'",
            test_result,
            test_expected,
        ));
    }
    if args.test_only {
        return Ok(("passed".into(), test_duration));
    }

    // get real data and run
    let data = get_data(day, args.year).await?;
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
