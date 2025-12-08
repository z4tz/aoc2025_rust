pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1_and_part2]
}

fn part1_and_part2(input: &str) -> String {
    let positions = parse_input(input);
    let mut distances: Vec<(f64, Position, Position)> = vec![];
    let mut circuits: Vec<Vec<Position>> = positions.iter().map(|p| vec![*p]).collect();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            distances.push((positions[i].distance_to(positions[j]),positions[i], positions[j]));
        }
    }
    let mut part1 = 0;
    let mut part2 = 0;
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (i, (_, pos1, pos2)) in distances.iter().enumerate() {
        let pos1_circuit= circuits.iter().position(|c| c.contains(pos1)).unwrap();
        let pos2_circuit= circuits.iter().position(|c| c.contains(pos2)).unwrap();

        if pos1_circuit != pos2_circuit {
            let temp_circuit = circuits[pos2_circuit].clone();
            circuits[pos1_circuit].extend(temp_circuit);
            circuits.remove(pos2_circuit);
        }

        if i == 999 {  //part 1
            circuits.sort_by_key(|c| c.len());
            circuits.reverse();
            part1 = circuits[..3].iter().map(|c|c.len()).product::<usize>();
        }
        if circuits.len() == 1 { //part 2
            part2 = pos1.x * pos2.x;
            break;
        }
    }

    format!("{part1}, {part2}")
}

fn parse_input(input: &str) -> Vec<Position> {
    let mut positions = Vec::new();
    for line in input.lines() {
        let coords: Vec<i64> = line.splitn(3,',').map(|x| {x.parse::<i64>().unwrap()}).collect();
        positions.push(Position::new(coords[0], coords[1], coords[2]));
    }
    positions
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Position {
    x: i64,
    y: i64,
    z: i64
}
impl Position {
    fn new(x: i64, y: i64, z: i64) -> Position {
        Position { x, y, z }
    }

    fn distance_to(self, rhs: Position) -> f64 {
        (((self.x - rhs.x).pow(2) + (self.y - rhs.y).pow(2) + (self.z - rhs.z).pow(2)) as f64).sqrt()
    }
}
