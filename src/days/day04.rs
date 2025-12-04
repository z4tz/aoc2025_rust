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
    let mut move_found = true;
    while move_found {
        let mut after_move: Array2<usize> = array.clone();
        move_found = false;
        for (index, window) in array.windows((3,3)).into_iter().enumerate() {
            if window[[1,1]] == 1 && window.sum() < 5 {
                let x = index % (array.dim().0-2) + 1;
                let y = index / (array.dim().0-2) + 1;
                after_move[[y,x]] = 0;
                forkliftable += 1;
                move_found = true;
            }
        }
        array = after_move;
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


