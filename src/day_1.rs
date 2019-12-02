use aoc_19::{get_args, Part};

fn calc_fuel_requirement(mass: usize) -> usize {
    (mass / 3) - 2
}

#[test]
fn simple_test_1() {
    assert_eq!(calc_fuel_requirement(1969), 654)
}

fn part_1(input: &String) -> usize {
    input
        .lines()
        .map(|line| calc_fuel_requirement(line.parse().unwrap()))
        .sum()
}

fn part_2(input: &String) -> usize {
    unimplemented!()
}

fn main() {
    let args = get_args();
    let solution = match args.part {
        Part::One => part_1(&args.input),
        Part::Two => part_2(&args.input),
    };
    println!("{}", solution);
}
