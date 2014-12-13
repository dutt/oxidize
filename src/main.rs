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
    let stem = script_name.filestem().unwrap();
    format!("/home/dutt/.rusted/{}", String::from_utf8_lossy(stem))
}
fn get_clean_name(script_path: &str) -> String {
    let last_idx = match script_path.rfind(std::path::posix::SEP) {
        Some(idx) => idx,
        None => 0
    };
    String::from_str(script_path[last_idx+1..script_path.len()])
}
fn main() {
    let args = os::args();
    println!("{}", args);
    let arg = match args.get(1) {
        Some(a) => a,
        None => panic!("No argument supplied"),
    };
    let cleaned = get_clean_name(arg.as_slice());
    println!("cleaned: {}", cleaned);
    let src = Path::new(String::from_str(cleaned.as_slice()));
    let target = get_cache_path(Path::new(String::from_str(cleaned.as_slice())));
    println!("target: {}", target);
    let sysroot = Path::new("/usr/local");
    let cached = has_cache(arg.as_slice(), target.as_slice());
    println!("cached: {}", cached);
    if !cached {
        compiler::compile_file(src, Path::new(target.clone()), Some(sysroot));
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
