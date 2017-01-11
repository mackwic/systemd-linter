
#[macro_use]
extern crate nom;

mod items;
mod parser;

#[cfg(test)]
mod parser_test;
#[cfg(test)]
mod items_test;

pub fn parse_string(input: &str) -> Result<items::SystemdUnit, ()> {
    let parse_res = parser::parse_unit(input);

    // FIXME: meaningful errors
    if parse_res.is_err() { return Err(()) }
    let units = parse_res.unwrap();

    let systemd_unit_res = items::SystemdUnit::new(&units);

    // FIXME: meaningful errors
    if systemd_unit_res.is_err() { return Err(()) }
    Ok(systemd_unit_res.unwrap())
}

