extern crate rand;

mod eval;
mod ga;

use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

const FLAGS: [&'static str; 23] = [
    "--wrap-opkill",
    "--eliminate-dead-branches",
    "--merge-return",
    "--inline-entry-points-exhaustive",
    "--eliminate-dead-functions",
    "--eliminate-dead-code-aggressive",
    "--private-to-local",
    "--eliminate-local-single-block",
    "--eliminate-local-single-store",
    "--scalar-replacement=100",
    "--convert-local-access-chains",
    "--ssa-rewrite",
    "--ccp",
    "--loop-unroll",
    "--redundancy-elimination",
    "--combine-access-chains",
    "--simplify-instructions",
    "--vector-dce",
    "--eliminate-dead-inserts",
    "--if-conversion",
    "--copy-propagate-arrays",
    "--reduce-load-size",
    "--merge-blocks",
];
const ITERATION_COUNT: usize = 50;
const POPULATION_SIZE: usize = 20;
const INVALID_VALUE: u64 = 99999999999;

fn flip_coin(rng: &mut ThreadRng) -> bool {
    rng.gen()
}

fn shuffle<T>(rng: &mut ThreadRng, mut v: Vec<T>) -> Vec<T> {
    v.shuffle(rng);
    v
}

fn sleep() {
    std::thread::sleep(std::time::Duration::from_secs(10));
}

fn print_genes(genes: &Vec<ga::Gene>) {
    println!("    [");
    let mut itr = genes.iter().peekable();
    while let Some(n) = itr.next() {
        if itr.peek().is_none() {
            println!("      {}", n.to_json(&FLAGS));
        } else {
            println!("      {},", n.to_json(&FLAGS));
        }
    }
    print!("    ]");
}

fn get_flags(code: &Vec<usize>) -> Vec<String> {
    code.iter().map(|n| FLAGS[*n].to_string()).collect()
}

fn run_non() -> u64 {
    eval::eval(&Vec::new()).unwrap_or(INVALID_VALUE)
}

fn run_o() -> u64 {
    eval::eval(&Vec::from([String::from("-O")])).unwrap_or(INVALID_VALUE)
}

fn run_here() -> u64 {
    eval::measure().unwrap_or(INVALID_VALUE)
}

fn run() {
    // create objects
    let items = (0..FLAGS.len()).collect::<Vec<usize>>();
    let mut rng = rand::thread_rng();
    let mut genes = Vec::new();

    println!("{{");
    println!("  \"generations\": [");

    // create the initial population
    for _ in 0..POPULATION_SIZE {
        let indices = items
            .clone()
            .into_iter()
            .filter(|_| flip_coin(&mut rng))
            .collect::<Vec<usize>>();
        let code = shuffle(&mut rng, indices);
        let value = eval::eval(&get_flags(&code)).unwrap_or(INVALID_VALUE);
        genes.push(ga::Gene { code, value });
        sleep();
    }

    print_genes(&genes);
    println!(",");

    // run Genetic Algorithm
    for i in 0..ITERATION_COUNT {
        for code in ga::crossover(&mut rng, &genes, &items) {
            let value = eval::eval(&get_flags(&code)).unwrap_or(INVALID_VALUE);
            genes.push(ga::Gene { code, value });
            sleep();
        }

        genes = ga::select(&mut rng, &genes, POPULATION_SIZE);
        print_genes(&genes);
        if i < ITERATION_COUNT - 1 {
            println!(",");
        }
    }

    println!("  ],");

    println!("  \"non\": {},", run_non());
    sleep();
    println!("  \"-O\": {}", run_o());

    println!("}}");
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        run();
    } else if args[1] == "non" {
        println!("{}", run_non());
    } else if args[1] == "-O" {
        println!("{}", run_o());
    } else if args[1] == "." {
        println!("{}", run_here());
    }
}
