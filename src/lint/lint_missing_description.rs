
use lint::*;
use systemd_parser;
use systemd_parser::items::*;

pub fn lint(unit: &SystemdUnit) -> Result<(), LintResult> {

    if let None = unit.lookup_by_key("Description") {
        return Err(LintResult {
            severity: LintSeverity::Lint,
            message: "Consider filling the Description= field",
            code: LintCode::LintMissingDescription,
        })
    }

    Ok(())
}

#[test]
fn success_case() {
    // arrange
    let input = "
        [Unit]
        Description= a dummy unit
        [Service]
        ExecStart=/bin/true
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
        ExecStart=/bin/true
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_err());
}

