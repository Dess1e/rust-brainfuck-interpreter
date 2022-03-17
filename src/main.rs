mod interp;
mod preproc;
mod util;

use std::fs;
use structopt::StructOpt;

use interp::*;
use preproc::preprocess;
use util::read_file;

#[derive(Debug, StructOpt)]
#[structopt(name = "BF interpreter",)]
struct Opt {
    #[structopt(short = "f", long)]
    file: String,
    #[structopt(short, long, default_value = "30000")]
    cell_array_size: usize,
    #[structopt(short, long)]
    auto_flush_stdout: bool,
}

fn main() {
    let opt = Opt::from_args();
    let raw_code = read_file(&opt.file);

    Interpreter::new(
        preprocess(&raw_code),
        opt.cell_array_size,
        opt.auto_flush_stdout,
    ).main_loop();
}
