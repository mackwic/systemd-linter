
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SystemdItem<'a> {
    Comment(&'a str),
    Category(&'a str),
    Directive(&'a str, &'a str),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct UnitDirective<'a> {
    key: &'a str,
    value: &'a str,
    category: &'a str,
}

impl<'a> UnitDirective<'a> {
    pub fn new<'b>(category: &'b str, key: &'b str, value: &'b str) -> UnitDirective<'b> {
        UnitDirective {
            category: category,
            value: value,
            key: key,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SystemdUnit<'a> {
    directives: HashMap<&'a str, UnitDirective<'a>>
}

impl<'a> SystemdUnit<'a> {

    pub fn new(unit_items: Vec<SystemdItem>) -> Result<SystemdUnit, ()> {

        if false == SystemdUnit::first_is_category(unit_items) {
            // TODO: error message
            return Err(())
        }

        let res = SystemdUnit {
            directives: HashMap::new()
        };
        Ok(res)
    }


    fn first_is_category(unit_items: Vec<SystemdItem>) -> bool {
        match unit_items.get(0) {
            Some(&SystemdItem::Category(_)) => true,
            _ => false
        }
    }
}


