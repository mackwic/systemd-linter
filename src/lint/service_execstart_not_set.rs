
use lint::*;
use systemd_parser::items::*;

pub fn lint(unit: &SystemdUnit) -> Result<(), LintResult> {

    if let Some(&DirectiveEntry::Solo(ref type_entry)) = unit.lookup_by_key("Type") {

        println!("{:?}", type_entry.value());

        if type_entry.value() != Some("Simple") {
            return Ok(());
        }

        if !unit.has_key("ExecStart") {
            return Err(LintResult {
                severity: LintSeverity::Error,
                message: "Service with Type==Simple MUST set ExecStart= field".into(),
                code: LintCode::ErrorServiceSimpleMustHaveExecstart,
            });
        }
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
        Type=Simple
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_err());
}
