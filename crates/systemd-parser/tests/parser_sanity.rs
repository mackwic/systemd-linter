
extern crate systemd_parser;

use std::fs;
use std::io::Read;

#[test]
fn it_should_parse_all_example_files_with_no_error() {

    let entries = fs::read_dir("./tests/success_units/").expect("directory exists");
    for entry in entries {
        let entry = entry.expect("entry is ok");
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        println!("Reading {}", path.to_str().unwrap());

        let mut f = fs::File::open(path).expect("file must be open-able");
        let mut buffer = String::with_capacity(4096);
        f.read_to_string(&mut buffer).expect("file must be readable");

        systemd_parser::parse_string(&buffer).expect("should be ok");
    }
}

