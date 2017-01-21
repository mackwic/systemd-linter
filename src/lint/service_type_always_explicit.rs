
use lint::*;
use systemd_parser::items::*;

pub fn lint(unit: &SystemdUnit) -> Result<(), LintResult> {

    // first check it's a service
    if !unit.has_category("Service") {
        return Ok(())
    }

    if let None = unit.lookup_by_key("Type") {

        return Err(LintResult {
            severity: LintSeverity::Warning,
            message: "Service Type= should always be explicit. Fill the Type= field.".into(),
            code: LintCode::WarnServiceTypeShouldAlwaysBeExplicit,
        })
    }

    Ok(())
}

#[cfg(test)]
use systemd_parser;

#[test]
fn success_case() {
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

