use abstract_cache::{CacheSim, ObjIdTraits};
use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reperm_gen::chain_find;
use reperm_gen::generator::gen::Generator;
use reperm_gen::generator::periodic::PeriodicGen;
use reperm_gen::group_theory::cycle::Cycle;
use reperm_gen::group_theory::group::Group;
use reperm_gen::group_theory::symmetric::sym;
use reperm_gen::locality::reuse::calculate_lru_hits;
use serde_json::json;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::sync::Arc;
use tracing::{event, Level};

struct MockCache;

impl<Obj: ObjIdTraits> CacheSim<Obj> for MockCache {
    fn cache_access(&mut self, _: Obj) -> abstract_cache::AccessResult {
        todo!()
    }

    fn set_capacity(&mut self, _: usize) -> &mut Self {
        todo!()
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, ValueEnum)]
enum LocalityCalculator {
    LRU,
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
        cache_capacity_rankings: Vec<usize>,

        #[arg(short = 'z', long, action = clap::ArgAction::SetTrue, default_value_t = false)]
        sorted: bool,

        #[arg(short = 'o', long = "output", value_hint = ValueHint::FilePath)]
        output_file: Option<String>,
    },
    FindChain {
        #[arg(short, long, value_parser)]
        symmetric_n: usize,

        #[arg(short, long, value_parser)]
        locality_calculator: LocalityCalculator,

        #[arg(short, long, value_delimiter = ',')]
        cache_capacity_rankings: Vec<usize>,

        #[arg(short = 'x', long, value_delimiter = ',')]
        start: Option<Vec<usize>>,

        #[arg(short, long, default_value_t = usize::MAX)]
        max_length: usize,

        #[arg(short = 'o', long = "output", value_hint = ValueHint::FilePath)]
        output_file: Option<String>,
    },
    Simulate {
        #[arg(short, long, value_parser)]
        symmetric_n: usize,
    },
}

type LocalityRanker<V, O> = dyn Fn(&Cycle<V>) -> O + Send + Sync;
fn get_calc<V, O>(
    calc_enum: &LocalityCalculator,
    rankings: Arc<Vec<usize>>,
) -> Box<LocalityRanker<V, O>>
where
    V: ObjIdTraits + Clone + Copy + Hash + Eq + PartialEq + Debug + PartialOrd + Sync,
    O: PartialOrd + PartialEq + std::convert::From<Vec<f32>>,
{
    let func = match calc_enum {
        LocalityCalculator::LRU => move |cycle: &Cycle<V>| {
            let mut generator = PeriodicGen::new();
            generator.set_start(&cycle.get_ground());
            generator.add(cycle.get_function());
            let simulated = generator.simulate(1);

            let mut v: Vec<f32> = rankings
                .par_iter()
                .map(|cs| calculate_lru_hits(&simulated, *cs) as f32)
                .collect::<Vec<f32>>()
                .into();
            // CODE SMELL!
            let mock = MockCache {};
            //let z = 4; //&simulated.len() - 3;
            let fp = mock.footprint(simulated.into_iter());
            //let mark = fp.get(z).unwrap();
            v.extend(fp.iter());
            // CODE SMELL!
            v.into()
        },
    };
    Box::new(func)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    event!(Level::INFO, "Initialization has started");
    let cli = Cli::parse();
    match cli.command {
        Commands::Plot {
            symmetric_n,
            locality_calculator,
            cache_capacity_rankings,
            sorted,
            output_file,
        } => {
            assert_ne!(
                cache_capacity_rankings.len(),
                0,
                "Expected rankings to not be empty (Supply rankings with non empty elements)"
            );
            assert!(cache_capacity_rankings.iter().max().unwrap() <= &symmetric_n);
            event!(
                Level::INFO,
                "Started trying to plot elements of the symmetric group"
            );
            let mut file = if let Some(o) = output_file {
                File::create(o)?
            } else {
                File::create("./output")?
            };

            let cache_capacity_rankings = Arc::new(cache_capacity_rankings);
            let clone = Arc::clone(&cache_capacity_rankings);
            let group = sym(symmetric_n);
            let locality_calc: Box<LocalityRanker<usize, Vec<f32>>> =
                get_calc(&locality_calculator, clone);
            let retraversal_header = String::from("\"inversions\",\"retraversal\",");
            let ranking_header = cache_capacity_rankings
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
            let header = retraversal_header + &ranking_header;
            let set = if sorted {
                let mut s = group.get_set().into_iter().collect::<Vec<_>>();
                s.sort_unstable_by_key(|cycle| cycle.inversions());
                s
            } else {
                group.get_set().into_iter().collect::<Vec<_>>()
            };
            //let set = group.get_set();
            let text: String = set
                .par_iter()
                .map(|retraversal| (retraversal, locality_calc(retraversal)))
                .map(|(retraversal, locality)| {
                    let locality_str = locality
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(",");
                    let cycle_str: String= retraversal.get_retraversal_str();
 
                    format!("{},\"{}\",{}\n", retraversal.inversions(), cycle_str, locality_str)
                })
                .collect();
            let out = format!("{}\n{}", header, text);
            file.write_all(out.as_bytes())?
        }
        Commands::FindChain {
            symmetric_n,
            locality_calculator,
            cache_capacity_rankings,
            start,
            max_length,
            output_file,
        } => {
            assert_ne!(
                cache_capacity_rankings.len(),
                0,
                "Expected rankings to not be empty (Supply rankings with non empty elements)"
            );
            assert!(cache_capacity_rankings.iter().max().unwrap() <= &symmetric_n);
            let mut file = if let Some(o) = output_file {
                File::create(o)?
            } else {
                File::create("./output")?
            };
            let group = sym(symmetric_n);
            let starting = if let Some(s) = start {
                group.create_retraversal(&s)
            } else {
                group.identity()
            };

            let cache_capacity_rankings = Arc::new(cache_capacity_rankings);
            let clone_1 = Arc::clone(&cache_capacity_rankings);
            let clone_2 = Arc::clone(&cache_capacity_rankings);
            let locality_calc: Box<LocalityRanker<usize, Vec<f32>>> =
                get_calc(&locality_calculator, clone_1);
            let chain_result = chain_find(&group, starting, locality_calc, max_length);
            let chain = &chain_result.chain;
            let locality_calc_2: Box<LocalityRanker<usize, Vec<f32>>> =
                get_calc(&locality_calculator, clone_2);
            let retraversal_iter = chain
                .par_iter()
                .map(|x| {
                    let b = locality_calc_2(x);
                    (x, b)
                })
                .map(|(a, b)| {
                    let retraversal = a.get_retraversal_str();

                    let locality_str = b
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(",");
                    format!("\"{}\",{}", retraversal, locality_str)
                });
            let retraversal_header = String::from("\"retraversal\",");
            let ranking_header = cache_capacity_rankings
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",");
            let header = retraversal_header + &ranking_header;
            let output: String = retraversal_iter.collect::<Vec<String>>().join("\n");

            let data = json!({
                "chain_data": chain_result,
                "raw_data": format!("{}\n{}", header, output),
            });

            let serialized = serde_json::to_string_pretty(&data).unwrap();
            file.write_all(serialized.as_bytes())?;
        }
        Commands::Simulate { .. } => todo!(),
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
    Ok(())
}
