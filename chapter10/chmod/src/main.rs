use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let args: Vec<String> = env::args().collect();
  
    if args.len() < 2 {
        println!("no mode given");
        std::process::exit(1);
    }

    let mode = u32::from_str_radix(&args[1], 8).expect("please input octal number");
    for arg in &args[2..] {
        let mut perms = fs::metadata(arg).unwrap().permissions();
        perms.set_mode(mode);
        fs::set_permissions(arg, perms).expect(arg);
    }
}
