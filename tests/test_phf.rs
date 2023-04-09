mod data;

#[cfg(feature = "phf")]
mod test {
    use super::data::{
        TEST_CHARS_DELETE, TEST_CHARS_PASS, TEST_CHARS_SUB_X,
        TEST_STRINGS_DELETE, TEST_STRINGS_PASS, TEST_STRINGS_SUB_X,
    };
    use charmap::{CharMapAction, CharMapper, MapCharsIter};
    use phf::{phf_map, phf_ordered_map};

    // TODO: Use mappings from data. Though this might require a build script.
    static HASH_CHARMAP: phf::Map<char, CharMapAction> = phf_map! {
        'd' => CharMapAction::SubChar('m'),
        'e' => CharMapAction::SubStr("eeee"),
        'l' => CharMapAction::Delete,
        'o' => CharMapAction::Pass,
    };

    // TODO: Use mappings from data. Though this might require a build script.
    static ORDERED_CHARMAP: phf::OrderedMap<char, CharMapAction> = phf_ordered_map! {
        'd' => CharMapAction::SubChar('m'),
        'e' => CharMapAction::SubStr("eeee"),
        'l' => CharMapAction::Delete,
        'o' => CharMapAction::Pass,
    };

    #[test]
    fn phf_hashmap_default_pass() {
        let mapper = CharMapper::new(&HASH_CHARMAP, CharMapAction::Pass);

        for (input, expected) in TEST_STRINGS_PASS {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);

            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }

        for (input, expected) in TEST_CHARS_PASS {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_hashmap_default_delete() {
        let mapper = CharMapper::new(&HASH_CHARMAP, CharMapAction::Delete);

        for (input, expected) in TEST_STRINGS_DELETE {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);

            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }

        for (input, expected) in TEST_CHARS_DELETE {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_hashmap_default_sub_x() {
        let mapper =
            CharMapper::new(&HASH_CHARMAP, CharMapAction::SubStr("x"));

        for (input, expected) in TEST_STRINGS_SUB_X {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);

            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }

        for (input, expected) in TEST_CHARS_SUB_X {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_hashmap_default_sub_empty() {
        let mapper = CharMapper::new(&HASH_CHARMAP, CharMapAction::SubStr(""));

        for (input, expected) in TEST_STRINGS_DELETE {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);

            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }

        for (input, expected) in TEST_CHARS_DELETE {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_orderedmap_default_pass() {
        let mapper = CharMapper::new(&ORDERED_CHARMAP, CharMapAction::Pass);

        for (input, expected) in TEST_STRINGS_PASS {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);

            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }

        for (input, expected) in TEST_CHARS_PASS {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_orderedmap_default_delete() {
        let mapper = CharMapper::new(&ORDERED_CHARMAP, CharMapAction::Delete);

        for (input, expected) in TEST_STRINGS_DELETE {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);

            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }

        for (input, expected) in TEST_CHARS_DELETE {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_orderedmap_default_sub_x() {
        let mapper =
            CharMapper::new(&ORDERED_CHARMAP, CharMapAction::SubStr("x"));

        for (input, expected) in TEST_STRINGS_SUB_X {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);

            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }

        for (input, expected) in TEST_CHARS_SUB_X {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_orderedmap_default_sub_empty() {
        let mapper =
            CharMapper::new(&ORDERED_CHARMAP, CharMapAction::SubStr(""));

        for (input, expected) in TEST_STRINGS_DELETE {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);

            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }

        for (input, expected) in TEST_CHARS_DELETE {
            let mapped: String = input.map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }
}
