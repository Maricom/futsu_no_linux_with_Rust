use std::env;
use std::fs::read_dir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}: no arguments", args[0]);
        std::process::exit(1);
    }
    
    for arg in &args[1..] {
        do_ls(arg);
    }
}

fn do_ls(path: &str) {
    let dir = read_dir(path).expect(path);
    for entry in dir {
        println!("{:?}", entry.unwrap().file_name());
    }
}
    
