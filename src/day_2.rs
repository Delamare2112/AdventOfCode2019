use aoc_19::{get_args, Part};

fn parse_input(input: &String) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

fn execute_program(input: &mut Vec<usize>) {
    let mut pc = 0;
    let size = input.len();
    while pc < size {
        let opcode = input[pc];
        if opcode == 99 {
            break;
        }
        let a = input[input[pc + 1]];
        let b = input[input[pc + 2]];
        let dst = input[pc + 3];
        input[dst] = match opcode {
            1 => a + b,
            2 => a * b,
            _ => panic!("Something went wrong!"),
        };
        pc += 4;
    }
}

fn part_1(input: &mut Vec<usize>) -> usize {
    input[1] = 12;
    input[2] = 2;
    execute_program(input);
    input[0]
}

fn part_2(input: &mut Vec<usize>) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = input.clone();
            program[1] = noun;
            program[2] = verb;
            execute_program(&mut program);
            if program[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("cannot find solution!")
}

#[test]
fn simple_test_1() {
    let mut program = parse_input(&String::from("1,9,10,3,2,3,11,0,99,30,40,50"));
    execute_program(&mut program);
    assert_eq!(program[0], 3500);
}

fn main() {
    let args = get_args();
    let mut program = parse_input(&args.input);
    let solution = match args.part {
        Part::One => part_1(&mut program),
        Part::Two => part_2(&mut program),
    };
    println!("{}", solution);
}
