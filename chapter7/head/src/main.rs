use std::io::Read;
use std::fs::File;
use std::env;
use std::process;

use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut nlines: u64 = 10;
    let mut start_idx = 1;
    let mut opts = Options::new();

    opts.optflag("n", "lines", "set the number of lines");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e.to_string());
            print_usage(opts, &args[0]);
            process::exit(1);
        }, 
    };

    if matches.opt_present("h") {
        print_usage(opts, &args[0]);
        return;
    }

    if let Some(n) = matches.opt_str("n") {
        nlines = n.parse().unwrap();
        start_idx += 2;
    }
 
    if start_idx == args.len() {
        do_head(Box::new(std::io::stdin()), nlines);
    } else {
        for arg in &args[start_idx..] {
            do_head(Box::new(File::open(arg).unwrap()), nlines);
        }
    }
}

fn do_head(f: Box<Read>, mut nlines: u64) {
    for b in f.bytes() {
        if nlines == 0 {
            return;
        }
        if let Ok(b) = b { 
            if b == b'\n' {
                nlines -= 1;
            }
            print!("{}", b as char);
        } else {
            process::exit(1);
        }
    }
}

fn print_usage(opts: Options, name: &str) {
    let brief = format!("Usage: {} [-n LINES] [FILE ...]", name);
    println!("{}", opts.usage(&brief));
} 

