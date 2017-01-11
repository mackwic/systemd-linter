
use lint::*;
use systemd_parser;
use systemd_parser::items::*;

pub fn lint(unit: &SystemdUnit) -> Result<(), LintResult> {

    // first check it's a service
    if !unit.has_category("Service") {
        return Ok(())
    }

    if let None = unit.lookup_by_key("Type") {

        return Err(LintResult {
            severity: LintSeverity::Lint,
            message: "Consider filling the Description= field",
            code: LintCode::LintMissingDescription,
        })
    }

    Ok(())
}

#[test]
fn it_should_exists() {
    // arrange
    let input = "
        [Service]
        Type=Simple
        ExecStart=/bin/true
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_ok())
}

#[test]
fn it_should_detect_missing_description() {
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
