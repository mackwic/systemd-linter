
use lint::*;
use systemd_parser::items::*;

static KNOWN_CATEGORIES: &'static [&'static str] =
    &["Unit", "Service", "Install", "Mount", "Socket", "Automount", "BusName", "Path", "Timer"];

pub fn lint(unit: &SystemdUnit) -> Result<(), LintResult> {

    for cat in unit.categories() {
        if &cat[0..2] != "X-" && !KNOWN_CATEGORIES.contains(&cat.as_ref()) {

            let error = Err(LintResult {
                severity: LintSeverity::Error,
                message: format!("Unknown category: {}", cat),
                code: LintCode::ErrorUnknownCategory,
            });

            return error;
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
        [Unit]
        Description= a dummy unit
        [Service]
        ExecStart=/bin/true
        [Install]
        DummyOne=1
        [Mount]
        DummyTwo=1
        [Socket]
        DummyThree=1
        [Automount]
        DummyFour=1
        [BusName]
        DummyFive=1
        [Path]
        DummySix=1
        [Timer]
        DummySeven=1
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
        [Services]
        ExecStart=/bin/true
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_err());
}

#[test]
fn error_message_should_contains_the_bad_category() {
    // arrange
    let input = "
        [Services]
        ExecStart=/bin/true
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit).unwrap_err();
    // assert
    assert!(res.message.contains("Services"))
}

#[test]
fn success_lint_should_skip_x_categories() {
    // arrange
    let input = "
        [Unit]
        Description=Some Monitoring Service

        [Service]
        ExecStart=/bin/monitorme

        [X-Fleet]
        MachineMetadata=location=chicago
        Conflicts=monitor*
    ";
    let unit = systemd_parser::parse_string(input).unwrap();
    // act
    let res = lint(&unit);
    // assert
    assert!(res.is_ok())
}
