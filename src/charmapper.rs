use core::{option, str::Chars};

use super::actionmap::{ActionMap, CharMapAction};

/// Primary struct used for character mapping.
pub struct CharMapper<'a, M>
where
    M: ActionMap,
{
    actionmap: &'a M,
    default: CharMapAction<'a>,
}

impl<'a, M> CharMapper<'a, M>
where
    M: ActionMap,
{
    /// Creates a new [`CharMapper`] with a given [`ActionMap`] and a default
    /// action to take if a character is not in the given [`ActionMap`].
    #[inline]
    pub fn new(actionmap: &'a M, default: CharMapAction<'a>) -> Self {
        CharMapper { actionmap, default: default }
    }

    /// Returns the [`CharMapAction`] asscociated with a given character.
    #[inline]
    pub fn get_action(&'a self, c: char) -> &'a CharMapAction<'a> {
        match self.actionmap.map_char(c) {
            None => &self.default,
            Some(action) => action,
        }
    }

    /// Returns an iterator that maps characters from a given character
    /// iterator.
    #[inline]
    pub fn map_chars_iter<I>(&'a self, text_chars: I) -> MappedChars<'a, M, I>
    where
        I: Iterator<Item = char>,
    {
        MappedChars::new(self, text_chars)
    }
}

/// Character iterator returned by
/// [`CharMapper::map_chars_iter`](super::CharMapper::map_chars_iter) and
/// [`MapCharsIter::map_chars`](super::MapCharsIter::map_chars).
#[derive(Clone)]
pub struct MappedChars<'a, M, I>
where
    M: ActionMap,
    I: Iterator<Item = char>,
{
    charmapper: &'a CharMapper<'a, M>,
    text_chars: I,
    sub_chars: Chars<'a>,
    in_sub: bool,
}

impl<'a, M, I> MappedChars<'a, M, I>
where
    M: ActionMap,
    I: Iterator<Item = char>,
{
    #[inline]
    pub(self) fn new(charmapper: &'a CharMapper<'a, M>, text_chars: I) -> Self
    where
        I: Iterator<Item = char>,
    {
        MappedChars {
            charmapper: charmapper,
            text_chars: text_chars,
            sub_chars: "".chars(),
            in_sub: false,
        }
    }
}

impl<'a, M, I> Iterator for MappedChars<'a, M, I>
where
    M: ActionMap,
    I: Iterator<Item = char>,
{
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // Check if we're currently outputing characters from sub string.
        if self.in_sub {
            match self.sub_chars.next() {
                Some(c) => return Some(c),
                None => self.in_sub = false,
            }
        }

        // We need to keep looping through the input until the we got a
        // non-delete action.
        loop {
            let next_char = self.text_chars.next();

            match next_char {
                Some(c) => {
                    match self.charmapper.get_action(c) {
                        CharMapAction::Pass => return Some(c),
                        CharMapAction::Delete => continue,
                        CharMapAction::Sub(sub_str) => {
                            self.in_sub = true;
                            self.sub_chars = sub_str.chars();
                            // Can't unwrap directly because sub string could
                            // be empty.
                            match self.sub_chars.next() {
                                Some(c) => return Some(c),
                                None => {
                                    self.in_sub = false;
                                    continue;
                                }
                            }
                        }
                    }
                }
                None => return None,
            }
        }
    }
}

/// A trait providing a convenience method for [`Iterators`](Iterator) of
/// [`char`] to charmap their output.
pub trait MapCharsIter<'a, M, I: Iterator<Item = char>>
where
    M: ActionMap,
{
    fn map_chars(self, mapper: &'a CharMapper<'a, M>)
        -> MappedChars<'a, M, I>;
}

impl<'a, M> MapCharsIter<'a, M, Chars<'a>> for &'a str
where
    M: ActionMap,
{
    #[inline]
    fn map_chars(
        self,
        mapper: &'a CharMapper<'a, M>,
    ) -> MappedChars<'a, M, Chars<'a>>
    where
        Self: Sized,
    {
        MappedChars::new(mapper, self.chars())
    }
}

impl<'a, M> MapCharsIter<'a, M, option::IntoIter<char>> for char
where
    M: ActionMap,
{
    #[inline]
    fn map_chars(
        self,
        mapper: &'a CharMapper<'a, M>,
    ) -> MappedChars<'a, M, option::IntoIter<char>>
    where
        Self: Sized,
    {
        MappedChars::new(mapper, Some(self).into_iter())
    }
}

impl<'a, M, I: Iterator<Item = char>> MapCharsIter<'a, M, I> for I
where
    M: ActionMap,
{
    #[inline]
    fn map_chars(
        self,
        mapper: &'a CharMapper<'a, M>,
    ) -> MappedChars<'a, M, Self>
    where
        Self: Sized,
    {
        MappedChars::new(mapper, self)
    }
}
