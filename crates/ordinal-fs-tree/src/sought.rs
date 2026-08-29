//! The word for a search that matched nothing.
//!
//! The library already has one negative answer — [`Refusal`](crate::Refusal) —
//! and every one of its variants is a refusal to *mutate*. A search is not a
//! mutation: nothing was asked to change, so nothing can have been refused, and
//! nothing is wrong with the tree either. Answering a search with `None` leaves
//! that distinction unsaid, and a consumer then invents a word for it in its own
//! vocabulary — which is how the same concept ends up with one name per caller.
//!
//! [`Sought`] is that word, in the library's vocabulary, and it is the answer to
//! *every* search on the public surface: `ARCHITECTURE.md`'s *Reading* table has
//! two rows that can match nothing, [`Snapshot::seek`](crate::Snapshot::seek)
//! and [`Snapshot::by_key`](crate::Snapshot::by_key), and both answer with this.
//!
//! # What is not a search
//!
//! An accessor that reads an attribute off something already in hand is not one,
//! and it keeps its `Option`: [`Entry::key`](crate::Entry::key) is absent for the
//! distinguished child because that name carries no key, and
//! [`Entry::contents`](crate::Entry::contents) is absent for a leaf because a
//! leaf holds nothing. Neither scanned anything. The test is whether a criterion
//! was supplied and a set was scanned for it — `Sought` says *that scan
//! completed and matched nothing*, which is a fact about the search, where
//! `None` on an accessor is a fact about the entry.
//!
//! # `Option` is a door, not a return type
//!
//! What a consumer *does* about a search that matched nothing is the consumer's
//! own, and this library states no policy over it. The reference CLI is both
//! answers at once: `show` turns it into a
//! [`Refusal::TargetMissing`](crate::Refusal::TargetMissing) it constructs
//! itself, and `list --first` renders the identical answer as an empty listing.
//! Most of that is a `match` or a `let … else`, and wants nothing from this
//! module. [`Sought::into_option`] and the two [`From`]
//! impls beside it are for the rest — a caller already mid-chain in `Option`'s
//! combinators, which the language has and this crate is not about to reproduce.
//! What the door does *not* do is put `Option` back in a signature the library
//! owns.

/// A search's answer: what it matched, or nothing.
///
/// **Not a refusal.** Nothing was asked to change, and nothing is wrong with the
/// tree — see this module's header for the distinction and for what is not a
/// search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use]
pub enum Sought<T> {
    /// The search matched, and this is what it matched.
    Match(T),
    /// The search completed and matched nothing.
    Nothing,
}

impl<T> Sought<T> {
    /// Whether the search matched.
    #[must_use]
    pub const fn is_match(&self) -> bool {
        matches!(self, Self::Match(_))
    }

    /// Whether the search matched nothing.
    #[must_use]
    pub const fn is_nothing(&self) -> bool {
        matches!(self, Self::Nothing)
    }

    /// The match, or `None`.
    ///
    /// The deliberate door out of this vocabulary and into the caller's own —
    /// see this module's header. [`Option::from`] is the same conversion for a
    /// caller who prefers it that way round; this spelling exists because the
    /// other one reads badly mid-chain.
    #[must_use]
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Match(found) => Some(found),
            Self::Nothing => None,
        }
    }

    /// Apply a function to the match, keeping the answer a search's answer.
    ///
    /// The one combinator here, and it earns its place by being the one that
    /// does *not* leave: mapping through [`Sought::into_option`] would answer a
    /// search in `Option`'s vocabulary for no reason but the shape of the
    /// function applied.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Sought<U> {
        match self {
            Self::Match(found) => Sought::Match(f(found)),
            Self::Nothing => Sought::Nothing,
        }
    }

    /// The match, panicking with `message` when the search matched nothing.
    ///
    /// For a caller that has already established the match exists — a test over
    /// a tree it built itself, most of all. A caller that has not should be
    /// matching on the two variants, which is why there is no `unwrap` beside
    /// this one: an unwrap has no room to say what it was relying on.
    ///
    /// # Panics
    ///
    /// When the search matched nothing.
    #[must_use]
    pub fn expect(self, message: &str) -> T {
        match self {
            Self::Match(found) => found,
            Self::Nothing => panic!("{message}"),
        }
    }
}

/// **`.into()` needs a concrete target here.** `core` already has
/// `impl<T> From<T> for Option<T>` — the blanket that makes `x.into()` produce
/// `Some(x)` — so against an inferred `Option<_>` both impls apply and the call
/// is ambiguous. Against a spelled-out `Option<Entry<'_, N>>` only this one can
/// match, and it resolves. [`Sought::into_option`] never has the problem, which
/// is the other half of why it exists.
impl<T> From<Sought<T>> for Option<T> {
    fn from(sought: Sought<T>) -> Self {
        sought.into_option()
    }
}

impl<T> From<Option<T>> for Sought<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(found) => Self::Match(found),
            None => Self::Nothing,
        }
    }
}
