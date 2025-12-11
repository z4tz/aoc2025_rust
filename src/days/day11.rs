use std::collections::HashMap;

pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1, part2]
}

fn part1(input: &str) -> String {
    let devices = parse_input(input);
    let mut total = 0;
    let mut to_visit = vec!["you".to_string()];
    while !to_visit.is_empty() {
        let mut new_devices = vec![];
        for name in to_visit.iter() {
            if *name == "out" {
                total += 1;
            }
            else { new_devices.extend(devices[name].clone()); }
        }
        to_visit = new_devices;
    }
    total.to_string()
}

fn part2(input: &str) -> String {
    let devices = parse_input(input);
    let mut memoization = HashMap::new();
    let total = valid_connections("svr".to_string(),Visited::Nether, &devices, &mut memoization);

    total.to_string()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Visited {
    Nether,
    DAC,
    FFT,
    Both
}

fn valid_connections(device: String, visited: Visited, devices: &HashMap<String,Vec<String>>, memoization: &mut HashMap<(String, Visited), usize>) -> usize {
    if memoization.contains_key(&(device.clone(),visited)) {
        return memoization[&(device.clone(),visited)];
    }

    if device == "out"{
        match visited {
            Visited::Both  => return 1,
            _ => return 0
        }
    }
    let mut total = 0;
    for next_device in &devices[&device] {
        let next_visited = match (visited, next_device.as_str()) {
            (Visited::Nether, "fft") => Visited::FFT,
            (Visited::Nether, "dac") => Visited::DAC,
            (Visited::Nether, _) => Visited::Nether,
            (Visited::FFT, "dac") => Visited::Both,
            (Visited::FFT, _) => Visited::FFT,
            (Visited::DAC, "fft") => Visited::Both,
            (Visited::DAC, _) => Visited::DAC,
            (Visited::Both, _) => Visited::Both,
        };
        total += valid_connections(next_device.clone(), next_visited, devices, memoization);
    }

    memoization.insert((device.clone(), visited), total);
    total
}

fn parse_input(input: &str) -> HashMap<String,Vec<String>> {
    input.lines().map(|l| {
        let (key, val) = l.split_once(':').unwrap();
        let vals = val.split_whitespace().map(|v| v.to_string()).collect::<Vec<String>>();
        (key.to_string(), vals)
    }).collect()
}