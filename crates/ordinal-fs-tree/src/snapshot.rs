//! The snapshot: a whole tree of names in memory, and the five reading
//! operations over it.
//!
//! Everything here is **pure**. A snapshot is built by handing it names, one
//! level at a time, and nothing in this module knows where those names came
//! from — `src/fs/` reads a directory and feeds this, and a test builds a tree
//! by hand and feeds the same thing. That is what makes walk order testable
//! without a directory, and it is the boundary
//! `tests/algebra_has_no_filesystem.rs` holds.
//!
//! [`Builder`] is crate-private, so those tests are in `tests.rs` beside this
//! file rather than in `tests/`. That is the shape `reading-k20` chose over
//! publishing the construction arena: the public surface is what a consumer
//! needs, and a consumer reads a tree from a directory.
//!
//! The specification is `docs/ordinal-fs-tree/ARCHITECTURE.md`, sections
//! *Operations → Reading* and *How an operation runs*.
//!
//! # Walk order is prose, and this module owns it
//!
//! Neither model checks it. `operations.qnt` models *reachability* — which
//! entries a walk reaches — and says so in its handoff block; the ordering is
//! unmodelled, and the model resolves `by_key` on a duplicate-key tree by
//! picking the least internal id rather than the first in walk order. So the
//! order below is implemented from the document, tested against hand-built
//! trees, and **not** described anywhere as checked.
//!
//! Within one level: the distinguished child first, then the positioned
//! children by ordinal. Two further tie-breaks exist because a level may be
//! hand-edited into carrying a duplicate ordinal, and because the order a
//! directory listing arrives in is arbitrary: equal ordinals are ordered by
//! key, and equal ordinals *and* keys by the rendered name, which is total
//! because two entries in one directory cannot share a filename. Without a
//! total order over a level, *the first in walk order* would name a different
//! entry on two filesystems holding byte-identical trees.

use crate::plan::Level;
use crate::{EntryName, EntryNameExt, Key, NameView, Ordinal, Sought, Species, Triple};

/// One entry as the snapshot holds it: its name, where it sits, and — when it
/// is a node — what is under it.
struct EntryData<N> {
    name: N,
    /// The entry whose directory holds this one; `None` for a child of the
    /// tree root, which is a node and not an entry.
    parent: Option<usize>,
    /// `Some` exactly when this entry is a node.
    contents: Option<Directory>,
    /// Distance from the tree root: a child of the root has depth 1.
    depth: usize,
}

/// One level: the children of the root, or the children of a node.
///
/// A single ordered list rather than a distinguished child beside a list of
/// siblings, and the difference is load-bearing. A domain that broke the
/// obligation *`distinguished()` names the only entry of its species* would
/// hand this level two distinguished children, and a single `Option` slot would
/// have to drop one of them — a name silently missing from every traversal,
/// which is precisely the failure the parse trichotomy exists to prevent. Held
/// as a list, an extra distinguished child is visible in walk order like
/// anything else, and the conformance kit is what refuses the domain.
#[derive(Default)]
struct Directory {
    /// Every child of this level, already in walk order.
    children: Vec<usize>,
}

/// A whole tree of names, read once.
///
/// Snapshot scope is the whole tree, deliberately: it is why a single name the
/// consumer recognises and cannot parse halts every operation, wherever in the
/// tree it sits. Narrowing that later would be an invisible refinement;
/// widening it would not.
pub struct Snapshot<N> {
    entries: Vec<EntryData<N>>,
    root: Directory,
}

/// Where in a snapshot under construction the next name goes: the tree root, or
/// a node already added.
///
/// Handed out by [`Builder::add`] and by [`Builder::root`], and by nothing else,
/// so a place naming a leaf cannot be written — a leaf is a regular file and
/// holds nothing.
///
/// A place that names a node carries **which builder handed it out**, and not
/// only where in that builder's arena the node sits. `reading-k19` found the
/// version that carried the index alone: two builders each holding a node at
/// arena index 0 made a place from one silently name the other's node, and the
/// promised panic fired only when the index happened to be absent or to land on
/// something that was not a node. A construction seam that can quietly build a
/// different tree from the one it was asked for is worse than one that refuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Place(Option<(BuilderId, usize)>);

impl Place {
    /// The tree root.
    ///
    /// The one place that needs no builder identity: every builder has a root,
    /// and *this builder's root* is the only thing a caller can mean by it.
    pub(crate) const ROOT: Self = Self(None);
}

/// Which [`Builder`] a [`Place`] came from.
///
/// A counter and not a pointer: a builder moves — `finish` consumes it by value
/// — so its address is not its identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct BuilderId(u64);

impl BuilderId {
    fn next() -> Self {
        static COUNTER: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
    }
}

/// Builds a [`Snapshot`] from names, one level at a time.
///
/// This is the only way a snapshot is made, and it is **crate-private**. The
/// filesystem is not the only source of one — every test of walk order in this
/// crate builds its trees here, without a directory — but those are this crate's
/// own tests, and a test arrangement does not earn a production interface. The
/// pure tests of the algebra therefore live beside it, in this module, rather
/// than the arena being published so that `tests/` can reach it. `reading-k20`
/// made that call; it is not a decision `ARCHITECTURE.md` ever recorded, because
/// the construction seam was never part of the specified surface.
pub(crate) struct Builder<N> {
    id: BuilderId,
    entries: Vec<EntryData<N>>,
    root: Directory,
}

impl<N: EntryName> Default for Builder<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: EntryName> Builder<N> {
    /// An empty tree: a root holding nothing.
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            id: BuilderId::next(),
            entries: Vec::new(),
            root: Directory::default(),
        }
    }

    /// The tree root, to add the top level's names at.
    #[must_use]
    pub(crate) fn root(&self) -> Place {
        Place::ROOT
    }

    /// Add one name to a level, returning the place its own children go when it
    /// is a node.
    ///
    /// `None` for a leaf and for the distinguished child, both of which are
    /// regular files. The species comes from the name — from
    /// [`EntryName::positioned_species`] through [`EntryNameExt::species`] — so
    /// a caller never states it and cannot state it wrongly.
    ///
    /// # Panics
    ///
    /// If `at` came from a different builder. Deterministically, on the
    /// identity the place carries — never on the accident of an arena index
    /// being out of range or naming something that is not a node.
    pub(crate) fn add(&mut self, at: Place, name: N) -> Option<Place> {
        let is_node = name.species() == Species::Node;
        let depth = match self.node_at(at) {
            None => 1,
            Some(parent) => self.entries[parent].depth + 1,
        };
        let index = self.entries.len();
        self.entries.push(EntryData {
            name,
            parent: self.node_at(at),
            contents: is_node.then(Directory::default),
            depth,
        });
        self.directory_mut(at).children.push(index);
        is_node.then_some(Place(Some((self.id, index))))
    }

    /// The arena index a place names in *this* builder, or `None` for the root.
    ///
    /// # Panics
    ///
    /// If the place came from another builder.
    fn node_at(&self, at: Place) -> Option<usize> {
        match at.0 {
            None => None,
            Some((id, index)) => {
                assert!(
                    id == self.id,
                    "a place from another builder names nothing here: places are not \
                     interchangeable between builders, and attaching a child to whatever \
                     this builder happens to hold at the same index would build a \
                     different tree from the one described"
                );
                Some(index)
            }
        }
    }

    fn directory_mut(&mut self, at: Place) -> &mut Directory {
        match self.node_at(at) {
            None => &mut self.root,
            Some(index) => self.entries[index]
                .contents
                .as_mut()
                .expect("a place is only ever handed out for a node"),
        }
    }

    /// Put every level into walk order and freeze the tree.
    #[must_use]
    pub(crate) fn finish(mut self) -> Snapshot<N> {
        let mut levels: Vec<Vec<usize>> = Vec::with_capacity(self.entries.len() + 1);
        levels.push(core::mem::take(&mut self.root.children));
        for entry in &mut self.entries {
            if let Some(contents) = entry.contents.as_mut() {
                levels.push(core::mem::take(&mut contents.children));
            }
        }
        for level in &mut levels {
            sort_level(level, &self.entries);
        }
        let mut levels = levels.into_iter();
        self.root.children = levels.next().expect("the root level was pushed first");
        for entry in &mut self.entries {
            if let Some(contents) = entry.contents.as_mut() {
                contents.children = levels
                    .next()
                    .expect("one level per node, in the same order");
            }
        }
        Snapshot {
            entries: self.entries,
            root: self.root,
        }
    }
}

/// Put one level into the order a walk visits it in. See this module's header:
/// this is prose from the architecture document, checked by no model.
fn sort_level<N: EntryName>(level: &mut [usize], entries: &[EntryData<N>]) {
    level.sort_by(|a, b| {
        let (a, b) = (&entries[*a].name, &entries[*b].name);
        match (a.view(), b.view()) {
            // The distinguished child first. Two of them is a domain that broke
            // an obligation; ordering them by name keeps both visible and keeps
            // the order total.
            (NameView::Distinguished, NameView::Distinguished) => a.to_string().cmp(&b.to_string()),
            (NameView::Distinguished, NameView::Positioned(_)) => core::cmp::Ordering::Less,
            (NameView::Positioned(_), NameView::Distinguished) => core::cmp::Ordering::Greater,
            (NameView::Positioned(x), NameView::Positioned(y)) => x
                .ordinal
                .cmp(&y.ordinal)
                .then_with(|| x.key.cmp(&y.key))
                .then_with(|| a.to_string().cmp(&b.to_string())),
        }
    });
}

/// One entry in a snapshot.
///
/// A handle, not the entry itself: it borrows the snapshot, so it is `Copy` and
/// costs nothing to pass around.
pub struct Entry<'a, N> {
    snapshot: &'a Snapshot<N>,
    index: usize,
}

// By hand rather than by derive, for the reason `Triple` gives: a derive would
// generate `impl<N: Clone> Clone`, and this type holds a *reference* to the
// snapshot, so the bound is spurious — and a spurious bound on a public type
// propagates into consumers' signatures for no reason.
impl<N> Clone for Entry<'_, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N> Copy for Entry<'_, N> {}

impl<N> PartialEq for Entry<'_, N> {
    /// Two handles are equal when they name the same entry of the same
    /// snapshot. Identity, not name equality: a tree carrying a duplicate key
    /// holds two entries a domain would call equal, and telling them apart is
    /// the whole reason `by_key` has a documented tie-break.
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.snapshot, other.snapshot) && self.index == other.index
    }
}

impl<N> Eq for Entry<'_, N> {}

impl<N: EntryName> core::fmt::Debug for Entry<'_, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Entry")
            .field("name", &self.name().to_string())
            .field("depth", &self.depth())
            .finish()
    }
}

impl<'a, N: EntryName> Entry<'a, N> {
    fn data(&self) -> &'a EntryData<N> {
        &self.snapshot.entries[self.index]
    }

    /// Where this entry sits in its snapshot's own arena.
    ///
    /// Crate-private, and the reason it exists is [`Level::Entry`]: a plan is
    /// built before anything is applied and names levels by identity rather
    /// than by path, because half the levels a plan mentions do not exist yet
    /// when it is built. An index is that identity, and it is meaningless
    /// outside the snapshot it came from — which is why it does not cross the
    /// public surface.
    pub(crate) fn index(&self) -> usize {
        self.index
    }

    /// The entry's name.
    #[must_use]
    pub fn name(&self) -> &'a N {
        &self.data().name
    }

    /// What the name is: a positioned entry with its triple, or the
    /// distinguished child.
    #[must_use]
    pub fn view(&self) -> NameView<'a, N::Parts> {
        self.data().name.view()
    }

    /// The `(ordinal, key, parts)` this entry's name is isomorphic to, or
    /// `None` for the distinguished child, which has none of the three.
    #[must_use]
    pub fn triple(&self) -> Option<Triple<'a, N::Parts>> {
        match self.view() {
            NameView::Positioned(triple) => Some(triple),
            NameView::Distinguished => None,
        }
    }

    /// Which of the three kinds of thing this entry is.
    #[must_use]
    pub fn species(&self) -> Species {
        self.data().name.species()
    }

    /// Where the entry sits among its siblings, or `None` for the
    /// distinguished child, which carries no ordinal and never participates in
    /// ordering.
    #[must_use]
    pub fn ordinal(&self) -> Option<Ordinal> {
        self.triple().map(|t| t.ordinal)
    }

    /// Which entry this is, or `None` for the distinguished child.
    #[must_use]
    pub fn key(&self) -> Option<Key> {
        self.triple().map(|t| t.key)
    }

    /// Distance from the tree root: a child of the root has depth 1.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.data().depth
    }

    /// The level this entry sits in.
    #[must_use]
    pub fn container(&self) -> Container<'a, N> {
        Container {
            snapshot: self.snapshot,
            of: self.data().parent,
        }
    }

    /// This entry's own level, or `None` when it is not a node.
    #[must_use]
    pub fn contents(&self) -> Option<Container<'a, N>> {
        if self.data().contents.is_some() {
            Some(Container {
                snapshot: self.snapshot,
                of: Some(self.index),
            })
        } else {
            None
        }
    }

    /// **`ancestors`**: the entry's containing nodes, root-first.
    ///
    /// The chain ends at the tree root, which is a node and *not* an entry — so
    /// its element type is [`Container`] and not [`Entry`]. There is no
    /// spelling of this that returns entries: the last element has no ordinal,
    /// no key and no parts, and its name is never parsed.
    #[must_use]
    pub fn ancestors(&self) -> Vec<Container<'a, N>> {
        let mut chain = Vec::with_capacity(self.depth());
        let mut at = self.data().parent;
        loop {
            chain.push(Container {
                snapshot: self.snapshot,
                of: at,
            });
            match at {
                None => break,
                Some(index) => at = self.snapshot.entries[index].parent,
            }
        }
        chain.reverse();
        chain
    }

    /// **`distinguished_chain`**: the distinguished child of each of this
    /// entry's ancestors, root-first, skipping levels that have none.
    ///
    /// A node's distinguished child is its own content, so this is every piece
    /// of content on the path down to the entry — which is what a consumer
    /// assembling context from a tree wants, and why it is an operation rather
    /// than something a caller composes out of [`Entry::ancestors`].
    #[must_use]
    pub fn distinguished_chain(&self) -> Vec<Entry<'a, N>> {
        self.ancestors()
            .into_iter()
            .filter_map(|container| container.distinguished())
            .collect()
    }
}

/// A level of the tree: the root, or a node.
///
/// The root is a node that is not an entry — it is the directory the consumer
/// handed the library, it has no ordinal, no key and no parts, and its own name
/// is never parsed. That is why this type exists at all: a chain of containing
/// levels cannot be a chain of entries.
pub struct Container<'a, N> {
    snapshot: &'a Snapshot<N>,
    /// The node whose level this is; `None` for the tree root.
    of: Option<usize>,
}

impl<N> Clone for Container<'_, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N> Copy for Container<'_, N> {}

impl<N> PartialEq for Container<'_, N> {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self.snapshot, other.snapshot) && self.of == other.of
    }
}

impl<N> Eq for Container<'_, N> {}

impl<N: EntryName> core::fmt::Debug for Container<'_, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.entry() {
            None => f.write_str("Container(root)"),
            Some(entry) => f.debug_tuple("Container").field(&entry).finish(),
        }
    }
}

impl<'a, N: EntryName> Container<'a, N> {
    /// The node whose level this is, or `None` when it is the tree root.
    #[must_use]
    pub fn entry(&self) -> Option<Entry<'a, N>> {
        self.of.map(|index| Entry {
            snapshot: self.snapshot,
            index,
        })
    }

    /// Whether this is the tree root.
    #[must_use]
    pub fn is_root(&self) -> bool {
        self.of.is_none()
    }

    fn directory(&self) -> &'a Directory {
        match self.of {
            None => &self.snapshot.root,
            Some(index) => self.snapshot.entries[index]
                .contents
                .as_ref()
                .expect("a container is only ever built for the root or a node"),
        }
    }

    /// Every child of this level, in walk order: the distinguished child first,
    /// then the positioned children by ordinal.
    pub fn children(&self) -> impl Iterator<Item = Entry<'a, N>> {
        let snapshot = self.snapshot;
        self.directory().children.iter().map(move |index| Entry {
            snapshot,
            index: *index,
        })
    }

    /// This level's positioned children, by ordinal — everything
    /// [`Container::children`] yields except the distinguished child.
    pub fn positioned(&self) -> impl Iterator<Item = Entry<'a, N>> {
        self.children()
            .filter(|entry| entry.species() != Species::Distinguished)
    }

    /// This level's distinguished child, if it has one.
    ///
    /// *At most one* is a theorem rather than something enforced here: a domain
    /// holding the obligation *`distinguished()` names the only entry of its
    /// species* cannot produce a second name of that species, and a directory
    /// cannot hold two entries of one name. A domain that broke it would put
    /// two in this level, and this answers with the first in walk order rather
    /// than hiding either.
    ///
    /// **`Option` and not [`Sought`], deliberately.** This is an accessor: a
    /// level either has a distinguished child or does not, and the absence is a
    /// fact about the level, exactly as [`Entry::key`]'s is a fact about the
    /// entry. `Sought` answers a *search* — a criterion the caller supplied and
    /// a set scanned for it — and no criterion crosses this call. That it is
    /// implemented over a walk is an implementation detail; if it were the test,
    /// every accessor here would be a search.
    #[must_use]
    pub fn distinguished(&self) -> Option<Entry<'a, N>> {
        self.children()
            .find(|entry| entry.species() == Species::Distinguished)
    }
}

impl<N: EntryName> Snapshot<N> {
    /// The tree root: a node that is not an entry.
    #[must_use]
    pub fn root(&self) -> Container<'_, N> {
        Container {
            snapshot: self,
            of: None,
        }
    }

    /// The entry at a position in this snapshot's arena.
    ///
    /// Crate-private for the reason [`Entry::index`] gives. It is a plain index
    /// into a `Vec` the snapshot owns and never shrinks, so the only way to
    /// hold an out-of-range one is to have taken it from a different snapshot.
    ///
    /// # Panics
    ///
    /// If the index names nothing here.
    pub(crate) fn at(&self, index: usize) -> Entry<'_, N> {
        assert!(
            index < self.entries.len(),
            "an index from another snapshot names nothing here"
        );
        Entry {
            snapshot: self,
            index,
        }
    }

    /// The level a plan's [`Level`] names, or `None` for one the plan itself
    /// creates — which the snapshot, read before anything ran, cannot know
    /// about.
    pub(crate) fn level(&self, level: Level) -> Option<Container<'_, N>> {
        match level {
            Level::Root => Some(self.root()),
            Level::Entry(index) => self.at(index).contents(),
            Level::Created(_) => None,
        }
    }

    /// **`walk`**: every entry in depth-first pre-order.
    ///
    /// Within a level the distinguished child comes first, then the children by
    /// ordinal; nodes are descended in place, so a node at an earlier ordinal is
    /// fully explored before a later sibling. The order is this module's own —
    /// see its header for what is checked and what is not.
    pub fn walk(&self) -> Walk<'_, N> {
        let mut pending: Vec<usize> = self.root.children.clone();
        pending.reverse();
        Walk {
            snapshot: self,
            pending,
        }
    }

    /// **`seek`**: the first entry in walk order satisfying a predicate.
    ///
    /// Short-circuits. This is also how a consumer asks every question about
    /// its own attributes — *which entry is next*, *which is a draft* — without
    /// the library ever learning what was asked. There is deliberately no
    /// lookup by label: the trait names no label type, so a `by_label` would
    /// have nothing to take as an argument.
    ///
    /// Named `seek` and not `find` because the answer is a [`Sought`] and not an
    /// `Option`: `find` is [`Iterator`]'s word, it is right there on
    /// [`Walk`], and two operations one character apart answering in two
    /// vocabularies is exactly the confusion one word for one concept exists to
    /// prevent. `Walk::find` stays — it is the iterator's, and the iterator's
    /// vocabulary is `Option`'s.
    pub fn seek(&self, mut predicate: impl FnMut(&Entry<'_, N>) -> bool) -> Sought<Entry<'_, N>> {
        self.walk().find(|entry| predicate(entry)).into()
    }

    /// **`by_key`**: the entry with a given key, or nothing.
    ///
    /// Keys are unique in any tree the library built. In one it did not — a
    /// hand edit can repeat a key, and nothing checks for it — this returns the
    /// first in walk order, and the caller has a tree to repair. That tie-break
    /// is the one reading behaviour no model checks; `operations.qnt` picks the
    /// least internal id instead, and says so in its handoff block.
    ///
    /// [`Sought::Nothing`] is not a refusal: no key was asked to change, and a
    /// tree holding no such key is not a damaged tree.
    pub fn by_key(&self, key: Key) -> Sought<Entry<'_, N>> {
        self.seek(|entry| entry.key() == Some(key))
    }

    /// How many entries the tree holds, distinguished children included.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the tree holds no entries at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Every entry of a snapshot, in walk order. See [`Snapshot::walk`].
pub struct Walk<'a, N> {
    snapshot: &'a Snapshot<N>,
    /// Entries still to visit, in reverse order so the next is the last.
    pending: Vec<usize>,
}

impl<'a, N: EntryName> Iterator for Walk<'a, N> {
    type Item = Entry<'a, N>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.pending.pop()?;
        let entry = Entry {
            snapshot: self.snapshot,
            index,
        };
        if let Some(contents) = self.snapshot.entries[index].contents.as_ref() {
            self.pending.extend(contents.children.iter().rev().copied());
        }
        Some(entry)
    }
}

#[cfg(test)]
mod tests;
