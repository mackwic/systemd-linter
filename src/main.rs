
#[macro_use]
extern crate nom;

use nom::*;

fn c_always_true(_c: char) -> bool { true }
fn c_is_alphabetic(c: char) -> bool { c.is_alphabetic() }
fn c_is_value_element(c: char) -> bool {
    match c {
        ' '|'/'|'-'|'_'|'.'|'@'|'+' => true,
        c if c.is_alphanumeric() => true,
        _ => false
    }
}
fn c_is_key_element(c: char) -> bool {
    match c {
        '!'|'|'|'@' => true,
        c if c.is_alphabetic() => true,
        _ => false
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum SystemdItem {
    Comment(String),
    Category(String),
    Directive(String, String),
}

named!(take_whole_line<&str, &str>, take_while_s!(c_always_true));

#[test]
fn take_whole_line_returns_whole_line() {
    let input = " yo 👋!";
    let res = take_whole_line(input);

    assert!(res.is_done());
    assert_eq!(input, res.unwrap().1)
}

named!(
    parse_comment<&str, SystemdItem>,
    do_parse!(
        tag_s!("#")                >>
        comment: take_whole_line   >>
        (SystemdItem::Comment(String::from(comment.trim())))
    )
);

#[test]
fn parse_comment_should_fail_when_not_starting_with_hash_char() {
    let res = parse_comment("yo");
    assert_eq!(true, res.is_err())
}

#[test]
fn parse_comment_should_be_done_when_starting_with_hash_char() {
    let res = parse_comment("# yo");
    assert_eq!(true, res.is_done())
}

#[test]
fn parse_comment_should_trim_whitespaces() {
    let res = parse_comment("# yo ");
    let expected = SystemdItem::Comment(String::from("yo"));
    assert_eq!(expected, res.unwrap().1)
}

named!(
    parse_category<&str, SystemdItem>,
    do_parse!(
        tag!("[") >>
        eat_separator!(" ") >>
        category: take_while1_s!(c_is_alphabetic) >>
        eat_separator!(" ") >>
        tag!("]") >>
        (SystemdItem::Category(String::from(category)))
    )
);

#[test]
fn parse_category_should_be_done() {
    let input = "[Category]";
    let res = parse_category(input);
    assert!(res.is_done())
}

#[test]
fn parse_category_should_consume_the_category() {
    let input = "[ Category ]";
    let res = parse_category(input);
    let expected = SystemdItem::Category(String::from("Category"));
    assert_eq!(expected, res.unwrap().1);
}

#[test]
fn parse_category_should_reject_more_than_one_word() {
    let input = "[ Category wrong ]";
    let res = parse_category(input);
    assert!(res.is_err())
}

named!(
    parse_directive<&str, SystemdItem>,
    do_parse!(
        key: take_while1_s!(c_is_key_element) >>
        eat_separator!(" ") >>
        tag!("=") >>
        eat_separator!(" ") >>
        value: take_while1_s!(c_is_value_element) >>
        (SystemdItem::Directive(String::from(key), String::from(value)))
    )
);

#[test]
fn parse_directive_should_be_done() {
    let input = "ExecStart=42";
    let res = parse_directive(input);

    assert!(res.is_done());
}

#[test]
fn parse_directive_should_consume_the_directive_key_and_value() {
    let input = "ExecStart = 42";
    let res = parse_directive(input);
    let expected = SystemdItem::Directive(String::from("ExecStart"), String::from("42"));

    assert_eq!(expected, res.unwrap().1);
}

#[test]
fn parse_directive_should_reject_the_directive_with_no_key() {
    let input = " = 42";
    let res = parse_directive(input);

    assert!(res.is_err());
}

#[test]
fn parse_directive_should_reject_the_directive_with_no_value() {
    let input = "Yo =";
    let res = parse_directive(input);

    assert!(res.is_err());
}

#[test]
fn parse_directive_should_reject_the_directive_with_invalid_characters() {
    let inputs = vec!["Yo == 42", "Yo =! 42", "Yo = !42"];
    for input in inputs {
        let res = parse_directive(input);
        assert!(res.is_err(), "expected {} to be rejected", input);
    }
}

#[test]
fn parse_directive_should_consume_path_in_values() {
    let input = "ExecStart=/usr/sbin/some-fancy-httpd-server -p 3000 -h localhost -l server.log";
    let res = parse_directive(input);
    let expected = SystemdItem::Directive(
        String::from("ExecStart"),
        String::from("/usr/sbin/some-fancy-httpd-server -p 3000 -h localhost -l server.log")
    );

    assert_eq!(expected, res.unwrap().1);
}

#[test]
fn parse_directive_doesnt_consume_comment_at_end_of_line() {
    let input = "ExecStart=/usr/sbin/some-fancy-httpd-server # I like this one";
    let res = parse_directive(input);

    assert_eq!("# I like this one", res.unwrap().0)
}

#[test]
fn parse_directive_should_accept_exotic_output() {
    let inputs = vec![
        ("!ConditionPathIsMountPoint=/mnt/plop", "bang in keys"),
        ("|ConditionPathIsMountPoint=/mnt/plop", "pipe in keys"),
        ("Alias=foo.service.wants/bar.service", "Alias=foo.service.wants/bar.service"),
        ("ExecStart=-/bin/false", "- in value"),
        ("ExecStart=@/bin/echo", "@ in value"),
        ("ExecStart=+/bin/true", "+ in value"),
        ("ExecStart=+@-/bin/true", "+@- in value"),
        ("ExecStart=/bin/echo $TERM", "$ in value"),
    ];

    for (input, msg) in inputs {
        let res = parse_directive(input);
        assert!(res.is_done(), "it should accept {}", msg)
    }
}

pub fn main() {
    println!("yo")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
