
use items::SystemdItem;
use nom::*;

fn c_always_true(_c: char) -> bool { true }
fn c_is_category_element(c: char) -> bool {
    c.is_alphabetic() || c == '-'
}
fn c_is_value_element(c: char) -> bool {
    match c {
        '\n'|'\r'|'#' => false,
        _ => true,
    }
}
fn c_is_key_element(c: char) -> bool {
    match c {
        '!'|'|'|'@' => true,
        c if c.is_alphabetic() => true,
        _ => false
    }
}

named!(pub take_whole_line<&str, &str>, take_while_s!(c_always_true));

named!(
    pub parse_comment<&str, SystemdItem>,
    complete!(do_parse!(
        eat_separator!(" \t")      >>
        tag_s!("#")                >>
        comment: take_whole_line   >>
        (SystemdItem::Comment(comment.trim()))
    ))
);

named!(
    pub parse_category<&str, SystemdItem>,
    complete!(do_parse!(
        eat_separator!(" \t")   >>
        tag!("[")               >>
        eat_separator!(" ")     >>
        category: take_while1_s!(c_is_category_element) >>
        eat_separator!(" ")     >>
        tag!("]")               >>
        (SystemdItem::Category(category))
    ))
);

named!(
    pub parse_directive<&str, SystemdItem>,
    complete!(do_parse!(
        eat_separator!(" \t")   >>
        key: take_while1_s!(c_is_key_element) >>
        eat_separator!(" ")     >>
        tag!("=")               >>
        eat_separator!(" ")     >>
        value: take_while_s!(c_is_value_element) >>
        (SystemdItem::Directive(key, (if value.is_empty() { None } else { Some(value) })))
    ))
);

named!(
    pub parse_line<&str, SystemdItem>,
    do_parse!(
        value: alt_complete!(parse_category | parse_comment | parse_directive) >>
        eat_separator!(" \t") >>
        eof!()                >>
        (value)
    )
);

pub fn parse_unit(input: &str) -> Result<Vec<SystemdItem>, Vec<(IError<&str>, u32)>> {

    let mut errors = vec!();
    let mut oks = vec!();

    let mixed_res = input.lines()
                         .filter(|line| !line.trim().is_empty()) // skip white lines
                         .map(|line| parse_line(line));

    for res in mixed_res {
        match res.to_full_result() {
            Ok(ok_res) => oks.push(ok_res),
            Err(err_res) => errors.push(err_res),
        }
    }

    if errors.len() > 0 {
        Err(enhance_with_line_numbers(errors, input))
    } else {
        Ok(oks)
    }
}

fn enhance_with_line_numbers<'a>(errors: Vec<IError<&'a str>>, input: &str)
    -> Vec<(IError<&'a str>, u32)> {

    use nom::IError::*;
    use nom::ErrorKind::*;
    use nom::Err::*;

    errors.iter()
          .map(|error| {
              if let &Error(Position(Alt, pattern)) = error {
                  let line_number = count_lines_by_pattern(pattern, input);
                  (error.clone(), line_number)
              } else {
                  (error.clone(), 0) // FIXME: is it possible ?
              }
          }).collect()
}

fn count_lines_by_pattern(pattern: &str, haystack: &str) -> u32 {

    let mut idx = 0;
    haystack.lines()
            .map(|line| { idx += 1; (line, idx) })
            .find(|&(line, _)| line.contains(pattern))
            .expect("it has been parsed once, it must be in the input somewhere").1

}

