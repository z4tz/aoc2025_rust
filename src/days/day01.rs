pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1_and_part2]
}

fn part1_and_part2(input: &str) -> String {
    let data = parse_input(input);
    let mut pointer = 50;
    let mut counter1 = 0;
    let mut counter2 = 0;
    for (turn, number) in data {
        match turn {
            Turn::Right => {
                counter2 += (pointer + number) / 100;
                pointer = (number + pointer ).rem_euclid(100);
            }
            Turn::Left => {
                let rev_pointer = (100 - pointer) % 100;  // reverse the dial and treat it like a right turn
                counter2 += (rev_pointer + number) / 100;
                pointer = (pointer - number ).rem_euclid(100);
            }
        }
        if pointer == 0 {
            counter1 += 1;
        }
    }
    format!("{counter1}, {counter2}")
}

fn parse_input(input: &str) -> Vec<(Turn, i32)> {
    input.lines()
        .map(|line| {
            let turn = if line.starts_with("L") {Turn::Left} else {Turn::Right};
            (turn, line[1..].parse().unwrap())
        }).collect()
}

enum Turn {
    Left,
    Right
}