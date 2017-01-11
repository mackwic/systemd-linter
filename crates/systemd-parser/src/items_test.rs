
pub use items::*;

mod systemd_unit {
    pub use super::*;
    pub use super::SystemdItem::*;

    mod new {
        pub use super::*;

        #[test]
        fn it_should_instantiate() {
            let input = vec![
                Category("Unit"),
                Directive("Description", "A dummy unit file"),
            ];

            let res = SystemdUnit::new(input);

            assert!(res.is_ok())
        }

        #[test]
        fn it_should_err_when_not_starting_with_a_category() {
            let input = vec![
                Directive("Description", "A dummy unit file"),
            ];

            let res = SystemdUnit::new(input);

            assert!(res.is_err())
        }
    }
}
