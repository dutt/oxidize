extern crate rustc;
extern crate rustc_trans;
extern crate syntax;

use self::rustc_trans::driver::driver::{
    FileInput,
    StrInput,
    Input,
    compile_input
};
use self::rustc_trans::session::config::{
    basic_options,
    build_configuration,
    OutputTypeExe
};
use self::rustc_trans::session::{
    build_session,
    Session
};
use self::syntax::diagnostics;

fn basic_sess(sysroot : Path) -> Session {
    let mut opts = basic_options();
    opts.output_types = vec![OutputTypeExe];
    opts.maybe_sysroot = Some(sysroot);

    let descriptions = diagnostics::registry::Registry::new(&rustc::DIAGNOSTICS);
    let sess = build_session(opts, None, descriptions);
    sess
}

#[allow(dead_code)]
pub fn compile_string(input: &str, output: Path, sysroot: Option<Path>) {
    compile_simple(StrInput(String::from_str(input)), output, sysroot)
}

#[allow(dead_code)]
pub fn compile_file(path: Path, output: Path, sysroot: Option<Path>) {
    compile_simple(FileInput(path), output, sysroot)
}
fn compile_simple(input: Input, output: Path, sysroot: Option<Path>) {
    let sess = basic_sess(match sysroot {
        Some(path) => path,
        None => Path::new("/usr/local/")
    });
    let cfg = build_configuration(&sess);

    compile_input(sess, cfg, &input, 
                  &None, //output directory, not used when there's an output file
                  &Some(output), //output file
                  None);
}
