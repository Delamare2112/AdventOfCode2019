#![feature(unboxed_closures)]

use aoc_19::{get_args, Part};
use cgmath::num_traits::abs;
use cgmath::Point2;
use regex::Regex;
use std::collections::HashMap;

type Input = String;
type Output = usize;

fn part_1(input: &Input) -> Output {
    let mut map = HashMap::<isize, HashMap<isize, u8>>::new();
    let mut shortest = std::usize::MAX;
    let re = Regex::new(r"(?m)(.)(\d+)").expect("failed to compile regex!");
    for (i, line) in input.lines().map(|line| re.captures_iter(line)).enumerate() {
        let i = i + 1;
        let mut cursor = Point2 { x: 0_isize, y: 0 };
        for command in line {
            let direction = command[1].chars().next().unwrap();
            let amount = command[2].parse().unwrap();
            let mut action: Box<dyn FnMut<(), Output = Point2<isize>>> = match direction {
                'U' => Box::new(|| {
                    cursor.y += 1;
                    cursor.clone()
                }),
                'D' => Box::new(|| {
                    cursor.y -= 1;
                    cursor.clone()
                }),
                'R' => Box::new(|| {
                    cursor.x += 1;
                    cursor.clone()
                }),
                'L' => Box::new(|| {
                    cursor.x -= 1;
                    cursor.clone()
                }),
                _ => Box::new(|| cursor.clone()),
            };
            for _ in 0..amount {
                let cursor = action();
                let cell = map
                    .entry(cursor.x)
                    .or_default()
                    .entry(cursor.y)
                    .or_default();
                if *cell > 0 && *cell != i as u8 {
                    let distance = (abs(cursor.x) + abs(cursor.y)) as usize;
                    if shortest > distance {
                        shortest = distance;
                    }
                } else {
                    *cell = i as u8;
                }
            }
        }
    }
    shortest
}

fn part_2(input: &Input) -> Output {
    panic!("cannot find solution!")
}

#[test]
fn simple_test_2() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".to_string();
    assert_eq!(part_1(&input), 159);
}

#[test]
fn simple_test_3() {
    let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
        .to_string();
    assert_eq!(part_1(&input), 135);
}

#[test]
fn simple_test_1() {
    let input = "R8,U5,L5,D3\nU7,R6,D4,L4".to_string();
    assert_eq!(part_1(&input), 6);
}

fn main() {
    let args = get_args();
    let mut program = &args.input;
    let solution = match args.part {
        Part::One => part_1(&mut program),
        Part::Two => part_2(&mut program),
    };
    println!("{}", solution);
}
