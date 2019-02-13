use std::io::Read;
use std::fs::File;
use std::env;

fn main() {
    let nlines = 5;
    for arg in env::args().skip(1) {
        let mut buf = String::new();
        let mut f = File::open(arg).unwrap();
      
        f.read_to_string(&mut buf).unwrap();
        let lines: Vec<&str> = buf.lines().collect();
        
        for line in &lines[lines.len()-nlines..] {
            println!("{}", line);
        }
    }
}
