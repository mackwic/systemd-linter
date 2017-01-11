
extern crate clap;
extern crate systemd_parser;

mod lint;

use clap::{Arg, App};

pub fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let matches = App::new("systemd-lint")
                    .version("0.1.0")
                    .author("Thomas Wickham <twickham@octo.com>")
                    .about("lint systemd unit files")
                    .arg(Arg::with_name("INPUT")
                         .help("Sets the input file to use")
                         .required(true))
                    .get_matches();

    let filepath = Path::new(matches.value_of("INPUT").expect("clap should ensure INPUT is set"));
    if !filepath.exists() { panic!("path does not exists !") }
    if !filepath.is_file() { panic!("path is not a file !") }

    let mut file = File::open(filepath).expect("file is not readable !");
    let mut contents = String::with_capacity(4096);
    file.read_to_string(&mut contents).expect("error when reading file !");

    let unit_file = systemd_parser::parse_string(&contents).expect("PARSE ERROR !");

    for lint_f in lint::ALL_LINTS.iter() {
        let res = lint_f(&unit_file);
        println!("** {:?}\n", res);
    }
}


