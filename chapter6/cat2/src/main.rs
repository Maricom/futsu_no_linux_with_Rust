use std::io::{Read, Write};
use std::fs::File;
use std::env;
use std::process;

fn main() {
    for arg in env::args().skip(1) {
        let f = File::open(arg).unwrap(); 
        for b in f.bytes() {
            print!("{}", b.unwrap() as char);
        }
    }
}
