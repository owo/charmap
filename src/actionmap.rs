/// An enum representing an action to be taken by a
/// [`CharMapper`](super::CharMapper) when it sees a certain [`char`].
#[derive(Clone, Copy, Debug)]
pub enum CharMapAction<'a> {
    /// Output the character as is.
    Pass,
    /// Delete the character.
    Delete,
    /// Substitute the character with a string.
    SubStr(&'a str),
    /// Substitute the character with another character.
    SubChar(char),
}

/// A trait that should be implemented by any struct providing char-to-action
/// mappings for [`CharMapper`](super::CharMapper).
pub trait ActionMap {
    /// Map a character to its respective CharMapAction.
    fn map_char<'a>(&'a self, c: char) -> Option<&'a CharMapAction<'a>>;
}

// ====== ActionMap implementations for most commonly used maps ====== //

#[cfg(feature = "std")]
impl ActionMap for std::collections::HashMap<char, CharMapAction<'_>> {
    #[inline]
    fn map_char<'a>(&'a self, c: char) -> Option<&'a CharMapAction<'a>> {
        self.get(&c)
    }
}

#[cfg(feature = "std")]
impl ActionMap for std::collections::BTreeMap<char, CharMapAction<'_>> {
    #[inline]
    fn map_char<'a>(&'a self, c: char) -> Option<&'a CharMapAction<'a>> {
        self.get(&c)
    }
}

#[cfg(feature = "hashbrown")]
impl ActionMap for hashbrown::HashMap<char, CharMapAction<'_>> {
    #[inline]
    fn map_char<'a>(&'a self, c: char) -> Option<&'a CharMapAction<'a>> {
        self.get(&c)
    }
}

#[cfg(feature = "phf")]
impl ActionMap for phf::Map<char, CharMapAction<'_>> {
    #[inline]
    fn map_char<'a>(&'a self, c: char) -> Option<&'a CharMapAction<'a>> {
        self.get(&c)
    }
}

#[cfg(feature = "phf")]
impl ActionMap for phf::OrderedMap<char, CharMapAction<'_>> {
    #[inline]
    fn map_char<'a>(&'a self, c: char) -> Option<&'a CharMapAction<'a>> {
        self.get(&c)
    }
}
