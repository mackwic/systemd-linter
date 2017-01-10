
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SystemdItem {
    Comment(String),
    Category(String),
    Directive(String, String),
}
