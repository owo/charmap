mod data;

#[cfg(feature = "hashbrown")]
mod test {
    use super::data::{
        TEST_MAPPING, TEST_STRINGS_DELETE, TEST_STRINGS_PASS,
        TEST_STRINGS_SUB_X,
    };
    use charmap::{CharMapAction, CharMapper, MapCharsIter};
    use hashbrown::HashMap;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref HASH_CHARMAP: HashMap<char, CharMapAction<'static>> =
            HashMap::from(TEST_MAPPING);
    }

    #[test]
    fn hashbrown_hashmap_default_pass() {
        let mapper = CharMapper::new(&*HASH_CHARMAP, CharMapAction::Pass);

        for (input, expected) in TEST_STRINGS_PASS {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn hashbrown_hashmap_default_delete() {
        let mapper = CharMapper::new(&*HASH_CHARMAP, CharMapAction::Delete);

        for (input, expected) in TEST_STRINGS_DELETE {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn hashbrown_hashmap_default_sub_x() {
        let mapper = CharMapper::new(&*HASH_CHARMAP, CharMapAction::Sub("x"));

        for (input, expected) in TEST_STRINGS_SUB_X {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }

    #[test]
    fn hashbrown_hashmap_default_sub_empty() {
        let mapper = CharMapper::new(&*HASH_CHARMAP, CharMapAction::Sub(""));

        for (input, expected) in TEST_STRINGS_DELETE {
            let mapped: String = input.chars().map_chars(&mapper).collect();
            assert_eq!(mapped, expected);
        }
    }
}
