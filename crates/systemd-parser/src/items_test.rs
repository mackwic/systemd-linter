
pub use items::*;
pub use items::SystemdItem::*;
pub use items::DirectiveEntry::*;

mod unit_directive {
    pub use super::*;

    mod item_list_to_unit_directive_list {
        pub use super::*;

        #[test]
        fn it_should_err_for_empty_vecs() {
            let input = vec!();
            let res = UnitDirective::item_list_to_unit_directive_list(&input);

            assert!(res.is_err());
        }

        #[test]
        fn it_should_instantiate_a_unit_directive_from_category_and_directive_items() {
            let input = vec![
                Category("Unit"),
                Directive("Description", Some("plop")),
            ];
            let res = UnitDirective::item_list_to_unit_directive_list(&input);
            let expected = UnitDirective::new("Unit".into(), "Description".into(), Some("plop".into()));

            assert!(res.is_ok());
            assert_eq!(Some(&expected), res.unwrap().get(0))
        }

        #[test]
        fn the_second_directive_should_keep_the_category() {
            let input = vec![
                Category("Unit"),
                Directive("Description", Some("plop")),
                Directive("Wants", Some("boot.target")),
            ];
            let res = UnitDirective::item_list_to_unit_directive_list(&input);
            let expected = UnitDirective::new("Unit".into(), "Wants".into(), Some("boot.target".into()));

            assert!(res.is_ok());
            assert_eq!(Some(&expected), res.unwrap().get(1))
        }

        #[test]
        fn it_should_change_the_category_if_a_category_item_is_seen() {
            let input = vec![
                Category("Unit"),
                Directive("Description", Some("plop")),
                Category("Service"),
                Directive("ExecStart", Some("/usr/bin/true")),
            ];
            let res = UnitDirective::item_list_to_unit_directive_list(&input);
            let expected = UnitDirective::new("Service".into(), "ExecStart".into(), Some("/usr/bin/true".into()));

            assert!(res.is_ok());
            assert_eq!(Some(&expected), res.unwrap().get(1))
        }

        #[test]
        fn it_should_err_if_the_first_item_is_not_a_category() {
            let input = vec![
                Directive("Description", Some("plop")),
                Category("Service"),
                Directive("ExecStart", Some("/usr/bin/true")),
            ];
            let res = UnitDirective::item_list_to_unit_directive_list(&input);

            assert!(res.is_err());
        }

        #[test]
        fn it_should_work_with_full_dummy_unit() {
            let input = vec![
                Category("Unit"),
                Directive("Description", Some("Some HTTP server")),
                Directive("After", Some("remote-fs.target sqldb.service memcached.service")),
                Directive("Requires", None),
                Directive("Requires", Some("sqldb.service memcached.service")),
                Directive("AssertPathExists", Some("/srv/www")),
                Category("Service"),
                Directive("Type", Some("notify")),
                Directive("ExecStart", Some("/usr/sbin/some-fancy-httpd-server")),
                Directive("Nice", Some("0")),
                Directive("PrivateTmp", Some("yes")),
                Category("Install"),
                Directive("WantedBy", Some("multi-user.target")),
            ];
            let res = UnitDirective::item_list_to_unit_directive_list(&input);

            let expected = vec![
                UnitDirective::new("Unit".into(), "Description".into(), Some("Some HTTP server".into())),
                UnitDirective::new("Unit".into(), "After".into(), Some("remote-fs.target sqldb.service memcached.service".into())),
                UnitDirective::new("Unit".into(), "Requires".into(), None),
                UnitDirective::new("Unit".into(), "Requires".into(),"sqldb.service memcached.service".into()),
                UnitDirective::new("Unit".into(), "AssertPathExists".into(),"/srv/www".into()),
                UnitDirective::new("Service".into(), "Type".into(), Some("notify".into())),
                UnitDirective::new("Service".into(), "ExecStart".into(), Some("/usr/sbin/some-fancy-httpd-server".into())),
                UnitDirective::new("Service".into(), "Nice".into(), Some("0".into())),
                UnitDirective::new("Service".into(), "PrivateTmp".into(), Some("yes".into())),
                UnitDirective::new("Install".into(), "WantedBy".into(), Some("multi-user.target".into())),
            ];

            assert_eq!(expected, res.unwrap());
        }
    }
}

mod systemd_unit {
    pub use super::*;

    mod new {
        pub use super::*;

        #[test]
        fn it_should_instantiate() {
            let input = vec![
                Category("Unit"),
                Directive("Description", Some("A dummy unit file")),
            ];

            let res = SystemdUnit::new(&input);

            assert!(res.is_ok())
        }

        #[test]
        fn it_should_err_when_not_starting_with_a_category() {
            let input = vec![
                Directive("Description", Some("A dummy unit file")),
            ];

            let res = SystemdUnit::new(&input);

            assert!(res.is_err())
        }

        #[test]
        fn it_should_err_when_many_directives_of_same_key_are_in_different_categories() {
            // arrange
            let input = vec![
                Category("Service"),
                Directive("ExecStartPre", Some("/usr/bin/true")),
                Category("Install"),
                Directive("ExecStartPre", Some("/usr/bin/true")),
            ];
            // act
            let res = SystemdUnit::new(&input);
            // assert
            assert!(res.is_err());
        }

        #[test]
        fn it_should_err_when_many_directives_of_same_key_are_in_different_categories2() {
            // arrange
            let input = vec![
                Category("Service"),
                Directive("ExecStartPre", Some("/usr/bin/true")),
                Directive("ExecStartPre", Some("/usr/bin/true")),
                Category("Install"),
                Directive("ExecStartPre", Some("/usr/bin/true")),
            ];
            // act
            let res = SystemdUnit::new(&input);
            // assert
            assert!(res.is_err());
        }
    }

    mod lookup_by_key {
        pub use super::*;

        #[test]
        fn it_should_return_the_unit_directive_pointed_by_key() {
            // arrange
            let input = vec![
                Category("Unit"),
                Directive("Description", Some("A dummy unit file")),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let directive = Solo(UnitDirective::new("Unit", "Description", Some("A dummy unit file")));
            let expected = Some(&directive);
            // act
            let res = unit.lookup_by_key("Description");
            // assert
            assert_eq!(expected, res);
        }

        #[test]
        fn it_should_return_none_when_inexistent() {
            // arrange
            let input = vec![
                Category("Service"),
                Directive("ExecStartPre", Some("/usr/bin/true")),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let expected = None;
            // act
            let res = unit.lookup_by_key("Description");
            // assert
            assert_eq!(expected, res);
        }

        #[test]
        fn it_should_return_a_vec_of_all_the_keys_when_multiple_keys() {
            // arrange
            let input = vec![
                Category("Service"),
                Directive("ExecStartPre", Some("/usr/bin/true")),
                Directive("ExecStartPre", Some("/usr/bin/true")),
                Directive("ExecStartPre", Some("/usr/bin/true")),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let directive = UnitDirective::new("Service", "ExecStartPre", Some("/usr/bin/true"));
            let expected = Many(vec![
                directive.clone(),
                directive.clone(),
                directive.clone(),
            ]);
            // act
            let res = unit.lookup_by_key("ExecStartPre");
            // assert
            assert_eq!(Some(&expected), res);
        }
    }

    mod lookup_by_category {
        pub use super::*;

        #[test]
        fn it_should_exists() {
            // arrange
            let input = vec![
                Category("Unit"),
                Directive("Description", Some("A dummy unit file")),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let directive = Solo(UnitDirective::new("Unit", "Description", Some("A dummy unit file")));
            let expected = vec![
                &directive
            ];
            // act
            let res = unit.lookup_by_category("Unit");
            // assert
            assert_eq!(expected, res);
        }

        #[test]
        fn it_should_return_an_empty_vec_the_category_doesnt_exists() {
            // arrange
            let input = vec![
                Category("Unit"),
                Directive("Description", Some("A dummy unit file")),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let expected : Vec<&DirectiveEntry> = vec![];
            // act
            let res = unit.lookup_by_category("Service");
            // assert
            assert_eq!(expected, res);
        }
    }

    mod categories {
        pub use super::*;

        #[test]
        fn it_should_return_categories() {
            // arrange
            let input = vec![
                Category("Unit0"),
                Directive("Description0", Some("A dummy unit file")),
                Category("Unit1"),
                Directive("Description1", Some("A dummy unit file")),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let expected : Vec<String> = vec![
                "Unit0".into(),
                "Unit1".into(),
            ];
            // act
            let res = unit.categories();
            // assert
            assert_eq!(expected, res);
        }

        #[test]
        fn it_should_skip_doubles_and_return_categories_once() {
            // arrange
            let input = vec![
                Category("Unit0"),
                Directive("Description00", Some("A dummy unit file")),
                Directive("Description01", Some("A dummy unit file")),
                Category("Unit1"),
                Directive("Description11", Some("A dummy unit file")),
                Directive("Description12", Some("A dummy unit file")),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let expected : Vec<String> = vec![
                "Unit0".into(),
                "Unit1".into(),
            ];
            // act
            let res = unit.categories();
            // assert
            assert_eq!(expected, res);
        }
    }
}
