use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("{}: wrong argument", args[0]);
        std::process::exit(1);
    }
 
    fs::hard_link(&args[1], &args[2]).expect(&args[1]);
}
