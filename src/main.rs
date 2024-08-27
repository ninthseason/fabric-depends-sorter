use clap::Parser;
use std::path;

mod mod_t;
mod graph;
mod logic;

#[derive(Parser)]
#[command(version, about)]
struct Opt {
    #[arg(help = "mods directory")]
    directory: path::PathBuf,

    #[arg(short = 'x', long, default_value_t = 1, help = "Number of threads")]
    threads: usize,

    #[arg(short, long, default_value = "[]", help = "Ignore mods")]
    ignores: Vec<String>,
}

fn main() {
    let opt = Opt::parse();
    std::process::exit(logic::real_main(opt));
}
