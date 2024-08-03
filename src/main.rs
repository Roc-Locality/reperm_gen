use bimap::BiMap;
use clap::{Parser, Subcommand, ValueEnum};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reperm_gen::generator::gen::Generator;
use reperm_gen::generator::periodic::PeriodicGen;
use reperm_gen::group_theory::cycle::Cycle;
use reperm_gen::group_theory::group::Group;
use reperm_gen::group_theory::symmetric::sym;
use reperm_gen::locality::reuse::calculate_lru_hits;
use std::env;
use std::fs::File;
use std::io::{self, stdout, BufRead, Write};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;
use tracing::{event, Level};

#[derive(Debug, Clone, ValueEnum)]
enum LocalityCalculator {
    LRU
}

#[derive(Parser)]
#[command(
    name = "symmmetric locality", 
    about = "symmetric locality usage: ./<binary> <command> ; ./<binary> -h for help",
    version = "1.0",
    author = "giordan escalona"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Plot {
        #[arg(short, long, value_parser)]
        symmetric_n: usize,

        #[arg(short, long, value_parser)]
        locality_calculator: LocalityCalculator,

        #[arg(short, long, value_delimiter = ',')]
        cache_capacity_rankings: Vec<usize>
    }, 
    FindChain {
        #[arg(short, long, value_parser)]
        symmetric_n: usize,

        #[arg(short, long, value_parser)]
        locality_calculator: LocalityCalculator,

        #[arg(short, long)]
        start: Option<Vec<usize>>,
    },
    Simulate {
        #[arg(short, long, value_parser)]
        symmetric_n: usize,
    }
}

fn get_calc<V, O>(calc_enum: LocalityCalculator, rankings: &[usize]) -> Arc<dyn Fn(&Cycle<V>) -> O + Send + Sync> 
where 
    V: Clone + Copy + Hash + Eq + PartialEq + Debug + PartialOrd,
    O: PartialOrd + PartialEq + Ord + std::convert::From<Vec<usize>>
{
    let vec = Arc::new(rankings.to_vec());
    let func = match calc_enum {
        LocalityCalculator::LRU => move |cycle: &Cycle<V>| {
            let mut generator = PeriodicGen::new();
            generator.set_start(&cycle.get_ground());
            generator.add(cycle.get_function());
            vec.iter().map(|cs| {
                calculate_lru_hits(&generator.simulate(1), *cs)
            }).collect::<Vec<usize>>().into()
        },
    };
    Arc::new(func)
}

fn main() {
    let stdout = stdout();
    event!(Level::INFO, "Initialization has started");
    let cli = Cli::parse();
    match cli.command {
        Commands::Plot{symmetric_n, locality_calculator, cache_capacity_rankings} => {
            assert_ne!(cache_capacity_rankings.len(), 0, "Expected rankings to not be empty (Supply rankings with non empty elements)");
            assert!(cache_capacity_rankings.iter().max().unwrap() <= &symmetric_n);
            event!(Level::INFO, "Started trying to plot elements of the symmetric group");
            
            let group = sym(symmetric_n);
            let locality_calc: Arc<dyn Fn(&Cycle<usize>) -> Vec<usize> + Send + Sync> = get_calc(locality_calculator, &cache_capacity_rankings);
            let header = cache_capacity_rankings
                .iter()
                .fold(String::from("\"retraversal\","), |mut acc, x| {
                    acc.push_str(x.to_string().as_str());
                    acc.push_str(",");
                    acc
                });
            let mut set = group.get_set()
                .into_iter()
                .collect::<Vec<_>>();
            set.sort_unstable_by_key(|cycle| cycle.inversions());
            let set = set;
            let text: String = set
                .par_iter()
                .map(|retraversal| {
                    let calc = Arc::clone(&locality_calc);
                    (retraversal, calc(retraversal))
                })
                .map(|(retraversal, locality)| {
                    let locality_str = locality.iter().fold(String::new(), |mut acc, val| {
                        acc.push_str(val.to_string().as_str());
                        acc.push_str(",");
                        acc
                    });
                    let cycle_str: String = retraversal.get_ground()
                        .into_iter()
                        .map(|x| retraversal.eval(x))
                        .fold(String::new(), |mut acc, x| {
                            acc.push_str(x.to_string().as_str());
                            acc.push_str(",");
                            acc
                        });
                    
                    format!("\"{}\",{}\n", cycle_str, locality_str)
                })
                .collect();
            write!(stdout.lock(), "{}\n{}", header, text).expect("Something went wrong with writing to output")
        },
        Commands::FindChain { symmetric_n, locality_calculator, start } => todo!(),
        Commands::Simulate { symmetric_n } => todo!(),
    }
    /*
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
     */
}
