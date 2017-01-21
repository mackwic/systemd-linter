
use lint::*;
use support::*;
use systemd_parser::items::*;

pub fn lint(unit: &SystemdUnit) -> Result<(), LintResult> {

    // Skip the lint if Type is not dbus
    if !unit.key_have_solo_value("Type", "dbus") {
        return Ok(());
    }

    if false == unit.has_key("BusName") {
        Err(LintResult {
            severity: LintSeverity::Error,
            message: "You must fill the BusName= directive in a dbus service".into(),
            code: LintCode::ErrorMissingBusNameDirectiveInDBusService,
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
use systemd_parser;

#[test]
fn success_case() {
    // arrange
    let input = "
        [Service]
        Type=dbus
        BusName=my-service
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_ok())
}

#[test]
fn success_case_not_dbus_service() {
    // arrange
    let input = "
        [Service]
        Type=simple
        ExecStart=/bin/true
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_ok());
}

#[test]
fn success_case_not_service() {
    // arrange
    let input = "
        [Unit]
        Description= a dummy file
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_ok());
}

#[test]
fn error_case_missing_directive() {
    // arrange
    let input = "
        [Service]
        Type=dbus
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_err());
}
