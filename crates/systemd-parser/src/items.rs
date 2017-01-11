
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

    pub fn item_list_to_unit_directive_list(unit_items: &'a Vec<SystemdItem<'a>>)
        -> Result<Vec<UnitDirective<'a>>, ()> {

        use self::SystemdItem::*;

        if unit_items.len() < 1 {
            return Err(()) // TODO: error message
        }

        let mut cat = try!(UnitDirective::get_first_category(unit_items));
        let mut res = vec!();

        for item in unit_items {
            match *item {
                Category(new_cat) => cat = new_cat,
                Directive(key, value) => res.push(UnitDirective::new(cat, key, value)),
                _ => () // TODO: do something with comments ?
            }
        }

        Ok(res)
    }

    fn get_first_category<'b>(unit_items: &'b Vec<SystemdItem<'b>>) -> Result<&'b str, ()> {
        if let Some(&SystemdItem::Category(first_cat)) = unit_items.get(0) {
            Ok(first_cat)
        } else {
            return Err(()) // TODO: error message
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SystemdUnit<'a> {
    directives: HashMap<&'a str, DirectiveEntry<'a>>
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DirectiveEntry<'a> {
    Solo(UnitDirective<'a>),
    Many(Vec<UnitDirective<'a>>)
}

impl<'a> SystemdUnit<'a> {

    pub fn new(unit_items: &'a Vec<SystemdItem<'a>>) -> Result<SystemdUnit<'a>, ()> {

        let directives = try!(
            UnitDirective::item_list_to_unit_directive_list(&unit_items)
        );

        let mut directives_hash = try!(
            SystemdUnit::hash_from_directives(directives)
        );

        let res = SystemdUnit {
            directives: directives_hash
        };
        Ok(res)
    }

    fn hash_from_directives(directives: Vec<UnitDirective>) -> Result<HashMap<&str, DirectiveEntry>, ()> {

        use self::DirectiveEntry::*;
        use std::collections::hash_map::Entry;

        let mut directives_hash = HashMap::new();

        for directive in directives {
            match directives_hash.entry(directive.key) {
                // first entry is a Solo
                Entry::Vacant(mut entry) => { entry.insert(Solo(directive)); },
                Entry::Occupied(mut entry) => {
                    let mut entry = entry.get_mut();
                    match *entry {
                // 2nd entry is a conversion from Solo to Many
                        Solo(first_dir) => {
                            let dirs = vec!(first_dir, directive);
                            try!(SystemdUnit::validate_many(&dirs));
                            *entry = Many(dirs);
                        }
                // 3rd+ entry is an append to vec in Many(vec)
                        Many(ref mut dirs) => {
                            dirs.push(directive);
                            try!(SystemdUnit::validate_many(dirs));
                        },
                    }
                }
            }
        }

        Ok(directives_hash)
    }

    fn validate_many(dirs: &Vec<UnitDirective>) -> Result<(), ()> {
        // contract: invariant
        assert!(dirs.len() >= 2);

        let first = dirs.first().unwrap();
        let last = dirs.last().unwrap();
        if first.category == last.category {
            Ok(())
        } else {
            Err(()) // TODO: error message
        }
    }

    pub fn lookup_by_key(&self, key: &str) -> Option<&DirectiveEntry> {
        self.directives.get(key)
    }

    pub fn lookup_by_category(&self, category: &str) -> Vec<&DirectiveEntry> {
		use self::DirectiveEntry::*;

        self.directives
            .values()
            .filter(|directive| match **directive { 
				Solo(dir) => dir.category == category,
				Many(ref dirs) => category == dirs.get(0).expect("dirs.len() > 0").category,
			})
            .collect()
    }
}

