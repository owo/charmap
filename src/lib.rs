//! # charmap
//!
//! A Rust library for one-to-(none/one/many) character mapping.
//! It's main use-case is preprocessing, transliterating, and cleaning natural
//! language text.
//!
//! ## Usage
//!
//! To use `charmap` with libstd's mapping types
//! ([`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and
//! [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)),
//! add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! charmap = "0.1"
//! ```
//!
//! This should also allow you to use
//! [`rustc-hash`](https://crates.io/crates/rustc-hash)'s
//! [`FxHashMap`](https://docs.rs/rustc-hash/latest/rustc_hash/type.FxHashMap.html)
//! since it is an instance of libstd's
//! [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
//!
//! `charmap` also supports [`hashbrown`](https://crates.io/crates/hashbrown)'s
//! [`HashMap`](https://docs.rs/hashbrown/0.13.2/hashbrown/struct.HashMap.html) and
//! [`phf`](https://crates.io/crates/phf)'s
//! [`Map`](https://docs.rs/phf/latest/phf/struct.Map.html) and
//! [`OrderedMap`](https://docs.rs/phf/latest/phf/struct.OrderedMap.html) types.
//! You can enable these by setting the `"hashbrown"` and `"phf"` features
//! respectively.
//! For example, to use `charmap` with [`phf`](https://crates.io/crates/phf),
//! add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! charmap = {version = "0.1", features = ["phf"]}
//! ```
//!
//! You can also disable libstd support for `no_std` builds by setting
//! `default-features = false`. For example:
//!
//! ```toml
//! [dependencies]
//! charmap = {version = "0.1", default-features = false, features = ["phf"]}
//! ```
//!
//! ## Example
//!
//! Below is an example of how to use `charmap` with libstd's
//! [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html):
//!
//! ```rust
//! use std::collections::HashMap;
//! use charmap::*;
//!
//! // We first define our action map that will tell our CharMapper what to do
//! // when it sees a particular character.
//! let actions = HashMap::from([
//!     ('!', CharMapAction::Delete),  // Delete instances of '!'
//!     ('l', CharMapAction::SubStr("LLL")),  // Substitute instances of 'l' with 'LLL'
//! ]);
//!
//! // This is the string we want to charmap.
//! let start_str = "Hello, world!";
//!
//! // Create a character mapper using the previously defined actions while
//! // allowing all other character to be output as they are.
//! let mapper = CharMapper::new(&actions, CharMapAction::Pass);
//!
//! // Use mapper to charmap start_str
//! let mapped_str: String = start_str.map_chars(&mapper).collect();
//!
//! // Output should be: HeLLLLLLo, worLLLd
//! println!("{}", mapped_str);
//! ```
//!
//! We can also use a different default action to apply to characters not
//! defined in our actions map:
//!
//! ```rust
//! use std::collections::HashMap;
//! use charmap::*;
//!
//! // We first define our action map that will tell our CharMapper what to do
//! // when it sees a particular character.
//! let actions = HashMap::from([
//!     ('!', CharMapAction::Delete),  // Delete instances of '!'
//!     ('l', CharMapAction::SubStr("LLL")),  // Substitute instances of 'l' with 'LLL'
//!     ('o', CharMapAction::Pass),  // Output instances of 'o' as is
//! ]);
//!
//! // This is the string we want to charmap.
//! let start_str = "Hello, world!";
//!
//! // Create a character mapper using the previously defined actions while
//! // replacing all other characters with "-".
//! let mapper = CharMapper::new(&actions, CharMapAction::SubChar('-'));
//!
//! // Use mapper to charmap start_str
//! let mapped_str: String = start_str.map_chars(&mapper).collect();
//!
//! // Output should be: --LLLLLLo---o-LLL-
//! println!("{}", mapped_str);
//! ```

mod actionmap;
mod charmapper;

pub use crate::actionmap::{ActionMap, CharMapAction};
pub use crate::charmapper::{CharMapper, MapCharsIter, MappedChars};
