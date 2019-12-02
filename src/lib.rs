use std::io::Read;

pub enum Part {
    One,
    Two,
}

pub struct Args {
    pub input: String,
    pub part: Part,
}

pub fn get_args() -> Args {
    let matches = clap::App::new("AoC 19")
        .arg(
            clap::Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("PATH")
                .help("path to the input file")
                .default_value("input.txt"),
        )
        .arg(
            clap::Arg::with_name("part")
                .short("p")
                .long("part")
                .value_name("UNSIGNED")
                .help("the part of the day to solve for")
                .possible_values(&["1", "2"])
                .default_value("1"),
        )
        .get_matches();
    let mut input = String::new();
    std::fs::File::open(matches.value_of("input").unwrap())
        .expect("Failed to open input file!")
        .read_to_string(&mut input)
        .expect("Failed to read input file!");
    Args {
        input,
        part: match matches.value_of("part").unwrap() {
            "1" => Part::One,
            "2" => Part::Two,
            _ => panic!("part was neither 1 nor 2!"),
        },
    }
}
