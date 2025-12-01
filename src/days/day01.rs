pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1_and_part2]
}

fn part1_and_part2(input: &str) -> String {
    let data = parse_input(input);
    let mut pointer = 50;
    let mut counter1 = 0;
    let mut counter2 = 0;
    for number in data {
        if number > 0 {
            counter2 += (pointer + number) / 100;
        }
        else {
            let rev_pointer = (100 - pointer) % 100;  // reverse the dial and treat it like a normal turn
            counter2 += (rev_pointer + number.abs()) / 100;
        }
        pointer = (number + pointer ).rem_euclid(100);
        if pointer == 0 {
            counter1 += 1;
        }
    }
    format!("{counter1}, {counter2}")
}

fn parse_input(input: &str) -> Vec<i32> {
    input.replace('L', "-").replace('R', "")
        .lines().map(|l| l.parse().unwrap())
        .collect()
}