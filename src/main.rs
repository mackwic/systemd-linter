
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate clap;
extern crate colored;
extern crate rustc_serialize;
extern crate systemd_parser;

mod support;
mod lint;

use clap::{Arg, App};

pub fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let matches = App::new("systemd-lint")
        .version("0.1.4")
        .author("Thomas Wickham <twickham@octo.com>")
        .about("lint systemd unit files")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true))
        .get_matches();

    let filepath = Path::new(matches.value_of("INPUT").expect("clap should ensure INPUT is set"));
    if !filepath.exists() {
        error_and_exit("path does not exists !".into())
    }

    let mut contents = String::with_capacity(4096);
    let mut file = File::open(filepath)
        .unwrap_or_else(|err| format_res_and_exit(err, "file is not readable"));

    file.read_to_string(&mut contents)
        .unwrap_or_else(|err| format_res_and_exit(err, "error when reading file"));

    let unit_file = systemd_parser::parse_string(&contents)
        .unwrap_or_else(|err| format_res_and_exit(err, "PARSE ERROR"));


    let mut has_errors = false;

    for lint_f in lint::ALL_LINTS.iter() {
        let res = lint_f(&unit_file);
        has_errors = has_errors || res.is_ok();

        println!("** {:?}\n", res);
    }

    if has_errors {
        error_and_exit(String::from("Lint errors. Exiting"))
    }
}

fn format_res_and_exit<T, Err: std::error::Error>(err: Err, msg: &str) -> T {
    let msg = format!("{}: {}", msg, err);
    error_and_exit(msg)
}

#[allow(unreachable_code)]
fn error_and_exit<T>(msg: String) -> T {
    use colored::*;
    use std::io::stderr;
    use std::io::Write;
    use std::process::exit;

    let _ = writeln!(stderr(), "{}: {}", "Error".red().bold(), msg.red());
    exit(1);
    unreachable!()
}
