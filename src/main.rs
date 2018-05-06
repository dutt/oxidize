//extern crate libc;

use std::env;
use std::process;
use std::path::Path;

/*fn has_cache(script_path: &str, cache_path : &str) -> bool {
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
*/

struct Config {
    path: String,
    name: String,
    args: Vec<String>, 
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 1 {
            return Err("no file specified");
        }
        let pathobj = Path::new(&args[1]);
        let filepath = pathobj.to_str().unwrap().to_string();
        let filestem = pathobj.file_stem().unwrap().to_str().unwrap().to_string();
        let mut fargs = Vec::new();
        for a in args.iter() {
            fargs.push(a.clone());
        }
        fargs.remove(0); //oxidize
        fargs.remove(0); //script
        Ok(Config { path: filepath, name: filestem, args: fargs })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|_err| {
        println!("Failed to parse arguments");
        process::exit(1);
    });
    let compile_output = process::Command::new("rustc").
                    arg(config.path).
                    arg("--out-dir=target/cache").
                    output().
                    expect("failed to run rustc");
    if !compile_output.status.success() {
        println!("compile status: {}", compile_output.status);
        println!("compile stdout: {}", String::from_utf8_lossy(&compile_output.stdout));
        println!("compile stderr: {}", String::from_utf8_lossy(&compile_output.stderr));
        return;
    }

    let target = Path::new("target").join("cache").join(config.name);
    let mut exec_cmd = process::Command::new(target);
    for arg in &config.args {
        exec_cmd.arg(arg);
    }
    let mut exec_proc = exec_cmd.spawn().expect("failed to start child");
    exec_proc.wait().expect("error during execution");
}
