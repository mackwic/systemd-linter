
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
    pub fn value(&self) -> &str { &self.value }
    pub fn key(&self) -> &str { &self.key }
    pub fn category(&self) -> &str { &self.category }
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
        -> Result<Vec<UnitDirective>, String> {

        use self::SystemdItem::*;

        if unit_items.len() < 1 {
            return Err(format!("No directives in the file"))
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

    fn get_first_category<'b>(unit_items: &'b Vec<SystemdItem<'b>>) -> Result<&'b str, String> {
        if let Some(&SystemdItem::Category(first_cat)) = unit_items.get(0) {
            Ok(first_cat)
        } else {
            return Err(format!("The first non-comment line must be a [Category]"))
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SystemdUnit {
    directives: HashMap<String, DirectiveEntry>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DirectiveEntry {
    Solo(UnitDirective),
    Many(Vec<UnitDirective>)
}

impl DirectiveEntry {
    pub fn category(&self) -> String {
        use self::DirectiveEntry::*;

        match *self {
            Solo(ref entry) => entry.category().into(),
            Many(ref entries) => entries.get(0).expect("len > 1").category().into()
        }
    }
}

impl SystemdUnit {

    pub fn new(unit_items: &Vec<SystemdItem>) -> Result<SystemdUnit, String> {

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

    fn hash_from_directives(directives: Vec<UnitDirective>) -> Result<HashMap<String, DirectiveEntry>, String> {

        use self::DirectiveEntry::*;
        use std::collections::hash_map::Entry;

        let mut directives_hash = HashMap::new();

        for directive in directives {
            match directives_hash.entry(directive.key.clone()) {
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

    fn validate_many(dirs: &Vec<UnitDirective>) -> Result<(), String> {
        // contract: invariant
        assert!(dirs.len() >= 2);

        let first = dirs.first().unwrap();
        let last = dirs.last().unwrap();
        if first.category == last.category {
            Ok(())
        } else {
            Err(format!(
                    "The same directive is repeated many times in different categories: {:?}, {:?}",
                    first, last
            ))
        }
    }

    pub fn lookup_by_key(&self, key: &str) -> Option<&DirectiveEntry> {
        self.directives.get(key)
    }

    pub fn lookup_by_category(&self, category: &str) -> Vec<&DirectiveEntry> {
        self.directives
            .values()
            .filter(|directive| directive.category() == category)
            .collect()
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.directives.contains_key(key)
    }

    pub fn has_category(&self, category: &str) -> bool {
        self.directives
            .values()
            .any(|directive| directive.category() == category)
    }

    pub fn keys(&self) -> Vec<&DirectiveEntry> {
        self.directives.values().collect()
    }

    pub fn categories(&self) -> Vec<String> {
        use itertools::Itertools;

        self.directives
            .values()
            .unique_by(|entry| entry.category())
            .map(|entry| entry.category())
            .sorted()
    }
}

