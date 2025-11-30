use std::fs;
use std::time::Instant;
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
    let url = format!("https://adventofcode.com/20024/day/{}/input", day);
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
    let start = Instant::now();
    match &input {
        Some(input) => {
            functions.iter().enumerate().for_each(| (index, function) | {
                let part_start = Instant::now();
                println! ("Part {} answer: {}", index + 1, function(input));
                println ! ("Time for part: {:?}\n", part_start.elapsed());
            });
        }
        None => {println!("No input found")}
    }
    println!("Time for all parts {:?}", start.elapsed());
}