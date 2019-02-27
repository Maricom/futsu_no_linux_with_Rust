use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}: no argument", args[0]);
        std::process::exit(1);
    }

    for arg in &args[1..] {
        fs::remove_file(arg).expect(arg);
    }
}
