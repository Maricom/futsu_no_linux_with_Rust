use std::env;
use std::fs::File;
use std::io::Read;

use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}: no argument", args[0]);
        std::process::exit(1);
    }

    let mut opts = Options::new();
    opts.optopt("n", "", "n rows", "-n N_ROWS");
    
    let mut n_lines = 10;
    let mut start_idx = 1;
    
    let matches = opts.parse(&args[1..]).unwrap(); 
    if matches.opt_present("n") {
        let n = matches.opt_str("n").expect("error: usage: -n N_LINES"); 
        n_lines = n.parse().unwrap();
        start_idx += 2;
    } 
    
    if args.len() < start_idx + 1 {
        print_tail(Box::new(std::io::stdin()), n_lines);
    } else {
        for arg in &args[start_idx..] {
            let f = File::open(arg).expect("error");
            print_tail(Box::new(f), n_lines);
        }
    }
}

fn print_tail(mut f: Box<Read>, n_lines: usize) {
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let lines: Vec<&str> = buf.lines().collect();
    
    for line in &lines[lines.len().saturating_sub(n_lines)..] {
        println!("{}", line);
    }
}
