
pub use parser::*;
pub use items::*;

mod take_whole_line {
    pub use super::*;

    #[test]
    fn it_returns_whole_line() {
        let input = " yo 👋!";
        let res = take_whole_line(input);

        assert!(res.is_done());
        assert_eq!(input, res.unwrap().1)
    }
}

mod parse_comment {
    pub use super::*;

    #[test]
    fn it_should_fail_when_not_starting_with_hash_char() {
        let res = parse_comment("yo");
        assert_eq!(true, res.is_err())
    }

    #[test]
    fn it_should_be_done_when_starting_with_hash_char() {
        let res = parse_comment("# yo");
        assert_eq!(true, res.is_done())
    }

    #[test]
    fn it_should_trim_whitespaces() {
        let res = parse_comment("# yo ");
        let expected = SystemdItem::Comment("yo");
        assert_eq!(expected, res.unwrap().1)
    }
}

mod parse_category {
    pub use super::*;

    #[test]
    fn it_should_be_done() {
        let input = "[Category]";
        let res = parse_category(input);
        assert!(res.is_done())
    }

    #[test]
    fn it_should_consume_the_category() {
        let input = "[ Category ]";
        let res = parse_category(input);
        let expected = SystemdItem::Category("Category");
        assert_eq!(expected, res.unwrap().1);
    }

    #[test]
    fn it_should_reject_more_than_one_word() {
        let input = "[ Category wrong ]";
        let res = parse_category(input);
        assert!(res.is_err())
    }
}

mod parse_directive {
    pub use super::*;

    #[test]
    fn it_should_be_done() {
        let input = "ExecStart=42";
        let res = parse_directive(input);

        assert!(res.is_done());
    }

    #[test]
    fn it_should_consume_the_directive_key_and_value() {
        let input = "ExecStart = 42";
        let res = parse_directive(input);
        let expected = SystemdItem::Directive("ExecStart", "42");

        assert_eq!(expected, res.unwrap().1);
    }

    #[test]
    fn it_should_reject_the_directive_with_no_key() {
        let input = " = 42";
        let res = parse_directive(input);

        assert!(res.is_err());
    }

    #[test]
    fn it_should_reject_the_directive_with_no_value() {
        let input = "Yo =";
        let res = parse_directive(input);

        assert!(res.is_err());
    }

    #[test]
    fn it_should_reject_the_directive_with_invalid_characters() {
        let inputs = vec!["Yo == 42", "Yo =! 42", "Yo = !42"];
        for input in inputs {
            let res = parse_directive(input);
            assert!(res.is_err(), "expected {} to be rejected", input);
        }
    }

    #[test]
    fn it_should_consume_path_in_values() {
        let input = "ExecStart=/usr/sbin/some-fancy-httpd-server -p 3000 -h localhost -l server.log";
        let res = parse_directive(input);
        let expected = SystemdItem::Directive(
            "ExecStart",
            "/usr/sbin/some-fancy-httpd-server -p 3000 -h localhost -l server.log"
        );

        assert_eq!(expected, res.unwrap().1);
    }

    #[test]
    fn it_doesnt_consume_comment_at_end_of_line() {
        let input = "ExecStart=/usr/sbin/some-fancy-httpd-server # I like this one";
        let res = parse_directive(input);

        assert_eq!("# I like this one", res.unwrap().0)
    }

    #[test]
    fn it_should_accept_exotic_output() {
        let inputs = vec![
            ("!ConditionPathIsMountPoint=/mnt/plop", "bang in keys"),
            ("|ConditionPathIsMountPoint=/mnt/plop", "pipe in keys"),
            ("Alias=foo.service.wants/bar.service", "Alias=foo.service.wants/bar.service"),
            ("ExecStart=-/bin/false", "- in value"),
            ("ExecStart=@/bin/echo", "@ in value"),
            ("ExecStart=+/bin/true", "+ in value"),
            ("ExecStart=+@-/bin/true", "+@- in value"),
            ("ExecStart=/bin/echo $TERM", "$ in value"),
            ("Environment=\"https_proxy=http://squidaws.internet.iz.eu-west-1.aws:8080\"",
             "inline quotes and URI"),
        ];

        for (input, msg) in inputs {
            let res = parse_directive(input);
            assert!(res.is_done(), "it should accept {}", msg)
        }
    }
}

mod parse_line {
    pub use super::*;

    #[test]
    fn it_is_present() {
        let input = "[Unit]";
        let res = parse_line(input);
        assert!(res.is_done())
    }

    #[test]
    fn it_can_parse_category() {
        let input = "[Unit]";
        let res = parse_line(input);
        assert_eq!(SystemdItem::Category("Unit"), res.unwrap().1)
    }

    #[test]
    fn it_can_parse_comment() {
        let input = "# comment";
        let res = parse_line(input);
        assert_eq!(SystemdItem::Comment("comment"), res.unwrap().1)
    }

    #[test]
    fn it_can_parse_directive() {
        let input = "ExecStart=/usr/bin/true";
        let res = parse_line(input);
        assert_eq!(
            SystemdItem::Directive("ExecStart", "/usr/bin/true"),
            res.unwrap().1
        )
    }
}

mod parse_unit {
    pub use super::*;

    const DUMMY_UNIT_STR : &'static str = "[Unit]
        Description=This is a dummy unit file
        [Service]
        ExecStart=/usr/bin/true
    ";

    #[test]
    fn it_returns_a_vec_of_parsed_lines() {
        let input = DUMMY_UNIT_STR;
        let res = parse_unit(input);
        assert_eq!(
            &SystemdItem::Category("Unit"),
            res.unwrap().get(0).unwrap()
        )
    }

    #[test]
    fn it_skip_empty_lines() {
        let input = "
[Unit]";
        let res = parse_unit(input);
        assert_eq!(
            &SystemdItem::Category("Unit"),
            res.unwrap().get(0).unwrap()
        )
    }

    #[test]
    fn it_trim_whitespaces_at_beginning_and_end_of_line() {
        let input = "\n     \t   [Unit]  ";
        let res = parse_unit(input);
        assert_eq!(
            &SystemdItem::Category("Unit"),
            res.unwrap().get(0).unwrap()
        )
    }

    #[test]
    fn it_works_with_the_dummy() {
        let dummy_unit_parsed = vec![
            SystemdItem::Category("Unit"),
            SystemdItem::Directive("Description", "This is a dummy unit file"),
            SystemdItem::Category("Service"),
            SystemdItem::Directive("ExecStart", "/usr/bin/true")
        ];

        let res = parse_unit(DUMMY_UNIT_STR);
        assert_eq!(dummy_unit_parsed, res.unwrap())
    }

    #[test]
    fn it_returns_an_error_in_case_of_parse_error() {
        let input = "[Unit]\nplop";
        let res = parse_unit(input);
        assert!(res.is_err())
    }

    #[test]
    fn it_keep_line_numbers_in_errors() {
        use nom::{IError, Err, ErrorKind};
        let input = "[Unit]\nplop";
        let res = parse_unit(input);
        assert_eq!(
            1,
            res.unwrap_err().get(0).unwrap().1
        )
    }

    #[test]
    fn it_keep_line_numbers_in_errors2() {
        use nom::{IError, Err, ErrorKind};
        let input = "[Unit]\nplop\nPlop=42\n[Nice things]";
        let res = parse_unit(input);
        assert_eq!(
            3,
            res.unwrap_err().get(1).unwrap().1
        )
    }
}

