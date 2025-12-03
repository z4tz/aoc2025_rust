pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1, part2]
}

fn part1(input: &str) -> String {
    let banks = parse_input(input);
    let mut total = 0;
    for bank in banks {
        let (index, max) = bank[.. bank.len() - 1]
            .iter()
            .enumerate()
            .rev()  // reverse to get first instance of max value
            .max_by_key(|(index, value)| *value)
            .unwrap();
        let second_max = bank[index+1..].iter().max().unwrap();
        total += max * 10 + second_max;
    }
    total.to_string()
}

fn part2(input: &str) -> String {
    let banks = parse_input(input);
    let mut total = 0;
    for bank in banks {
        let mut numbers = vec![];
        let mut start_index = 0;
        for i in (0..12).rev() {
            let (max_value_index, max_value) = bank[start_index.. bank.len() - i]
                .iter()
                .enumerate()
                .rev()  // reverse to get first instance of max value
                .max_by_key(|(_, value)| *value)
                .unwrap();
            numbers.push(max_value);
            start_index += max_value_index+1;
        }
        total += numbers.into_iter()  // join vec to integer
            .rev()
            .enumerate()
            .fold(0, |acc, (index, value)| acc + 10_usize.pow(index as u32) *value);
    }
    total.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect()).collect()
}