use std::io::Read;
use std::fs::File;
use std::env;
use std::process;

use regex::Regex;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("i", "", "ignoring case");
    opts.optflag("v", "", "print NOT matched line");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e.to_string());
            process::exit(1);
        },
    };

    let pat_idx = matches.opt_count("i") + matches.opt_count("v") + 1;

    if args.len() <= pat_idx {
        println!("no pattern");
        process::exit(1);
    }

    let pat_s = if matches.opt_present("i") {
        format!("(?i)({})", args[pat_idx])
    } else {
        args[pat_idx].to_string()
    };
    
    let v_opt = if  matches.opt_present("v") {
        true
    } else {
        false
    };

    let pat = Regex::new(&pat_s).unwrap();

    if args.len() == pat_idx+1 {
        do_grep(&pat, Box::new(std::io::stdin()), v_opt);
    } else {
        for arg in &args[pat_idx+1..] {
            let f = File::open(arg).unwrap();
            do_grep(&pat, Box::new(f), v_opt);
        }
    }
}

fn do_grep(pat: &Regex, mut src: Box<Read>, v_opt: bool) {
    let mut buf = String::new();
    src.read_to_string(&mut buf).unwrap();

    for line in buf.lines() {
        if pat.is_match(line) ^ v_opt {
            println!("{}", line);
        }
    }
}

