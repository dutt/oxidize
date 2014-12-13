 #![feature(slicing_syntax)]

extern crate libc;

use std::os;
use std::io::{
    Command,
    process,
};
use std::path::GenericPath;
use std::io::fs::PathExtensions;

mod compiler;

fn has_cache(script_path: &str, cache_path : &str) -> bool {
    let scriptp = Path::new(script_path);
    let cachep = Path::new(cache_path);  
    if !scriptp.exists() || !scriptp.is_file() {
        panic!("Can't read script");
    }
    let curr = match scriptp.stat() {
        Ok(stat) => stat,
        Err(_) => { return false },
    };
    if  !cachep.exists() || !cachep.is_file() {
        return false;
    }
    let cached = match cachep.stat() {
        Ok(stat) => stat,
        Err(_) => { return false },
    };
    curr.modified < cached.modified
}
fn get_cache_path(script_name: Path) -> String {
    let file = script_name.filename_str().unwrap();
    let stem = match file.rfind('.') {
        Some(pos) => {
            file[0..pos]
        },
        None => {
            file
        },
    };
    let home = match std::os::homedir() {
        Some(p) => { let s = p.as_str(); String::from_str(s.unwrap()) },
        None => panic!("No home directory identified"),
    };
    let dir = format!("{}/.rusted/", home);
    let dirpath = Path::new(dir);
    match std::io::fs::mkdir_recursive(&dirpath, std::io::USER_RWX) {
        Ok(_) => {},
        Err(e) => panic!("Failed to create cache directory: {}", e),
    }
    format!("{}/{}", dirpath.as_str().unwrap(), stem)
}

fn get_filename(script_path: &str) -> String {
    String::from_str(Path::new(script_path).filename_str().unwrap())
}

fn main() {
    let args = os::args();
    let arg = match args.get(1) {
        Some(a) => a,
        None => panic!("No argument supplied"),
    };
    let cleaned = get_filename(arg.as_slice());
    let src = Path::new(String::from_str(arg.as_slice()));
    let target = get_cache_path(Path::new(cleaned.as_slice()));
    let sysroot = Path::new("/usr/local");
    let cached = has_cache(arg.as_slice(), target.as_slice());
    if !cached {
        compiler::compile_file(src, Path::new(target.as_slice()), Some(sysroot));
    }
    
    let mut cmd = Command::new(target);
    let mut argsvec: Vec<&String> = Vec::new();
    for a in args.iter() {
        argsvec.push(a);
    }
    for i in range(1, argsvec.len()) {
        cmd.arg(argsvec[i]);
    }
    cmd.stdout(process::StdioContainer::InheritFd(libc::STDOUT_FILENO));
    cmd.stderr(process::StdioContainer::InheritFd(libc::STDERR_FILENO));
    cmd.stdin(process::StdioContainer::InheritFd(libc::STDIN_FILENO));
    match cmd.spawn() {
        Ok(_) => (),
        Err(e) => panic!("Failed to execute process: {}", e),
    };
}
