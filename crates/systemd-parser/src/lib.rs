
extern crate itertools;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate quick_error;

pub mod errors;
pub mod items;
pub mod parser;

#[cfg(test)]
mod parser_test;
#[cfg(test)]
mod items_test;

pub fn parse_string(input: &str) -> Result<items::SystemdUnit, errors::ParserError> {

    // FIXME: this should be inside `parse_unit` but then, the lifetime would be wrong
    let input = String::from(input).replace("\\\n", "");
    let units = try!(parser::parse_unit(&input));
    let systemd_unit = try!(items::SystemdUnit::new(&units));
    Ok(systemd_unit)
}

