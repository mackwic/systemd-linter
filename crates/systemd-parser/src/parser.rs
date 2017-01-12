
use items::SystemdItem;
use nom::*;

fn c_always_true(_c: char) -> bool { true }
fn c_is_alphabetic(c: char) -> bool { c.is_alphabetic() }
fn c_is_value_element(c: char) -> bool {
    match c {
        ' '|'/'|'-'|'_'|'.'|'@'|'+'|':'|'"'|'|'|'\'' => true,
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
        category: take_while1_s!(c_is_alphabetic) >>
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
        value: take_while1_s!(c_is_value_element) >>
        (SystemdItem::Directive(key, value))
    ))
);

named!(
    pub parse_line<&str, SystemdItem>,
    alt_complete!(parse_category | parse_comment | parse_directive)
);

pub fn parse_unit(input: &str) -> Result<Vec<SystemdItem>, Vec<(IError<&str>, u32)>> {
    let mut errors = vec!();
    let mut oks = vec!();
    let mut line_index = 0;
    let mixed_res = input.lines()
                         .map(|l| {
                             let res = (l, line_index);
                             line_index += 1;
                             res
                         })
                         .filter(|&(line, _)| !line.trim().is_empty()) // skip white lines
                         .map(|(line, idx)| (parse_line(line), idx));

    for (res, line_idx) in mixed_res {
        match res.to_full_result() {
            Ok(ok_res) => oks.push(ok_res),
            Err(err_res) => errors.push((err_res, line_idx))
        }
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(oks)
    }
}


