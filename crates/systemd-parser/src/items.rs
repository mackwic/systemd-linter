
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SystemdItem<'a> {
    Comment(&'a str),
    Category(&'a str),
    Directive(&'a str, &'a str),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UnitDirective {
    key: String,
    value: String,
    category: String,
}

impl UnitDirective {
    pub fn new(category: &str, key: &str, value: &str) -> UnitDirective {
        UnitDirective {
            category: String::from(category),
            value: String::from(value),
            key: String::from(key),
        }
    }

    pub fn item_list_to_unit_directive_list(unit_items: &Vec<SystemdItem>)
        -> Result<Vec<UnitDirective>, ()> {

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
pub struct SystemdUnit {
    directives: HashMap<String, DirectiveEntry>
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DirectiveEntry {
    Solo(UnitDirective),
    Many(Vec<UnitDirective>)
}

impl SystemdUnit {

    pub fn new(unit_items: &Vec<SystemdItem>) -> Result<SystemdUnit, ()> {

        let directives = try!(
            UnitDirective::item_list_to_unit_directive_list(&unit_items)
        );

        let directives_hash = try!(
            SystemdUnit::hash_from_directives(directives)
        );

        let res = SystemdUnit {
            directives: directives_hash
        };
        Ok(res)
    }

    fn hash_from_directives(directives: Vec<UnitDirective>) -> Result<HashMap<String, DirectiveEntry>, ()> {

        use self::DirectiveEntry::*;
        use std::collections::hash_map::Entry;

        let mut directives_hash = HashMap::new();

        for directive in directives {
            match directives_hash.entry(directive.key.clone()) {
                // first entry is a Solo
                Entry::Vacant(entry) => { entry.insert(Solo(directive)); },
                Entry::Occupied(mut entry_container) => {
                    let mut vecs = vec!();
                    // FIXME do not clone :(
                    match *entry_container.get() {
                        Solo(ref first_dir) => { vecs.push(first_dir.clone()); },
                        Many(ref dirs) => { vecs = dirs.clone(); }
                    }
                    vecs.push(directive);
                    try!(SystemdUnit::validate_many(&vecs));
                    entry_container.insert(Many(vecs));
                },
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
            .filter(|directive| match *directive { 
				&Solo(ref dir) => dir.category == category,
				&Many(ref dirs) => category == dirs.get(0).expect("dirs.len() > 0").category,
			})
            .collect()
    }
}

