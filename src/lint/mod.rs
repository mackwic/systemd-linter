
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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LintCode {
    LintMissingDescription = 20_000,
    WarnServiceTypeShouldAlwaysBeExplicit = 30_000,
    ErrorServiceSimpleMustHaveExecstart = 40_000,
}

mod lint_missing_description;
mod service_type_always_explicit;
mod service_execstart_not_set;

use systemd_parser::items::SystemdUnit;

pub const ALL_LINTS : [fn(&SystemdUnit) -> Result<(), LintResult>; 3] = [
    lint_missing_description::lint,
    service_type_always_explicit::lint,
    service_execstart_not_set::lint,
];

