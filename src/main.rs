mod interp;
mod preproc;
mod util;

use std::process::exit;
use std::sync::atomic::Ordering;
use std::time::SystemTime;
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

    let start_time = SystemTime::now();

    let raw_code = read_file(&opt.file);
    let mut interpreter = Interpreter::new(
        preprocess(&raw_code),
        opt.cell_array_size,
        opt.auto_flush_stdout,
    );

    let counter_ref = interpreter.executed_instr_count.clone();

    ctrlc::set_handler(move | | {
        let secs = SystemTime::now().duration_since(start_time).unwrap().as_secs_f64();
        println!("\nInstruction / Second rate: {:4}", counter_ref.load(Ordering::SeqCst) as f64 / secs);
        exit(0);
    }).expect("Error setting ctrl-c handler");

    interpreter.main_loop();
}
