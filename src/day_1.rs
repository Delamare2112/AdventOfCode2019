use aoc_19::{get_args, Part};

fn calc_fuel_requirement(mass: usize) -> usize {
    (mass / 3).checked_sub(2).unwrap_or(0)
}

fn calc_fuel_requirement_of_fuel(mass: usize) -> usize {
    if mass > 0 {
        let cost = calc_fuel_requirement_of_fuel(calc_fuel_requirement(mass));
        mass + cost
    } else {
        mass
    }
}

fn part_1(input: &String) -> usize {
    input
        .lines()
        .map(|line| calc_fuel_requirement(line.parse().unwrap()))
        .sum()
}

fn part_2(input: &String) -> usize {
    input
        .lines()
        .map(|line| calc_fuel_requirement_of_fuel(calc_fuel_requirement(line.parse().unwrap())))
        .sum()
}

#[test]
fn simple_test_1() {
    assert_eq!(calc_fuel_requirement(1969), 654)
}

#[test]
fn simple_test_2() {
    assert_eq!(part_2(&String::from("100756")), 50346);
}

fn main() {
    let args = get_args();
    let solution = match args.part {
        Part::One => part_1(&args.input),
        Part::Two => part_2(&args.input),
    };
    println!("{}", solution);
}
