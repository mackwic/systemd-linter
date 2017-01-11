
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LintCode {
    LintMissingDescription = 20_000,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LintSeverity {
    Ignore,
    Lint,
    Warning,
    Error,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LintResult {
    severity: LintSeverity,
    message: &'static str,
    code: LintCode,
    // TODO: add line + column
}

use systemd_parser::items::SystemdUnit;

mod lint_missing_description;

pub const ALL_LINTS : [fn(&SystemdUnit) -> Result<(), LintResult>; 1] = [
    lint_missing_description::lint
];

