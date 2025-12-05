use ndarray::Array2;

pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1, part2]
}

fn part1(input: &str) -> String {
    let array = parse_input(input);
    array.windows((3,3)).into_iter()
        .filter(|&window| { window[[1,1]] == 1 && window.sum() < 5})
        .count()
        .to_string()
}

fn part2(input: &str) -> String {
    let mut array = parse_input(input);
    let mut forkliftable = 0;
    loop {
        let mut to_remove = vec![];
        for (index, window) in array.windows((3,3)).into_iter().enumerate() {
            if window[[1,1]] == 1 && window.sum() < 5 {
                let x = index % (array.dim().0-2) + 1;
                let y = index / (array.dim().0-2) + 1;
                to_remove.push((y, x));
            }
        }
        if to_remove.is_empty() {break}
        forkliftable += to_remove.len();
        for coordinate in to_remove {
            array[coordinate] = 0;
        }
    }
    forkliftable.to_string()
}

fn parse_input (input : &str) -> Array2<usize> {
    // create 2d array with zeros padded around the data for the window to move along properly
    let height = input.lines().count();
    let width = input.lines().next().unwrap().trim().len();
    let mut array: Array2<usize> = Array2::zeros((width + 2, height + 2));
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '@' {
                array[[y+1, x+1]] = 1;
            }
        }
    }
    array
}


