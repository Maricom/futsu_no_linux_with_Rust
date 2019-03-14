use std::io::{stdin, stdout, Write};
use std::ffi::CString;

use nix::unistd::{fork, ForkResult, execv};
use nix::sys::wait::waitpid;

fn main() {
    let stdin = stdin();
    let mut buf = String::new();

    print_dollar();
    while let Ok(_) = stdin.read_line(&mut buf) {
        
        let args: Vec<CString> = buf.split_whitespace()
            .map(|a| CString::new(a).unwrap())
            .collect();

        match fork() {
            Ok(ForkResult::Child) => {
                if let Err(e) = execv(&args[0], &args) {
                    println!("{}", e);
                }
                break;
            },
            Ok(ForkResult::Parent { child }) => {
                waitpid(child, None);
            },
            Err(e) => println!("{}", e),
        }

        buf.clear();
        print_dollar();
    }
}

fn print_dollar() {
    let mut o = stdout();
    o.write("$ ".as_bytes());
    o.flush();
}
