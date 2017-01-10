
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SystemdItem<'a> {
    Comment(&'a str),
    Category(&'a str),
    Directive(&'a str, &'a str),
}
