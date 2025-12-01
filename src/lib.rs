use std::fs;
use std::time::{Duration, Instant};
use std::env::var;

use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use reqwest::header::HeaderMap;
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
            let part_start = Instant::now();
            println! ("Answers for both parts: {}", functions.first().unwrap()(input));
            let elapsed = part_start.elapsed();
            println ! ("Total time for day: {:?}\n", elapsed);
        }
        (Some(input), _) => {  // for 2 (or more?) solutions
            let mut total_time = Duration::new(0,0);
            functions.iter().enumerate().for_each(| (index, function) | {
                let part_start = Instant::now();
                println! ("Part {} answer: {}", index + 1, function(input));
                let elapsed = part_start.elapsed();
                println ! ("Time for part: {:?}\n", elapsed);
                total_time += elapsed;
            });
            println!("Time for all parts {:?}", total_time);
        }
    }
}