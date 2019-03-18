use nix::sys::signal::{Signal, sigaction, SigAction, SigHandler, SaFlags, SigSet};
use nix::unistd::pause;

fn main() {
    let sgact: SigAction = SigAction::new(
        SigHandler::Handler(handler), 
        SaFlags::SA_RESETHAND,
        SigSet::empty()
    ); 

    unsafe { sigaction(Signal::SIGINT, &sgact) }.unwrap();

    pause();
}

extern "C" fn handler(_sig: i32) {
    println!("GoodBye World!");
    std::process::exit(0);
}
