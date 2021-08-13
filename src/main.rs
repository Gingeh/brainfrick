use brainfrick::BrainFuck;
use std::io::Read;
use std::{fs, io, path, process};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "brainfrick", about = "Rust implementation of Brainfuck")]
struct Opt {
    /// Path to program [optional: will use stdin instead]
    #[structopt()]
    path: Option<path::PathBuf>,

    /// Input given to program
    #[structopt(short, long, default_value = "")]
    input: String,

    /// Number of cells
    #[structopt(short, long, default_value = "30000")]
    size: usize,
}

fn main() {
    let opt = Opt::from_args();

    let mut program = String::new();
    match opt.path {
        Some(path) => {
            program = fs::read_to_string(path).expect("Failed to read file");
        }
        None => {
            io::stdin()
                .read_to_string(&mut program)
                .expect("Failed to get stdin");
        }
    }

    let mut engine = BrainFuck::new(opt.size, &opt.input, 10000);
    match engine.run(&program) {
        Ok(res) => println!("{}", res),
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1)
        }
    }
}
