use std::io::{self, Write, Read};
use std::fs::File;
use std::env;
use std::process;

const BUFFER_SIZE: usize = 2048;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 {
        do_cat(None);
    } else {
        for arg in &args[1..] {
            do_cat(Some(arg)).unwrap_or_else(die);
        }
    }
    process::exit(0);
}

fn do_cat(path: Option<&str>) -> io::Result<()> {
    let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut s: Box<Read>;

    if let Some(path) = path {
        s = Box::new(File::open(path)?);
    } else {
        s = Box::new(io::stdin());
    }

    loop {
        let n = s.read(&mut buf)?;
        if n == 0 {
            break;
        }
        io::stdout().write(&buf[..n])?;
    }
    Ok(())
}

fn die(e: io::Error) {
    io::stderr().write(&format!("{}\n", e).into_bytes());
    process::exit(1);
}
