use std::io::Read;
use std::fs::File;
use std::env;
fn main() {
    for arg in env::args().skip(1) {
        let f = File::open(arg).unwrap();
        for b in f.bytes() {
            match b {
                Ok(b'\t') => print!("\\t"),
                Ok(b'\n') => print!("$\n"),
                Ok(b) => print!("{}", b as char),
                Err(e) => std::process::exit(1),
            }
        }
    } 
}
