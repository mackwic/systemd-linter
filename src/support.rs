
use systemd_parser::items::*;

pub trait SystemdUnitExt {
    /// Key exists and have value Solo(expected_value)
    fn key_have_solo_value(&self, key: &str, expected_value: &str) -> bool;
    /// Key may exists, in this case it has value Solo(expected_value)
    fn key_may_have_solo_value(&self, key: &str, expected_value: &str) -> bool;
}

impl SystemdUnitExt for SystemdUnit {
    fn key_have_solo_value(&self, key: &str, expected_value: &str) -> bool {

        match self.lookup_by_key(key) {
            Some(&DirectiveEntry::Solo(ref entry)) => entry.value() == Some(expected_value),
            _ => false,
        }
    }

    fn key_may_have_solo_value(&self, key: &str, expected_value: &str) -> bool {

        match self.lookup_by_key(key) {
            Some(&DirectiveEntry::Solo(ref entry)) => entry.value() == Some(expected_value),
            None => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    mod key_have_solo_value {
        pub use super::*;
        use systemd_parser;

        #[test]
        fn it_exists() {
            let input = "
                [Unit]
                Description= a dummy unit
            ";
            let unit = systemd_parser::parse_string(input).unwrap();

            assert!(unit.key_have_solo_value("Description", "a dummy unit"))
        }

        #[test]
        fn should_check_value() {
            let input = "
                [Unit]
                Description= a dummy unit
            ";
            let unit = systemd_parser::parse_string(input).unwrap();

            assert!(false == unit.key_have_solo_value("Description", "not this"))
        }

        #[test]
        fn should_fail_if_key_does_not_exists() {
            let input = "
                [Unit]
                Description= a dummy unit
            ";
            let unit = systemd_parser::parse_string(input).unwrap();

            assert!(false == unit.key_have_solo_value("ExecStart", ""))
        }

        #[test]
        fn should_fail_if_key_have_not_solo() {
            let input = "
                [Service]
                ExecStartPre=/bin/true
                ExecStartPre=/bin/true
                ExecStart=/bin/true
            ";
            let unit = systemd_parser::parse_string(input).unwrap();

            assert!(false == unit.key_have_solo_value("ExecStartPre", "/bin/true"))
        }
    }

    mod key_may_have_solo_value {
        pub use super::*;
        use systemd_parser;

        #[test]
        fn it_exists() {
            let input = "
                [Unit]
                Description= a dummy unit
            ";
            let unit = systemd_parser::parse_string(input).unwrap();

            assert!(unit.key_may_have_solo_value("Description", "a dummy unit"))
        }

        #[test]
        fn should_check_value() {
            let input = "
                [Unit]
                Description= a dummy unit
            ";
            let unit = systemd_parser::parse_string(input).unwrap();

            assert!(false == unit.key_may_have_solo_value("Description", "not this"))
        }

        #[test]
        fn should_be_ok_if_key_does_not_exists() {
            let input = "
                [Unit]
                Description= a dummy unit
            ";
            let unit = systemd_parser::parse_string(input).unwrap();

            assert!(true == unit.key_may_have_solo_value("ExecStart", ""))
        }

        #[test]
        fn should_fail_if_key_have_not_solo() {
            let input = "
                [Service]
                ExecStartPre=/bin/true
                ExecStartPre=/bin/true
                ExecStart=/bin/true
            ";
            let unit = systemd_parser::parse_string(input).unwrap();

            assert!(false == unit.key_may_have_solo_value("ExecStartPre", "/bin/true"))
        }
    }
}
