use aoc2025_rust::{get_input, time_solutions};

mod days;


fn main() {
    run_day(8);
    //run_day_once(8)
    //run_all();
}

fn run_all() {
    for day in 1..=12 {
        run_day(day);
    }
}

fn run_day(day: u32) {
    match get_day_solutions(day) {
        Some(solutions) => {
            println!("Solving day {day}");
            time_solutions(&solutions, get_input(day));
            println!("-------------------------------------------------------\n");
        }
        None => {println!("Invalid day selected: {day}");}
    }
}

fn run_day_once(day: u32) {
    match (get_day_solutions(day), get_input(day)) {
        (Some(solutions), Some(input)) => {
            println!("Solving day {day}");
            for (i, solution) in solutions.iter().enumerate() {
                println!("Part {i}: {}", solution(&input));
            }
        }
        _ => println!("Missing solution or input for day: {day}"),
    }
}


fn get_day_solutions(day: u32) -> Option<Vec<fn(&str) -> String>> {
    match day {
        1 => Some(days::day01::get_solutions()),
        2 => Some(days::day02::get_solutions()),
        3 => Some(days::day03::get_solutions()),
        4 => Some(days::day04::get_solutions()),
        5 => Some(days::day05::get_solutions()),
        6 => Some(days::day06::get_solutions()),
        7 => Some(days::day07::get_solutions()),
        8 => Some(days::day08::get_solutions()),
        // 9 => Some(days::day09::get_solutions()),
        // 10 => Some(days::day10::get_solutions()),
        // 11 => Some(days::day11::get_solutions()),
        // 12 => Some(days::day12::get_solutions()),
        _ => None,

    }
}
