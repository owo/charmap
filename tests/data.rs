use charmap::CharMapAction;

#[allow(dead_code)]
pub const TEST_MAPPING: [(char, CharMapAction); 3] = [
    ('e', CharMapAction::Sub("eeee")),
    ('l', CharMapAction::Delete),
    ('o', CharMapAction::Pass),
];

#[allow(dead_code)]
pub const TEST_STRINGS_PASS: [(&str, &str); 6] = [
    ("", ""),
    ("lllllll", ""),
    ("ooooo", "ooooo"),
    ("teehee", "teeeeeeeeheeeeeeee"),
    ("Hello, world!", "Heeeeo, word!"),
    ("Foo Bar", "Foo Bar"),
];

#[allow(dead_code)]
pub const TEST_STRINGS_DELETE: [(&str, &str); 6] = [
    ("", ""),
    ("lllllll", ""),
    ("ooooo", "ooooo"),
    ("teehee", "eeeeeeeeeeeeeeee"),
    ("Hello, world!", "eeeeoo"),
    ("Foo Bar", "oo"),
];

#[allow(dead_code)]
pub const TEST_STRINGS_SUB_X: [(&str, &str); 6] = [
    ("", ""),
    ("lllllll", ""),
    ("ooooo", "ooooo"),
    ("teehee", "xeeeeeeeexeeeeeeee"),
    ("Hello, world!", "xeeeeoxxxoxxx"),
    ("Foo Bar", "xooxxxx"),
];

#[allow(dead_code)]
pub const TEST_CHARS_PASS: [(char, &str); 5] =
    [('m', "m"), ('e', "eeee"), ('l', ""), ('o', "o"), ('n', "n")];

#[allow(dead_code)]
pub const TEST_CHARS_DELETE: [(char, &str); 5] =
    [('m', ""), ('e', "eeee"), ('l', ""), ('o', "o"), ('n', "")];

#[allow(dead_code)]
pub const TEST_CHARS_SUB_X: [(char, &str); 5] =
    [('m', "x"), ('e', "eeee"), ('l', ""), ('o', "o"), ('n', "x")];
