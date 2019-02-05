use std::io::{self, Read};
use std::fs::File;
use std::env;

const BUFFER_SIZE: usize = 2048;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", count_lines(None).unwrap());
    } else {
        let mut total = 0;
        for arg in &args[1..] {
            match count_lines(Some(arg)) {
                Ok(cnt) => {
                    total += cnt; 
                    println!("{} {}", cnt, arg)
                },
                Err(err) => println!("wc: {}: {}", arg, err)
            }
        }
        println!("{} total", total);
    }
}

fn count_lines(path: Option<&str>) -> io::Result<u32> {
    let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut s: Box<Read>;
    
    if let Some(path) = path {
        s = Box::new(File::open(path)?);
    } else {
        s = Box::new(io::stdin());
    }
    
    let mut cnt = 0; 
    loop {
        let n = s.read(&mut buf)?;
        if n == 0 {
            break;
        }
        cnt += buf.iter()
            .fold(0, |cnt, &c| if c == b'\n' { cnt + 1 } else { cnt });
    } 
    Ok(cnt)
}
