
use lint::*;
use systemd_parser::items::*;

pub fn lint(unit: &SystemdUnit) -> Result<(), LintResult> {

    let error = Err(LintResult {
        severity: LintSeverity::Lint,
        message: "Consider filling the Description= field".into(),
        code: LintCode::LintMissingDescription,
    });

    match unit.lookup_by_key("Description") {
        None => error,
        Some(&DirectiveEntry::Solo(ref entry)) if entry.value().is_none() => error,
        _ => Ok(()),
    }
}

#[cfg(test)]
use systemd_parser;

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
fn error_case_missing_directive() {
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

#[test]
fn error_case_missing_value() {
    // arrange
    let input = "
        [Unit]
        Description=
        [Service]
        ExecStart=/bin/true
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_err());
}
