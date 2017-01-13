
use lint::*;
use rustc_serialize::json::{self, Json, ToJson};
use systemd_parser;
use systemd_parser::items::*;
use std::collections::HashMap;

static DIRECTIVES : &'static str = include_str!("./directives.json");

#[derive(PartialEq, Eq, Clone, Debug, RustcDecodable)]
struct DocumentedDirective {
   url: String,
   field: String,
}

fn open_and_parse_directive_files(file: &str) -> HashMap<String, DocumentedDirective> {
    let vec : Vec<DocumentedDirective> = json::decode(DIRECTIVES).expect("json file should be ok");
    let mut res = HashMap::with_capacity(vec.len());
    for directive in vec {
        res.insert(directive.field.clone(), directive);
    }
    res
}

pub fn lint(unit: &SystemdUnit) -> Result<(), LintResult> {

    let directives = open_and_parse_directive_files("./src/lint/directives.json");
    const known_categories : &'static [&'static str] = &[
        "Unit", "Service", "Install", "Mount", "Socket", "Automount", "BusName", "Path", "Timer"
    ];

    let has_unknown = unit.keys()
                          .iter()
                          .any(|unit_entry| {
                              known_categories.contains(&unit_entry.category().as_ref()) &&
                                !directives.contains_key(&unit_entry.key())
                          });

    if has_unknown {
        return Err(LintResult {
            severity: LintSeverity::Error,
            message: "unknown directive found",
            code: LintCode::ErrorUnknownDirective,
        });
    }

    Ok(())
}

#[test]
fn success_case_in_known_category() {
    // arrange
    let input = "
        [Unit]
        Description= a dummy unit
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_ok())
}

#[test]
fn success_case_in_unknown_category() {
    // arrange
    let input = "
        [X-Unit]
        DescriptionFromOuterSpace= a dummy unit
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_ok())
}

#[test]
fn error_case() {
    // arrange
    let input = "
        [Service]
        ExecStrat=/bin/true
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_err());
}

