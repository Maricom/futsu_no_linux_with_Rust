use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;

use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opt = Options::new();
    opt.optflag("p", "", "make parent directories as needed");
    
    if args.len() < 2 {
        println!("{}: no argument", args[0]);
        std::process::exit(1);
    }

    let matches = opt.parse(&args[1..]).expect("error");
    let p_flag = matches.opt_present("p");
    let start_idx = if p_flag { 2 } else { 1 };

    for arg in &args[start_idx..] {
        if p_flag {
            // using fs::create_dir_all() is more excellent way.
            let mut pathbuf = PathBuf::new();
            let dirs = arg.split("/");

            for dir in dirs {
                pathbuf.push(dir);
                let path = pathbuf.as_path();
                if !path.exists() {
                    make(path).unwrap();
                }
            }

        } else {
            make(Path::new(arg)).unwrap();
        }
    }
}

fn make(path: &Path) -> std::io::Result<()> {
    fs::create_dir(path)?;
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755); 
    fs::set_permissions(path, permissions)?;

    Ok(())
}

