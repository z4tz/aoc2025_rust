pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1_and_part2]
}

fn part1_and_part2(input: &str) -> String {
    let mut beams = [0; 142];
    let mut splits = 0;
    for line in input.lines() {
        for (index, char) in line.chars().enumerate() {
            match char {
                'S' => {beams[index] = 1;}
                '^' => {
                    if beams[index] > 0 {
                        splits += 1;
                        beams[index-1] += beams[index];
                        beams[index+1] += beams[index];
                        beams[index] = 0;
                    }}
                _ => {}
            }
        }
    }
    format!("{}, {}", splits, beams.iter().sum::<usize>())
}
