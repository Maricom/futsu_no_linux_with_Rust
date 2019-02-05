use std::io::{self, Write, Read};
use std::fs::File;
use std::env;
use std::process;

const BUFFER_SIZE: usize = 2048;

fn main() {
   let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 {
        io::stderr().write(&format!("{}: file name not given\n", args[0]).into_bytes());
        process::exit(1);
    }
    for arg in &args[1..] {
        do_cat(arg);
    }
    process::exit(0);
}

fn do_cat(path: &str) -> io::Result<()> {
    let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut f = File::open(path)?;
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        io::stdout().write(&buf[..n])?;
    }
    Ok(())
}
