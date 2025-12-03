
pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1, part2]
}

fn part1(input: &str) -> String {
    let numbers = parse_input(input);
    let mut invalid_id_sum = 0;
    for (num1, num2) in numbers {
        for number in num1..num2+1 {
            let number_string = number.to_string();
            let (start, end) = number_string.split_at(number_string.len()/2);
            if start == end { invalid_id_sum += number;}
        }
    }
    invalid_id_sum.to_string()
}

fn part2(input: &str) -> String {
    let numbers = parse_input(input);
    let mut invalid_id_sum = 0;
    for (num1, num2) in numbers {
        for number in num1..=num2 {
            let number_string = number.to_string();
            for pattern_len in 1..=number_string.len()/2 {
                if number_string.len()%pattern_len != 0 {continue}  // skip patterns where length cant match
                let pattern = &number_string[..pattern_len];
                let repeated = pattern.repeat(number_string.len() / pattern_len);
                if repeated == number_string {
                    invalid_id_sum += number;
                    break;
                }
            }
        }
    }
    invalid_id_sum.to_string()
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input.trim().split(',').map(|s| {
        let (first, second) = s.split_once('-').unwrap();
        (first.parse::<i64>().unwrap(), second.parse::<i64>().unwrap())
    }).collect()
}
