use std::io::Read;
use std::fs::File;
use std::env;

use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut skip = 1;
    let mut opts = Options::new();
    
    opts.optflag("T", "", "visualize tab");
    opts.optflag("E", "", "visualize new line");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e.to_string());
            std::process::exit(1);
        },
    };

    let tab: &str;
    let nline: &str;
    
    if matches.opt_present("T") {
        tab = "^I";
        skip += 1;
    } else {
        tab = "\t";
    }

    if matches.opt_present("E") {
        nline = "$\n";
        skip += 1;
    } else {
        nline = "\n";
    }

    for arg in env::args().skip(skip) {
        let f = File::open(arg).unwrap();
        for b in f.bytes() {
            match b {
                Ok(b'\t') => print!("{}", tab),
                Ok(b'\n') => print!("{}", nline),
                Ok(b) => print!("{}", b as char),
                Err(e) => std::process::exit(1),
            }
        }
    } 
}
