
#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LintSeverity {
    Ignore,
    Lint,
    Warning,
    Error,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LintResult {
    // TODO: add line + column
    severity: LintSeverity,
    message: String,
    code: LintCode,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LintCode {
    LintMissingDescription                              = 20_000,
    WarnServiceTypeShouldAlwaysBeExplicit               = 30_000,
    ErrorServiceSimpleMustHaveExecstart                 = 40_000,
    ErrorUnknownDirective                               = 40_001,
    ErrorUnknownCategory                                = 40_002,
}

mod lint_missing_description;
mod service_type_always_explicit;
mod service_execstart_not_set;
mod unknown_directive;
mod unknown_category;

use systemd_parser::items::SystemdUnit;

pub const ALL_LINTS: &'static [fn(&SystemdUnit) -> Result<(), LintResult>] =
    &[lint_missing_description::lint,
      service_type_always_explicit::lint,
      service_execstart_not_set::lint,
      unknown_directive::lint,
      unknown_category::lint];
