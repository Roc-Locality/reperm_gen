use bimap::BiMap;
use reperm_gen::generator::gen::Generator;
use reperm_gen::generator::periodic::PeriodicGen;
use reperm_gen::group_theory::cycle::Cycle;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let file = File::open(file_path).unwrap();
    let buf = io::BufReader::new(file);
    let mut lines = buf.lines();
    let parameter_number: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Should have been able to read the parameter number");
    let mut ground: Vec<String> = Vec::new();
    let mut permutation: BiMap<String, String> = BiMap::new();
    for _ in 0..parameter_number {
        let line = lines.next().unwrap().unwrap();
        permutation.insert(line.clone(), line.clone());
        ground.push(line);
    }
    let mut generator: PeriodicGen<String> = PeriodicGen::new();
    generator.set_start(&ground);
    let permutation_number: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Should have been able to read the permutation number");
    for _ in 0..permutation_number {
        let line = lines.next().unwrap().unwrap();
        let space_index = line.find(' ').unwrap_or(line.len());
        let (input, output) = line.split_at(space_index);
        permutation.insert(input.trim().to_owned(), output.trim().to_owned());
    }
    let cycle = Cycle::new(permutation, ground.clone());
    println!("{cycle}");
    let func = cycle.get_function();
    generator.add(func);
    let simulation_number: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("Should have been able to read the simulation number");
    println!("{:?}", generator.simulate(simulation_number));
}
