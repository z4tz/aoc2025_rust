pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1, part2]
}

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication
}

fn part1(input: &str) -> String {
    let (columns, operations) = parse_input(input);
    let mut total_sum = 0;
    for (column, operation) in columns.iter().zip(operations.iter()) {
        match operation {
            Operation::Addition => {total_sum += column.iter().sum::<usize>()}
            Operation::Multiplication => {total_sum += column.iter().product::<usize>()}
        }
    }
    total_sum.to_string()
}

fn part2(input: &str) -> String {
    let (columns, operations) = parse_alternative(input);
    let mut total_sum = 0;
    for (column, operation) in columns.iter().zip(operations.iter()) {
        match operation {
            Operation::Addition => {total_sum += column.iter().sum::<usize>()}
            Operation::Multiplication => {total_sum += column.iter().product::<usize>()}
        }
    }
    total_sum.to_string()
}

fn parse_alternative( input: &str ) -> (Vec<Vec<usize>>, Vec<Operation>) {
    let mut columns = vec![];
    let operations = parse_operators(input.lines().last().unwrap());

    let mut char_columns = vec![vec![]; input.lines().nth(0).unwrap().len()];
    for line in input.lines().rev().skip(1) {
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                char_columns[i].push(c.to_digit(10).unwrap() as usize);
            }
        }
    }

    let mut new_column = vec![];
    for char_column in char_columns {
        if char_column.is_empty() {
            columns.push(new_column);
            new_column = vec![];
        }
        else {
            new_column.push(char_column.iter().enumerate().rev().fold(0, |acc, (i, num)| {10_usize.pow(i as u32) * num + acc}));
        }
    }
columns.push(new_column);
    (columns, operations)
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Operation>) {
    let mut columns = vec![];
    let operations = parse_operators(input.lines().last().unwrap());

    for line in input.lines().rev().skip(1) {
        for (i, part) in line.split_ascii_whitespace().enumerate() {
            if columns.len() <= i {
                columns.push(vec![part.parse::<usize>().unwrap()]);
            }
            else {
                columns[i].push(part.parse::<usize>().unwrap());
            }
        }
    }
    (columns, operations)
}

fn parse_operators(line: &str) -> Vec<Operation> {
    line.split_ascii_whitespace()
        .map(|part| {
            match part {
                "*" => {Operation::Multiplication}
                "+" => {Operation::Addition}
                _ => {panic!("Invalid operation \"{part}\"")}
            }
        })
        .collect()
}