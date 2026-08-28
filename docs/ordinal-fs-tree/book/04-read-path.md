# Read path
<!-- book-page id="read-path" slice="read-path-k14" order="4" -->
[Previous: Reference domain](03-reference-domain.md) | [Contents](README.md) | [Next: Mutation algebra](05-mutation-algebra.md)

A read turns one directory tree into an immutable set of names under a shared
advisory lock. The filesystem layer observes directory entries without following
them, the consumer classifies each observed name, and the snapshot layer freezes
the accepted names into deterministic level and walk order. File contents are not
read. The snapshot contains names, hierarchy, depth, and ordered child lists.

<!-- fragment «read-snapshot-source» owner="read-path-k14" source="crates/ordinal-fs-tree/src/snapshot.rs" lines="1-650" parent="source-snapshot" -->
<!-- insert «snapshot-storage» -->
<!-- insert «snapshot-builder» -->
<!-- insert «snapshot-entry-views» -->
<!-- insert «snapshot-containers» -->
<!-- insert «snapshot-queries» -->
<!-- /fragment -->

<!-- fragment «read-filesystem-source» owner="read-path-k14" source="crates/ordinal-fs-tree/src/fs/read.rs" lines="1-179" parent="source-filesystem-read" -->
<!-- insert «read-tree-discovery» -->
<!-- insert «read-directory-listing» -->
<!-- insert «read-lock-location» -->
<!-- /fragment -->

<a id="worked-read"></a>
## One complete read

The read-path variant from the previous page is the worked input. Its handed-in
root is spelled `s`, matching the orientation example:

```text
s/
├── OVERVIEW.md
├── 01-published-orientation-i1.md
├── 02-linear-algebra-i2/
│   ├── OVERVIEW.md
│   ├── 01-published-vectors-i5.md
│   └── 02-draft-matrices-i6.md
└── 03-draft-assessment-i9.md
```

A call to `fs::read::<SyllabusName>(Path::new("s"))` performs this
sequence:

1. `containing_directory` resolves `s/..` through the kernel and
   identifies the directory whose lock covers this tree.
2. `acquire` takes a shared advisory lock on that directory and calls
   `read::snapshot` while the lock is held.
3. `snapshot` lists the root. Each `DirEntry::file_type` result becomes
   `Found::File`, `Found::Dir`, or `Found::Other` without following a symbolic
   link. Each UTF-8 filename and its `Found` value go to `SyllabusName::parse`.
4. Every accepted name is added to the snapshot builder. Accepted nodes add
   their directory to an explicit worklist. Leaves and distinguished children
   do not.
5. The worklist reaches `02-linear-algebra-i2`, repeats the same listing and
   classification there, then `Builder::finish` sorts every level.
6. `ReadGuard` returns the caller-spelled root, the frozen snapshot, and the
   still-open lock descriptor.

Calling `guard.walk().map(|entry| (entry.depth(), entry.name().to_string()))`
produces the following public query result. Indentation is derived from `depth`;
each displayed value is the entry's local name, not a reconstructed path:

```text
OVERVIEW.md
01-published-orientation-i1.md
02-linear-algebra-i2
  OVERVIEW.md
  01-published-vectors-i5.md
  02-draft-matrices-i6.md
03-draft-assessment-i9.md
```

The distinguished child leads each level. Positioned children follow by
ordinal. Entering a node happens before visiting its later siblings, so the
module's complete subtree precedes assessment.

<a id="filesystem-discovery"></a>
## Filesystem discovery and classification

The filesystem reader owns this first transition. It takes the caller's root
path and produces a finished `Snapshot` by classifying every reachable,
unfollowed directory entry through the consumer seam. Its whole-tree invariant
is that foreign names are absent while any malformed, reserved, non-UTF-8, or
non-component owned name halts construction; here it processes the complete
`s` example before a guard can return.

<!-- fragment «read-tree-discovery» owner="read-path-k14" source="crates/ordinal-fs-tree/src/fs/read.rs" lines="1-81" parent="read-filesystem-source" -->
````rust
//! Turning a directory tree into a [`Snapshot`].
//!
//! This is the whole of the parse trichotomy in practice: every name in every
//! directory a walk reaches is handed to the consumer's
//! [`parse`](EntryName::parse) together with **what the listing found under it,
//! unfollowed**, and the three outcomes are the three things that can happen to
//! a name. `Entry` joins the tree. `Foreign` is skipped — and skipped
//! recursively when it is a directory, which is sound precisely because the
//! consumer said the name was not its own. `Malformed` and `Reserved` halt.
//!
//! Snapshot scope is the **whole tree**, so a halt anywhere halts everything.

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use crate::snapshot::{Builder, Snapshot};
use crate::{EntryName, Error, Found, Verdict};

/// Read a whole tree, or halt.
pub(super) fn snapshot<N: EntryName>(root: &Path) -> Result<Snapshot<N>, Error<N>> {
    let mut builder = Builder::new();
    // An explicit worklist rather than recursion: the depth of a tree on disk
    // is the user's to choose, and a stack overflow is not a refusal any
    // consumer can handle.
    let mut pending = vec![(root.to_path_buf(), builder.root())];
    while let Some((directory, place)) = pending.pop() {
        let mut descend = Vec::new();
        for (name, found) in listing(&directory)? {
            let path = directory.join(&name);
            let Some(name) = name.to_str() else {
                return Err(Error::NonUtf8Name { path });
            };
            match N::parse(name, found) {
                // Not this consumer's name, so it is not this consumer's
                // problem — and not this library's either.
                Verdict::Foreign => {}
                Verdict::Malformed(source) => return Err(Error::Malformed { path, source }),
                Verdict::Reserved(source) => return Err(Error::Reserved { path, source }),
                // A walk descends into recognised nodes and nothing else, and
                // `add` answers with a place exactly for those. A distinguished
                // child is a node's own content rather than a level of the
                // tree, and it is a regular file — a domain holding the
                // obligations cannot produce one that is not.
                Verdict::Entry(parsed) => {
                    // The seventh obligation, enforced at the first of the two
                    // boundaries where a name becomes a path. A snapshot name is
                    // rendered by `entry_path` to reach the entry a move starts
                    // from, and by `level_path` to reach a node a plan writes
                    // into, so one that is not a filename addresses outside the
                    // tree the lock covers. The rendering costs one allocation
                    // per entry, alongside the two the listing already makes,
                    // and it buys the property that *every name in a snapshot is
                    // one path component* — which is what makes both of those
                    // functions safe without repeating the check.
                    let rendered = parsed.to_string();
                    if let Some(reason) = crate::name::not_one_component(&rendered) {
                        return Err(Error::NameIsNotOneComponent {
                            root: root.to_path_buf(),
                            rendered,
                            reason,
                        });
                    }
                    if let Some(below) = builder.add(place, parsed) {
                        descend.push((path, below));
                    }
                }
            }
        }
        // Sorted order is the order the *listing* was read in; pushing the
        // subdirectories in reverse makes the stack pop them in that order, so
        // which of two broken names halts the tree does not depend on where the
        // filesystem happened to put them.
        while let Some(child) = descend.pop() {
            pending.push(child);
        }
    }
    Ok(builder.finish())
}

````
<!-- /fragment -->

`snapshot` uses an explicit LIFO worklist rather than call-stack recursion.
`listing` returns one directory in filename order. Nodes accepted from that
listing are accumulated in the same order, then pushed in reverse, so they are
popped in deterministic order. This ordering decides which error is reported
first when separate subtrees are both broken; it does not establish snapshot
walk order. `Builder::finish` does that after discovery.

`Verdict::Foreign` contributes no entry and no directory to the worklist. A
foreign directory is therefore skipped with its entire subtree. The consumer
has positively disclaimed the name, so recursive skipping is part of that
verdict. `Malformed` and `Reserved` instead halt the whole read and retain the
consumer's error as recovery advice. A non-UTF-8 name also halts because
`EntryName::parse` accepts `&str`, leaving no consumer verdict that could
safely classify it.

The `listing` helper owns one directory observation. It turns `DirEntry` values
into a filename-sorted vector of `(OsString, Found)` pairs without following
links, establishing a total order and retaining the observed filesystem species
for the consumer's parser. On the example root it supplies the four direct
names and marks only `02-linear-algebra-i2` as a directory.

<!-- fragment «read-directory-listing» owner="read-path-k14" source="crates/ordinal-fs-tree/src/fs/read.rs" lines="82-124" parent="read-filesystem-source" -->
````rust
/// One directory's names and what is under each, sorted.
///
/// Sorted because the halt has to be deterministic: a tree carrying two names
/// the consumer cannot parse would otherwise report whichever one `read_dir`
/// reached first, so the recovery advice a consumer sees would depend on the
/// filesystem rather than on the tree.
fn listing<N: EntryName>(directory: &Path) -> Result<Vec<(OsString, Found)>, Error<N>> {
    let reading = fs::read_dir(directory).map_err(|source| Error::<N>::Io {
        path: directory.to_path_buf(),
        doing: "reading the directory",
        source,
    });
    let mut found = Vec::new();
    for entry in reading? {
        let entry = entry.map_err(|source| Error::<N>::Io {
            path: directory.to_path_buf(),
            doing: "reading the directory",
            source,
        })?;
        // `DirEntry::file_type` does not traverse a symbolic link — it is
        // `symlink_metadata`, not `metadata`. That is what makes a symbolic
        // link wearing an entry's name `Found::Other`, and therefore
        // `Malformed`, rather than whatever it points at.
        // <https://doc.rust-lang.org/std/fs/struct.DirEntry.html#method.file_type>
        let kind = entry.file_type().map_err(|source| Error::<N>::Io {
            path: entry.path(),
            doing: "inspecting",
            source,
        })?;
        let kind = if kind.is_file() {
            Found::File
        } else if kind.is_dir() {
            Found::Dir
        } else {
            Found::Other
        };
        found.push((entry.file_name(), kind));
    }
    // By name: two entries of one directory cannot share one, so this is total.
    found.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(found)
}

````
<!-- /fragment -->

`DirEntry::file_type` observes the directory entry itself. It does not follow
a symbolic link. A symlink wearing a syllabus filename is consequently
`Found::Other`; the reference parser reports the file/species contradiction as
malformed rather than reading the link's target. The raw listing is sorted by
`OsString` before UTF-8 conversion and parsing, making both accepted discovery
and the first reported failure independent of filesystem enumeration order.

<a id="snapshot-construction"></a>
## Snapshot construction and level order

The snapshot module owns the pure in-memory representation. It receives parsed
names from the filesystem reader and stores them as an arena plus distinct root
level, establishing that every public view refers to stable snapshot-owned data
while the root remains a level that is not an entry. This structure is where
the accepted syllabus names acquire parents and depths without carrying paths.

<!-- fragment «snapshot-storage» owner="read-path-k14" source="crates/ordinal-fs-tree/src/snapshot.rs" lines="1-104" parent="read-snapshot-source" -->
````rust
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
use crate::{EntryName, EntryNameExt, Key, NameView, Ordinal, Species, Triple};

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

````
<!-- /fragment -->

`EntryData` is the arena record: a name, an optional parent arena index, an
optional child directory for nodes, and depth from the root. `Snapshot` owns
the arena plus a separate root `Directory`. The root is a level but not an
entry, so it has no `EntryData`, name, ordinal, key, or parts. Each `Directory`
stores arena indices rather than nested values. Moving or borrowing a view
therefore does not move the tree.

A `Place` is a construction-only address for one level. The root place is
universal; a node place carries the originating builder identity as well as
its arena index. This prevents a place from one builder from silently attaching
a child to the same numeric index in another.

`Builder` owns the construction transition. Each `(Place, parsed name)` input
adds one arena record and returns a new place only for a node; `finish` consumes
that provisional arena and outputs a snapshot with every level in total walk
order. Builder identity prevents cross-builder attachment, and the example uses
the place returned for `02-linear-algebra-i2` to attach its three children.

<!-- fragment «snapshot-builder» owner="read-path-k14" source="crates/ordinal-fs-tree/src/snapshot.rs" lines="105-267" parent="read-snapshot-source" -->
````rust
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
````
<!-- /fragment -->

`Builder::add` derives node status from the parsed name, records parent and
depth, appends the new arena index to its level, and returns a child place only
for a node. Arrival order is intentionally provisional. `finish` extracts
every level, sorts it, restores it, and consumes the builder into an immutable
`Snapshot`.

Within a level, sorting places distinguished names first. Positioned names are
ordered by ordinal, then key, then rendered name. The final two comparisons
make the order total even for a hand-edited tree with duplicate ordinals or
keys. They make `walk`, `find`, and the documented duplicate-key behavior
deterministic; they do not validate or repair those duplicates.

<a id="borrowed-views"></a>
## Entries and levels are borrowed views

`Entry` owns the borrowed entry view. A snapshot reference plus arena index
becomes name, triple, species, depth, containing-level, ancestor, and
distinguished-chain readings without cloning stored names. Its identity is the
same index in the same snapshot, and the matrices entry demonstrates both the
root-first ancestor chain and the two-overview distinguished chain.

<!-- fragment «snapshot-entry-views» owner="read-path-k14" source="crates/ordinal-fs-tree/src/snapshot.rs" lines="268-438" parent="read-snapshot-source" -->
````rust

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

````
<!-- /fragment -->

`Entry<'a, N>` is a copyable pair of a snapshot reference and an arena index.
It exposes the name and its generic readings without cloning either. `view`,
`triple`, `species`, `ordinal`, and `key` all derive from the stored name.
`depth` is stored during construction. Equality means the same arena position
in the same snapshot, not equal names or equal keys.

`container` returns the level holding an entry. `contents` returns the level
held by a node and returns `None` for a leaf or distinguished child.
`ancestors` starts at the containing level, follows parent indices to the root,
then reverses the collected chain. Its result is root-first and uses
`Container`, because the root cannot be represented as an `Entry`.

For `02-draft-matrices-i6.md`, `ancestors` yields the root container followed
by the `02-linear-algebra-i2` container. `distinguished_chain` maps that chain
to the distinguished child present at each level, yielding the root
`OVERVIEW.md` followed by the module `OVERVIEW.md`. Levels without one are
skipped rather than terminating the chain.

`Container` owns the borrowed level view. It turns either the root marker or a
node arena index into ordered child, positioned-child, and distinguished-child
iterators, preserving every stored name even when a consumer violates the
one-distinguished-name obligation. In the example, the module container yields
its overview before vectors and matrices while the root uses no entry index.

<!-- fragment «snapshot-containers» owner="read-path-k14" source="crates/ordinal-fs-tree/src/snapshot.rs" lines="439-532" parent="read-snapshot-source" -->
````rust
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
    #[must_use]
    pub fn distinguished(&self) -> Option<Entry<'a, N>> {
        self.children()
            .find(|entry| entry.species() == Species::Distinguished)
    }
}
````
<!-- /fragment -->

`Container<'a, N>` uses `of: None` for the root and `Some(index)` for a node's
level. `children` follows the already-sorted index list. `positioned` removes
distinguished children, while `distinguished` returns the first distinguished
entry. Under a conforming `EntryName`, at most one exists; retaining all names
in the level keeps a broken domain observable instead of silently discarding
the extra one.

<a id="queries-and-walk"></a>
## Queries and walk order

`Snapshot` and `Walk` own public traversal and lookup. They turn the immutable
root ordering, predicates, and keys into depth-first entries or the first
matching borrowed view, preserving the same total level order for every query.
The example walk produces the indented name sequence above, while draft and key
predicates both select matrices at their documented first match.

<!-- fragment «snapshot-queries» owner="read-path-k14" source="crates/ordinal-fs-tree/src/snapshot.rs" lines="533-650" parent="read-snapshot-source" -->
````rust

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

    /// **`find`**: the first entry in walk order satisfying a predicate.
    ///
    /// Short-circuits. This is also how a consumer asks every question about
    /// its own attributes — *which entry is next*, *which is a draft* — without
    /// the library ever learning what was asked. There is deliberately no
    /// lookup by label: the trait names no label type, so a `by_label` would
    /// have nothing to take as an argument.
    pub fn find(&self, mut predicate: impl FnMut(&Entry<'_, N>) -> bool) -> Option<Entry<'_, N>> {
        self.walk().find(|entry| predicate(entry))
    }

    /// **`by_key`**: the entry with a given key, or nothing.
    ///
    /// Keys are unique in any tree the library built. In one it did not — a
    /// hand edit can repeat a key, and nothing checks for it — this returns the
    /// first in walk order, and the caller has a tree to repair. That tie-break
    /// is the one reading behaviour no model checks; `operations.qnt` picks the
    /// least internal id instead, and says so in its handoff block.
    #[must_use]
    pub fn by_key(&self, key: Key) -> Option<Entry<'_, N>> {
        self.find(|entry| entry.key() == Some(key))
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
````
<!-- /fragment -->

`Snapshot::walk` initializes a stack from the root's ordered children in
reverse. Each iterator step pops the next entry and, for a node, pushes its
children in reverse. The observable result is depth-first pre-order while
preserving the sorted order within every level. Distinguished children are
regular files, so they never add descendants.

`find` short-circuits at the first entry in walk order satisfying a consumer
predicate. This is the generic attribute query: for the worked tree, a
predicate over `Parts::Lesson { status: Draft, .. }` first finds matrices.
`by_key(Key::new(6))` is the specialized predicate lookup and also returns
matrices. If a hand edit duplicates a key, it returns the first occurrence in
walk order; the read API reports rather than repairs the tree's current names.
`len` counts distinguished children as entries, and `is_empty` means the root
contains no accepted entry at all.

<a id="read-guard"></a>
## Shared lock and snapshot lifetime

The public filesystem module owns guarded reads. It turns a caller-spelled root
into `ReadGuard<N>` only after taking a shared lock and constructing the whole
snapshot under it, establishing that the lock token and immutable names cross
the API boundary together. The example's `s` spelling therefore remains
available beside the exact snapshot used by its public walk.

<!-- fragment «filesystem-read-opening» owner="read-path-k14" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="1-86" parent="source-filesystem-module" -->
````rust
//! The filesystem, and the only module in this crate that may name it.
//!
//! Everything else here is the algebra: pure, testable without a directory, and
//! modellable without an abstraction of one. That boundary is what makes a later
//! split of this crate into separately-modellable units mechanical rather than
//! archaeological, and `tests/algebra_has_no_filesystem.rs` is what holds it —
//! inside one crate, a seam the compiler does not enforce is a seam nothing
//! measures.
//!
//! # What a guard is
//!
//! One advisory lock and one [`Snapshot`], taken together. Every operation the
//! library performs begins by reading the tree's names under a lock, so a guard
//! is that pair and holding one is holding both: the lock is released when the
//! guard is dropped, and there is no unlock call to forget.
//!
//! ```no_run
//! # use std::path::Path;
//! # use ordinal_fs_tree::reference::SyllabusName;
//! let tree = ordinal_fs_tree::fs::read::<SyllabusName>(Path::new("syllabus"))?;
//! for entry in tree.walk() {
//!     println!("{:indent$}{}", "", entry.name(), indent = (entry.depth() - 1) * 2);
//! }
//! # Ok::<(), ordinal_fs_tree::Error<SyllabusName>>(())
//! ```
//!
//! # Locking is invisible in the interface
//!
//! There is no lock type in this module's public surface, no *try* variant and
//! no timeout — an API offering any of those would be an API that mentions
//! locking, which the architecture document says consumers never do. What that
//! costs is stated rather than hidden: [`read`] and [`write`] block until the
//! tree is free.
//!
//! # Paths come back the way they went in
//!
//! Nothing here canonicalises. The lock follows inode identity through the
//! descriptor, so it does not need a canonical path to be correct, and every
//! path a guard reports is built from the caller's own spelling of the root. On
//! macOS `/var` and `/private/var` name the same inode, so canonicalising would
//! make the mere presence of a lock observably rewrite every path the reading
//! operations return.
//!
//! The lock still has to name the **tree** rather than a spelling of it, or two
//! accepted spellings of one root would not exclude each other. That is bought
//! by asking the kernel instead of the string: the directory to lock is
//! `<root>/..`, which resolves through `..` and through a symbolic link naming
//! the root and lands on one inode however the caller wrote the path. See
//! `read::containing_directory`, which carries the counterexample that made a
//! lexical parent insufficient.

use std::fs::File;
use std::path::{Path, PathBuf};

use crate::ops::{self, NewEntry, Target};
use crate::plan::Decision;
use crate::report::Report;
use crate::snapshot::Snapshot;
use crate::{EntryName, Error, Key, Ordinal};

mod apply;
mod lock;
mod read;

/// Read a tree under a **shared** lock: other readers may hold it at the same
/// time, no writer may.
///
/// Blocks until the tree is free. Halts — rather than skipping anything — on a
/// name the consumer recognises and cannot parse, wherever in the tree it sits.
///
/// # Errors
///
/// [`Error::Malformed`] or [`Error::Reserved`] for a name the consumer owns and
/// refuses, carrying the consumer's own recovery advice; [`Error::NonUtf8Name`]
/// for a filename that cannot be classified at all; [`Error::Io`] for a
/// filesystem refusal; [`Error::NoContainingDirectory`] for a root with nothing
/// to lock.
pub fn read<N: EntryName>(root: &Path) -> Result<ReadGuard<N>, Error<N>> {
    let (guard, snapshot) = acquire(root, lock::Mode::Shared)?;
    Ok(ReadGuard {
        _guard: guard,
        root: root.to_path_buf(),
        snapshot,
    })
}

````
<!-- /fragment -->

The public `read` surface blocks until it obtains a shared advisory lock.
Cooperating readers may coexist and a cooperating writer waits. The lock is on
the directory containing the root, not on the root itself, so operations that
create, replace, or remove the root use the same lock identity. Advisory locks
do not prevent an uncooperating process from changing files; the guarantee is
the synchronization contract among processes using this library.

`acquire` owns the lock-and-capture transition. Given a root and lock mode, it
returns the open lock descriptor and finished snapshot, with the invariant that
snapshot discovery begins only after the descriptor is held. `fs::read` selects
shared mode for the example and stores both outputs in its `ReadGuard`.

<!-- fragment «filesystem-read-acquire-and-guard» owner="read-path-k14" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="106-131" parent="source-filesystem-module" -->
````rust
fn acquire<N: EntryName>(root: &Path, mode: lock::Mode) -> Result<(File, Snapshot<N>), Error<N>> {
    let directory = read::containing_directory::<N>(root)?;
    let guard = lock::take(&directory, mode).map_err(|source| Error::Io {
        path: directory.clone(),
        doing: "locking the directory containing the tree",
        source,
    })?;
    // Under the lock, and only under it: a snapshot read outside one could be
    // stale before the caller saw it.
    let snapshot = read::snapshot(root)?;
    Ok((guard, snapshot))
}

/// A tree read under a shared lock.
///
/// Derefs to its [`Snapshot`], so the reading operations are called on the guard
/// itself: `tree.walk()`, `tree.by_key(k)`.
pub struct ReadGuard<N> {
    // The lock *is* this descriptor: `flock` is released when the last handle to
    // the open file description closes, so the field never has to be read and
    // dropping the guard is the whole of releasing it.
    _guard: File,
    root: PathBuf,
    snapshot: Snapshot<N>,
}

````
<!-- /fragment -->

`acquire` takes the descriptor before calling `read::snapshot`. The descriptor
and snapshot are returned together, and `ReadGuard` stores both. `_guard` is
never read: the open descriptor is the lock token, and dropping the guard
closes it. The caller's root spelling is copied separately rather than
canonicalized.

`ReadGuard` owns the explicit read accessors. It turns a shared borrow of the
guard into the caller-spelled root or the exact immutable snapshot captured
under its lock, without copying either, so every returned snapshot borrow stays
bounded by the guard. The worked walk starts from that `snapshot` result.

<!-- fragment «filesystem-read-guard-api» owner="read-path-k14" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="155-168" parent="source-filesystem-module" -->
````rust
impl<N: EntryName> ReadGuard<N> {
    /// The tree root, in the caller's own spelling.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The tree's names, as they were when the lock was taken.
    #[must_use]
    pub fn snapshot(&self) -> &Snapshot<N> {
        &self.snapshot
    }
}

````
<!-- /fragment -->

The `Deref` implementation owns the ergonomic forwarding step. It turns
`&ReadGuard<N>` into `&Snapshot<N>` and always returns the same stored snapshot,
so direct calls cannot bypass the captured names or their borrow lifetime. This
is why the example can spell its public query as `guard.walk()`.

<!-- fragment «filesystem-read-deref» owner="read-path-k14" source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="379-386" parent="source-filesystem-module" -->
````rust
impl<N: EntryName> core::ops::Deref for ReadGuard<N> {
    type Target = Snapshot<N>;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

````
<!-- /fragment -->

`ReadGuard::root` returns the caller-spelled path and `snapshot` returns the
immutable capture made after locking. `Deref<Target = Snapshot<N>>` makes
`guard.walk()`, `guard.find(...)`, and `guard.by_key(...)` ordinary snapshot
calls. Every returned `Entry` or `Container` borrows that snapshot, so Rust
prevents a view from outliving the guard that owns it. Using such a view keeps
its borrow of the guard valid and prevents the guard from being dropped first;
the shared lock remains held for that interval.

`containing_directory` and `identity` own lock-location resolution. They turn
the caller's root spelling into a containing-directory path while comparing
device/inode pairs, establishing one lock identity for direct, roundabout, and
symlinked spellings and refusing a root with no distinct parent. For the `s`
example, this produces `s/..` without canonicalizing the root later returned by
the guard.

<!-- fragment «read-lock-location» owner="read-path-k14" source="crates/ordinal-fs-tree/src/fs/read.rs" lines="125-179" parent="read-filesystem-source" -->
````rust
/// The directory whose lock covers this tree: the one **containing** the root,
/// as the *kernel* resolves it.
///
/// Asked for as `<root>/..`, and that spelling is the whole of the fix
/// `reading-k19` found necessary. A lexical `Path::parent` chops a component
/// off a string, but `..` and symbolic links are resolved by the kernel, one
/// component at a time, against the directory actually reached — so the
/// accepted spelling `syllabus/02-linear-algebra-i2/..` reads the tree
/// `syllabus` while its lexical parent is `syllabus/02-linear-algebra-i2`, a
/// different directory from the one the direct spelling locks. Two spellings of
/// one tree would then take two locks, and the premise that a snapshot is read
/// under the lock covering it would be false.
///
/// Handing the kernel `<root>/..` makes it resolve the root — following a final
/// symbolic link, because a component in the middle of a path is followed — and
/// then step to that directory's real parent. Every spelling of one tree
/// therefore reaches one inode, and nothing here canonicalises: the path is
/// still built from the caller's own spelling, so what a refusal reports is
/// still what went in.
pub(super) fn containing_directory<N: EntryName>(root: &Path) -> Result<PathBuf, Error<N>> {
    // The spellings with nothing to open at all — a filesystem root, and the
    // empty path. Refused lexically because there is no directory to ask about.
    if root.parent().is_none() {
        return Err(Error::NoContainingDirectory {
            root: root.to_path_buf(),
        });
    }
    let directory = root.join("..");
    // And the spellings that *do* open and land back on the root: `/..` is `/`,
    // and so is any symbolic link to it. The identity is the filesystem's own —
    // device and inode — because that is the identity `flock` attaches to, and
    // a lexical rule is exactly what was wrong before.
    if identity(root, "reading the tree root")?
        == identity(&directory, "reading the directory containing the tree")?
    {
        return Err(Error::NoContainingDirectory {
            root: root.to_path_buf(),
        });
    }
    Ok(directory)
}

/// What the filesystem calls this path: the pair `flock` attaches to.
///
/// `metadata` follows symbolic links, deliberately — the question is which
/// directory the caller's spelling *names*, not what its last component is
/// stored as.
fn identity<N: EntryName>(path: &Path, doing: &'static str) -> Result<(u64, u64), Error<N>> {
    let metadata = fs::metadata(path).map_err(|source| Error::<N>::Io {
        path: path.to_path_buf(),
        doing,
        source,
    })?;
    Ok((metadata.dev(), metadata.ino()))
}
````
<!-- /fragment -->

`containing_directory` rejects a filesystem root because no containing
directory exists to lock. It resolves `<root>/..` through the filesystem and
compares device/inode identity, which unifies direct, roundabout, and symlinked
spellings of one tree without changing the path later reported to the caller.

<a id="read-errors"></a>
## Read failures and recovery

A read does not run the mutation algebra, produce a `Decision`, or return an
algebraic `Refusal`. `Verdict::Foreign` is also not a refusal: it is a
successful classification that deliberately excludes one directory entry.
The remaining boundary outcomes are `Error` variants:

- `Malformed` identifies an owned name that must be repaired according to the
  consumer error before any read of the tree can succeed.
- `Reserved` identifies an owned non-entry such as `PUBLISHING`; the consumer
  advice determines how to recover or resume the interrupted protocol.
- `NonUtf8Name` cannot carry consumer advice because parsing never ran. Rename
  or remove the offending filesystem entry before retrying.
- `NameIsNotOneComponent` reports a broken consumer implementation. Correct
  its rendering contract; changing unrelated tree names cannot make it safe.
- `NoContainingDirectory` means the selected root cannot participate in the
  lock protocol. Select a tree with a containing directory.
- `Io` records the caller-spelled path, the operation being attempted, and the
  filesystem error. Recovery follows that underlying failure: permissions,
  availability, or the changed filesystem object must be addressed.

All of these fail before a `ReadGuard` is returned. Any `Builder` or lock
descriptor already created is dropped during error propagation, and no
filesystem mutation has been attempted. `NoContainingDirectory`, identity
failure, and lock-acquisition failure can occur before either resource exists.

[Previous: Reference domain](03-reference-domain.md) | [Contents](README.md) | [Next: Mutation algebra](05-mutation-algebra.md)
