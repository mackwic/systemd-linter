
use nom;
use std::convert::From;
use std::fmt;
use std::error::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseErrorInternal(String, u32);

impl From<(String, u32)> for ParseErrorInternal {
    fn from(error_tuple: (String, u32)) -> ParseErrorInternal {
        ParseErrorInternal(error_tuple.0, error_tuple.1)
    }
}

impl fmt::Display for ParseErrorInternal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "line {}: {}", self.1, self.0)
    }
}

fn helper_format(errors: &Vec<ParseErrorInternal>) -> String {

    errors.iter()
          .map(|err| format!("* {}", err))
          .fold(String::with_capacity(100), |mut acc, line| { acc.push_str(&line); acc })
}

quick_error!(
    #[derive(Debug)]
    pub enum ParserError {
        ParseError(errors: Vec<ParseErrorInternal>) {
            from()
            description("Failed to parse the unit file")
            display(error)-> ("{}, errors:\n{}", error.description(), helper_format(errors))
        }
        UnitGrammarError(err: String) {
            from()
            description("The unit file doesn't make sense")
        }
    }
);

// TODO: understand why blanket implem does not work
impl<'a> From<Vec<(nom::IError<&'a str>, u32)>> for ParserError {
    fn from(errors: Vec<(nom::IError<&'a str>, u32)>) -> ParserError {

        let mut res = vec!();
        for (err, line) in errors {
            res.push((format!("{:?}", err), line).into())
        }
        ParserError::ParseError(res)
    }
}
