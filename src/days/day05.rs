use std::ops::RangeInclusive;

pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1, part2]
}

fn part1(input: &str) -> String {
    let (ranges, numbers) = parse_input(input);
    let mut fresh = 0;
    for number in numbers {
        for range in ranges.iter() {
            if range.contains(&number) { fresh += 1; break; }
        }
    }
    fresh.to_string()
}

fn part2(input: &str) -> String {
    let (mut ranges, _) = parse_input(input);
    ranges.sort_by_key(|range| *range.start());
    let mut combined_ranges = vec![ranges[0].clone()];
    for range in ranges.iter().skip(1) {
        match (combined_ranges.last().unwrap().contains(range.start()), combined_ranges.last().unwrap().contains(range.end())) {
            (true, true) => {continue} // range already contained
            (false, true) => {panic!("Not possible since ranges are sorted")}
            (true, false) => {
                let combined_range = combined_ranges.pop().unwrap();
                combined_ranges.push(*combined_range.start()..=*range.end());
            }
            (false, false) => {combined_ranges.push(range.clone());}
        }
    }
    combined_ranges.iter()
        .fold(0, |acc, x| acc + (x.end() - x.start() + 1))
        .to_string()
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let mut ranges = vec![];
    let mut numbers = vec![];

    for line in input.lines() {
        if line.contains('-') {
            let (start, end) = line.split_once('-').unwrap();
            ranges.push(start.parse().unwrap()..=end.parse().unwrap());
        }
        else if line.is_empty() {continue}
        else {numbers.push(line.parse().unwrap());}
    }
    (ranges, numbers)
}