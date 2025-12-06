use std::fs;
use std::time::{Duration, Instant};
use std::env::var;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, COOKIE};
use reqwest::StatusCode;

pub fn get_input(day: u32) -> Option<String> {
    let basepath = var("AOC2025-PATH").expect("AOC2025-PATH must be set to project directory");
    let filepath = format!(r"{basepath}\inputs\day{day}.txt");
    match fs::read_to_string(&filepath) {
        Ok(data) => {
            if !data.is_empty() {
                return Some(data)
            }
        }
        Err(_) => {}
    }
    match download_input(day) {
        Some(data) => {
            fs::write(&filepath, &data).unwrap_or(());
            Some(data)
        },
        None => {None}
    }
}

fn download_input(day: u32) -> Option<String> {
    let url = format!("https://adventofcode.com/2025/day/{}/input", day);
    let session = var("AOC2025-SESSION").expect("AOC2025-SESSION must be set");
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, format!("session={}", session).parse().unwrap());
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    match client.get(&url).send() {
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    return Some(response.text().unwrap());
                }
                _ => {println!("Bad status in response from AOC2025: {:?}", response.status());}
            }
        },
        Err(e) => {
            println!("Error downloading input for day {}: {}", day, e);
        }
    }
    None
}

pub fn time_solutions(functions: &[fn(&str) -> String], input: Option<String>) {
    match (&input, functions.len()) {
        (None, _)    => {println!("No input available")}
        (Some(_), 0) => {println!("No solution found")}
        (Some(input), 1) => {
            let (avg_time, answer) = average_time(functions.first().unwrap(), input);
            println! ("Answers for both parts: {}", answer);
            println ! ("Total time for day: {:?}\n", avg_time);
        }
        (Some(input), _) => {  // for 2 (or more?) solutions
            let mut total_time = Duration::new(0,0);
            functions.iter().enumerate().for_each(| (index, function) | {
                let (part_time, answer) = average_time(function, input);
                println!("Part {} answer: {}", index + 1, answer);
                println!("Time for part: {:?}", part_time);
                total_time += part_time;
            });
            println!("Time for all parts {:?}", total_time);
        }
    }
}

fn average_time(function : &fn(&str) -> String, input: &str) -> (Duration, String ) {
    let mut count = 0;
    let mut answer = String::new();
    let time = Instant::now();
    while Instant::now() - time < Duration::from_millis(500) {
        answer = function(input);
        count += 1;

    }

    ((Instant::now() - time)/count , answer)
}