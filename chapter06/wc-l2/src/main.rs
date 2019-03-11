use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    for arg in env::args().skip(1) {
        let mut buf = String::new();
        match File::open(&arg) {
            Ok(mut f) => {
                f.read_to_string(&mut buf).unwrap();
                println!("{} {}", buf.lines().count(), arg);
            },
            Err(e) => {
                println!("wc: {}: {}", arg, e);
            },
        }
    }
}
