use std::env;
use std::fs::read_dir;
use std::path::Path;

use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("R", "", "recursively list sub directories");

    if args.len() < 2 {
        println!("{}: no arguments", args[0]);
        std::process::exit(1);
    }
    
    let matches = opts.parse(&args[1..]).unwrap();
    let r_flag = matches.opt_present("R");
    let start_idx = if r_flag { 2 } else { 1 };

    if args.len() <= start_idx {
        println!("{}: no arguments", args[0]);
        std::process::exit(1);
    }

    for arg in &args[start_idx..] {
        println!("{}", arg);
        do_ls(Path::new(arg), 0, r_flag);
    }
}

fn do_ls(path: &Path, layer: usize, r_flag: bool) {
    let dir = read_dir(path).expect("invalid path");
    for entry in dir {
        let entry = entry.unwrap();
        println!("{}|---{:?}", "|  ".repeat(layer), entry.file_name());
        if r_flag && entry.file_type().unwrap().is_dir() {
            do_ls(path.join(entry.file_name()).as_path(), layer+1, r_flag); 
        }
    }
}
    
