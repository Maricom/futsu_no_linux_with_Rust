use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}: no argument", args[0]);
        std::process::exit(1);
    }

    for arg in &args[1..] {
        fs::create_dir(arg).expect(arg);
        let mut permissions = fs::metadata(arg).unwrap().permissions();
        permissions.set_mode(0o755); 
        fs::set_permissions(arg, permissions).unwrap();
    }
}
