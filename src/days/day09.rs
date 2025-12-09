pub fn get_solutions() -> Vec<fn(&str) -> String> {
    vec![part1, part2]
}

fn part1(input: &str) -> String {
    let points = parse_input(input);
    let mut largest_area: i64 = 0;
    for (i, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(i + 1) {
            let area = ((point1.x - point2.x).abs() + 1) * ((point1.y - point2.y).abs() + 1);
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    largest_area.to_string()
}

fn part2(input: &str) -> String {
    let mut points = parse_input(input);
    points.push(points[0].clone()); // add first point again so segment p1 -> pn-1 can be created
    let segments = points.iter()
        .zip(points.iter().skip(1))
        .map(|(p1,p2)| (*p1,*p2))
        .collect();
    points.pop().unwrap();
    let mut largest_area: i64 = 0;

    for (i, point1) in points.iter().enumerate() {
        for point2 in points.iter().skip(i + 1) {
            let area = ((point1.x - point2.x).abs() + 1) * ((point1.y - point2.y).abs() + 1);
            if area > largest_area && rectangle_is_inside(point1, point2, &segments) {
                largest_area = area;
            }
        }
    }
    largest_area.to_string()
}

fn rectangle_is_inside(p1: &Point, p2: &Point, segments: &Vec<(Point, Point)>) -> bool {
    let (min_x, max_x) = (p1.x.min(p2.x), p1.x.max(p2.x));
    let (min_y, max_y) = (p1.y.min(p2.y), p1.y.max(p2.y));

    // if any segment cuts into the rectangle it's partially outside, discard
    for (s1, s2) in segments {
        if s1.x == s2.x {  // vertical
            if s1.x > min_x && s1.x < max_x {
                let seg_y_min = s1.y.min(s2.y);
                let seg_y_max = s1.y.max(s2.y);
                if min_y.max(seg_y_min) < max_y.min(seg_y_max) {
                    return false;
                }
            }
        }
        else {  // horizontal
            if s1.y > min_y && s1.y < max_y {  //potential cut
                let seg_x_min = s1.x.min(s2.x);
                let seg_x_max = s1.x.max(s2.x);
                if min_x.max(seg_x_min) < max_x.min(seg_x_max) {
                    return false;
                }
            }
        }
    }
    //check so rectangle is inside, use center point and check how many segments are left of it
    let center_x = (p1.x + p2.x)/2;
    let center_y = (p1.y + p2.y)/2;
    segments.iter()
        .filter(|(s1, s2)| {s1.x == s2.x && s1.x < center_x})  // segment vertical and to the left of
        .filter(|(s1, s2)| {  // segment in same y-space
            let segment_y_min = s1.y.min(s2.y);
            let segment_y_max = s1.y.max(s2.y);
            center_y > segment_y_min && center_y < segment_y_max
        }).count() % 2 == 1
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        Point::new(x.parse().unwrap(), y.parse().unwrap())
    }).collect()
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point {x, y}
    }
}