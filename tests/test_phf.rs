mod data;

#[cfg(feature = "phf")]
mod test {
    use super::data::{
        TEST_STRINGS_DELETE, TEST_STRINGS_PASS, TEST_STRINGS_SUB_X,
    };
    use charmap::{CharMapAction, CharMapper, MapCharsIter};
    use phf::{phf_map, phf_ordered_map};

    // TODO: Use mappings from data. Though this might require a build script.
    static HASH_CHARMAP: phf::Map<char, CharMapAction> = phf_map! {
        'e' => CharMapAction::Sub("eeee"),
        'l' => CharMapAction::Delete,
        'o' => CharMapAction::Pass,
    };

    // TODO: Use mappings from data. Though this might require a build script.
    static ORDERED_CHARMAP: phf::OrderedMap<char, CharMapAction> = phf_ordered_map! {
        'e' => CharMapAction::Sub("eeee"),
        'l' => CharMapAction::Delete,
        'o' => CharMapAction::Pass,
    };

    #[test]
    fn phf_hashmap_default_pass() {
        let mapper = CharMapper::new(&HASH_CHARMAP, CharMapAction::Pass);

        for (input, expected) in TEST_STRINGS_PASS {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_hashmap_default_delete() {
        let mapper = CharMapper::new(&HASH_CHARMAP, CharMapAction::Delete);

        for (input, expected) in TEST_STRINGS_DELETE {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_hashmap_default_sub_x() {
        let mapper = CharMapper::new(&HASH_CHARMAP, CharMapAction::Sub("x"));

        for (input, expected) in TEST_STRINGS_SUB_X {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_hashmap_default_sub_empty() {
        let mapper = CharMapper::new(&HASH_CHARMAP, CharMapAction::Sub(""));

        for (input, expected) in TEST_STRINGS_DELETE {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_orderedmap_default_pass() {
        let mapper = CharMapper::new(&ORDERED_CHARMAP, CharMapAction::Pass);

        for (input, expected) in TEST_STRINGS_PASS {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_orderedmap_default_delete() {
        let mapper = CharMapper::new(&ORDERED_CHARMAP, CharMapAction::Delete);

        for (input, expected) in TEST_STRINGS_DELETE {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_orderedmap_default_sub_x() {
        let mapper =
            CharMapper::new(&ORDERED_CHARMAP, CharMapAction::Sub("x"));

        for (input, expected) in TEST_STRINGS_SUB_X {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn phf_orderedmap_default_sub_empty() {
        let mapper = CharMapper::new(&ORDERED_CHARMAP, CharMapAction::Sub(""));

        for (input, expected) in TEST_STRINGS_DELETE {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }
}
