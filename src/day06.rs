use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

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

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn distance_to(&self, other: &Point) -> i32 {
        i32::abs(other.0 - self.0) + i32::abs(other.1 - self.1)
    }

    fn get_closest_point(&self, points: &Vec<Point>) -> Option<Point> {
        let min = points.iter().map(|p| { (p, p.distance_to(self)) }).min_by_key(|p| { p.1 }).unwrap();
        if points.iter().any(|p| { p != min.0 && p.distance_to(self) == min.1 }) {
            None
        } else {
            Some(min.0.to_owned())
        }
    }
}

fn part_one(input: &Vec<Point>) -> i32 {
    let min_x = input.iter().map(|p| p.0).min().unwrap();
    let max_x = input.iter().map(|p| p.0).max().unwrap();
    let min_y = input.iter().map(|p| p.1).min().unwrap();
    let max_y = input.iter().map(|p| p.1).max().unwrap();

    let height_y = max_y - min_y;
    let width_x = max_x - min_x;

    let mut area_counts = HashMap::new();
    for y in min_y - height_y..max_y + height_y {
        for x in min_x - width_x..max_x + width_x {
            let this_point = Point(x, y);
            let closest = this_point.get_closest_point(input);
            if let Some(closest) = closest {
                *area_counts.entry(closest.to_owned()).or_insert(0) += 1;
            }
        }
    }

    //inf areas
    let mut inf_areas = HashSet::new();
    for x in min_x - width_x..max_x + width_x {
        let this_point = Point(x, min_y - height_y);
        let closest = input
            .iter()
            .min_by_key(|p| p.distance_to(&this_point))
            .unwrap();
        inf_areas.insert(closest);

        let this_point = Point(x, max_y + height_y);
        let closest = input
            .iter()
            .min_by_key(|p| p.distance_to(&this_point))
            .unwrap();
        inf_areas.insert(closest);
    }

    for y in min_y - height_y..max_y + height_y {
        let this_point = Point(min_x - width_x, y);
        let closest = input
            .iter()
            .min_by_key(|p| p.distance_to(&this_point))
            .unwrap();
        inf_areas.insert(closest);

        let this_point = Point(max_x + width_x, y);
        let closest = input
            .iter()
            .min_by_key(|p| p.distance_to(&this_point))
            .unwrap();
        inf_areas.insert(closest);
    }

    let mut area_counts = area_counts
        .iter()
        .filter(|p| !inf_areas.contains(p.0))
        .collect::<Vec<_>>();
    area_counts.sort_by_key(|p| -1 * p.1);
    *area_counts[0].1
}

fn part_two() {}
