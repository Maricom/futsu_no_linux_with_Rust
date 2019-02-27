use std::env;
use std::fs::{self, FileType};
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::FileTypeExt;

use chrono::naive::NaiveDateTime;

const S_IFMT: u32 = 0o170000;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("wrong argument");
        std::process::exit(1);
    }

    let meta = fs::metadata(&args[1]).expect(&args[1]);
    println!("type\t{} ({})", meta.st_mode() & S_IFMT, filetype(meta.file_type())); 
    println!("mode\t{}", meta.st_mode() & !S_IFMT);
    println!("dev\t{}", meta.st_dev());
    println!("ino\t{}", meta.st_ino());
    println!("rdev\t{}", meta.st_rdev());
    println!("nlink\t{}", meta.st_nlink());
    println!("uid\t{}", meta.st_uid());
    println!("gid\t{}", meta.st_gid());
    println!("size\t{}", meta.st_size());
    println!("blksize\t{}", meta.st_blksize());
    println!("blocks\t{}", meta.st_blocks());
    println!("atime\t{}", NaiveDateTime::from_timestamp(meta.st_atime(), 0));
    println!("mtime\t{}", NaiveDateTime::from_timestamp(meta.st_mtime(), 0));
    println!("ctime\t{}", NaiveDateTime::from_timestamp(meta.st_ctime(), 0));
}

fn filetype(ft: FileType) -> &'static str {
    if ft.is_file() {
        return "file";
    }
    if ft.is_dir() {
        return "directory";
    }
    if ft.is_char_device() {
        return "chardev";
    }
    if ft.is_block_device() {
        return "blockdev";
    }
    if ft.is_fifo() {
        return "fifo";
    }
    if ft.is_symlink() {
        return "symlink";
    }
    if ft.is_socket() {
        return "socket";
    } 
    
    "unknown"
}
