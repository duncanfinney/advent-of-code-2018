pub fn solve() {

    let input = include_str!("../input/day06");
    let input = parse_input(input);

    let answer = part_one(&input);
    println!("part_one= {:?}", answer);

}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            let parts = l
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect::<Vec<_>>();
            Point(parts[0], parts[1])
        })
        .collect()
}

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn distance_to(&self, other: &Point) -> i32 {
        i32::abs(other.0 - self.0) + i32::abs(other.1 - self.1)
    }
}

fn part_one(input :&Vec<Point>) {
    let min_x = input.iter().map(|p | p.0).min().unwrap();
    let max_x = input.iter().map(|p | p.0).max().unwrap();
    let min_y = input.iter().map(|p | p.1).min().unwrap();
    let max_y = input.iter().map(|p | p.1).max().unwrap();

    for y in min_y..(2*max_y) {
        for x in min_x..(2*max_x) {
            let this_point = Point(x, y);
            let closest = input.iter().min_by_key(|p| p.distance_to(&this_point));
            println!("closest={:?}", closest);
        }
    }
}

fn part_two() {}
