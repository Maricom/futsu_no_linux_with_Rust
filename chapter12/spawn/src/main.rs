use std::env;
use std::process;
use std::ffi::CString;

use nix::unistd::{fork, ForkResult, execv};
use nix::sys::wait::{waitpid, WaitStatus};

fn main() {
    let args: Vec<CString> = env::args()
        .map(|a| CString::new(a).unwrap())
        .collect();
    
    if args.len() != 3 {
        println!("Usage: {} <command> <arg>", args[0].clone().into_string().unwrap());
        process::exit(1);
    }

    match fork() {
        Ok(ForkResult::Child) => {
            if let Err(e) = execv(&args[1], &args[1..]) {
                println!("{}", e);
                process::exit(99); 
            } 
        },
        Ok(ForkResult::Parent { child }) => {
            let status = waitpid(child, None).unwrap();
            
            println!("\nchild (PID={}) finished;", child);
            
            match status {
                WaitStatus::Exited(_, st) => println!("exit, status={}", st),
                WaitStatus::Signaled(_, st, _) => println!("signal, sig={}", st),
                _ => println!("abnormal exit"),
            }
        },
        Err(e) => println!("{}", e),    
    }
}

