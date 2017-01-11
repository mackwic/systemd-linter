
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
                Directive("Description", "plop"),
            ];
            let res = UnitDirective::item_list_to_unit_directive_list(&input);
            let expected = UnitDirective::new("Unit".into(), "Description".into(), "plop".into());

            assert!(res.is_ok());
            assert_eq!(Some(&expected), res.unwrap().get(0))
        }

        #[test]
        fn the_second_directive_should_keep_the_category() {
            let input = vec![
                Category("Unit"),
                Directive("Description", "plop"),
                Directive("Wants", "boot.target"),
            ];
            let res = UnitDirective::item_list_to_unit_directive_list(&input);
            let expected = UnitDirective::new("Unit".into(), "Wants".into(), "boot.target".into());

            assert!(res.is_ok());
            assert_eq!(Some(&expected), res.unwrap().get(1))
        }

        #[test]
        fn it_should_change_the_category_if_a_category_item_is_seen() {
            let input = vec![
                Category("Unit"),
                Directive("Description", "plop"),
                Category("Service"),
                Directive("ExecStart", "/usr/bin/true"),
            ];
            let res = UnitDirective::item_list_to_unit_directive_list(&input);
            let expected = UnitDirective::new("Service".into(), "ExecStart".into(), "/usr/bin/true".into());

            assert!(res.is_ok());
            assert_eq!(Some(&expected), res.unwrap().get(1))
        }

        #[test]
        fn it_should_err_if_the_first_item_is_not_a_category() {
            let input = vec![
                Directive("Description", "plop"),
                Category("Service"),
                Directive("ExecStart", "/usr/bin/true"),
            ];
            let res = UnitDirective::item_list_to_unit_directive_list(&input);

            assert!(res.is_err());
        }

        #[test]
        fn it_should_work_with_full_dummy_unit() {
            let input = vec![
                Category("Unit"),
                Directive("Description", "Some HTTP server"),
                Directive("After", "remote-fs.target sqldb.service memcached.service"),
                Directive("Requires", ""),
                Directive("Requires","sqldb.service memcached.service"),
                Directive("AssertPathExists","/srv/www"),
                Category("Service"),
                Directive("Type", "notify"),
                Directive("ExecStart", "/usr/sbin/some-fancy-httpd-server"),
                Directive("Nice", "0"),
                Directive("PrivateTmp", "yes"),
                Category("Install"),
                Directive("WantedBy", "multi-user.target"),
            ];
            let res = UnitDirective::item_list_to_unit_directive_list(&input);

            let expected = vec![
                UnitDirective::new("Unit".into(), "Description".into(), "Some HTTP server".into()),
                UnitDirective::new("Unit".into(), "After".into(), "remote-fs.target sqldb.service memcached.service".into()),
                UnitDirective::new("Unit".into(), "Requires".into(), ""),
                UnitDirective::new("Unit".into(), "Requires".into(),"sqldb.service memcached.service".into()),
                UnitDirective::new("Unit".into(), "AssertPathExists".into(),"/srv/www".into()),
                UnitDirective::new("Service".into(), "Type".into(), "notify".into()),
                UnitDirective::new("Service".into(), "ExecStart".into(), "/usr/sbin/some-fancy-httpd-server".into()),
                UnitDirective::new("Service".into(), "Nice".into(), "0".into()),
                UnitDirective::new("Service".into(), "PrivateTmp".into(), "yes".into()),
                UnitDirective::new("Install".into(), "WantedBy".into(), "multi-user.target".into()),
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
                Directive("Description", "A dummy unit file"),
            ];

            let res = SystemdUnit::new(&input);

            assert!(res.is_ok())
        }

        #[test]
        fn it_should_err_when_not_starting_with_a_category() {
            let input = vec![
                Directive("Description", "A dummy unit file"),
            ];

            let res = SystemdUnit::new(&input);

            assert!(res.is_err())
        }

        #[test]
        fn it_should_err_when_many_directives_of_same_key_are_in_different_categories() {
            // arrange
            let input = vec![
                Category("Service"),
                Directive("ExecStartPre", "/usr/bin/true"),
                Category("Install"),
                Directive("ExecStartPre", "/usr/bin/true"),
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
                Directive("ExecStartPre", "/usr/bin/true"),
                Directive("ExecStartPre", "/usr/bin/true"),
                Category("Install"),
                Directive("ExecStartPre", "/usr/bin/true"),
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
                Directive("Description", "A dummy unit file"),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let directive = Solo(UnitDirective::new("Unit", "Description", "A dummy unit file"));
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
                Directive("ExecStartPre", "/usr/bin/true"),
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
                Directive("ExecStartPre", "/usr/bin/true"),
                Directive("ExecStartPre", "/usr/bin/true"),
                Directive("ExecStartPre", "/usr/bin/true"),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let directive = UnitDirective::new("Service", "ExecStartPre", "/usr/bin/true");
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
                Directive("Description", "A dummy unit file"),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let directive = Solo(UnitDirective::new("Unit", "Description", "A dummy unit file"));
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
                Directive("Description", "A dummy unit file"),
            ];
            let unit = SystemdUnit::new(&input).unwrap();
            let expected : Vec<&DirectiveEntry> = vec![];
            // act
            let res = unit.lookup_by_category("Service");
            // assert
            assert_eq!(expected, res);
        }
    }
}
