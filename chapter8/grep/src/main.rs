use std::io::Read;
use std::fs::File;
use std::env;
use std::process;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("no pattern");
        process::exit(1);
    }
    
    let pat = Regex::new(&args[1]).unwrap();

    if args.len() == 2 {
        do_grep(&pat, Box::new(std::io::stdin()));
    } else {
        for arg in &args[2..] {
            let f = File::open(arg).unwrap();
            do_grep(&pat, Box::new(f));
        }
    }
}

fn do_grep(pat: &Regex, mut src: Box<Read>) {
    let mut buf = String::new();
    src.read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        if pat.is_match(line) {
            println!("{}", line);
        }
    }
}

