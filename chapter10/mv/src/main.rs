use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("{}: wrong number of arguments", args[0]);
        std::process::exit(1);
    }

    fs::rename(&args[1], &args[2]).expect(&args[1]);
}
