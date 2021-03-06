use std::io::{stdin, stdout, Write};
use std::fs::File;
use std::ffi::CString;
use std::os::unix::io::{RawFd, IntoRawFd};

use nix::unistd::{close, pipe, dup2, fork, ForkResult, execv};
use nix::sys::wait::waitpid;

fn main() {
    let stdin = stdin();
    let mut buf = String::new();

    print_dollar();
    while let Ok(_) = stdin.read_line(&mut buf) {
        let mut tmp = buf.split(" > ");
        let commands: Vec<&str> = tmp.next().unwrap().split('|').collect(); 
        let mut redirect = None;

        if let Some(rd) = tmp.next() {
            let rd = rd.split_whitespace().next().unwrap();
            match File::create(rd) {
                Ok(f) => redirect = Some(f.into_raw_fd()),
                Err(e) => {
                    println!("{}", e);
                    continue;
                },
            }
        }
        
        let mut pids = Vec::new();
        let mut fds = Vec::new();
        for i in 0..commands.len() {
            fds.push(pipe().unwrap());
        }

        for i in 0..commands.len() { 
            let args: Vec<CString> = commands[i].split_whitespace()
                .map(|s| CString::new(s).unwrap())
                .collect();
            
            match fork() {
                Ok(ForkResult::Child) => {
                    if i > 0 {
                        close(0).unwrap();
                        dup2(fds[i-1].0, 0);
                    }
                    
                    if i < commands.len()-1 {
                        close(1).unwrap();
                        dup2(fds[i].1, 1); 
                    }
                    
                    if i == commands.len()-1 {
                        if let Some(rd) = redirect {
                            close(1).unwrap();
                            dup2(rd, 1);
                        }
                    }

                    close_fds(fds);

                    if let Err(e) = execv(&args[0], &args) {
                        println!("{}", e);
                    }
                    std::process::exit(1);
                },
                Ok(ForkResult::Parent { child }) => {
                    pids.push(child);
                },
                Err(e) => println!("{}", e),
            }
        }

        close_fds(fds);

        for pid in pids {
            waitpid(pid, None);
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

fn close_fds(fds: Vec<(RawFd, RawFd)>) {
    for (i, o) in fds {
        close(i).unwrap();
        close(o).unwrap();
    }
}
