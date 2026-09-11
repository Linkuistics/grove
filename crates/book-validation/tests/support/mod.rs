// Every test binary compiles this module and uses a different subset of it, so
// an item unused by one binary is not an unused item.
#![allow(dead_code)]

use std::{collections::BTreeMap, fs, path::PathBuf};

use book_validation::{BookSnapshot, Manifest, Scope};

/// The fixture book's corpus exceptions, which are the relocated book's own.
///
/// The fixture cannot omit them. Its derived corpus is produced by walking the
/// real `crates/ordinal-fs-tree/`, so a fixture that declared the bare rule
/// would derive the four inline test modules and the test-support module and
/// then report five `F006`s against itself — which is the corpus control
/// working, on a manifest that lied.
const CORPUS_ADD: &[(&str, &str)] = &[(
    "crates/ordinal-fs-tree/bin/syllabus.rs",
    "production-outside-src",
)];

const CORPUS_EXCLUDE: &[(&str, &str)] = &[
    ("crates/ordinal-fs-tree/src/fixtures.rs", "test-support"),
    (
        "crates/ordinal-fs-tree/src/fs/apply/tests.rs",
        "inline-test-module",
    ),
    (
        "crates/ordinal-fs-tree/src/ops/tests.rs",
        "inline-test-module",
    ),
    (
        "crates/ordinal-fs-tree/src/plan/tests.rs",
        "inline-test-module",
    ),
    (
        "crates/ordinal-fs-tree/src/snapshot/tests.rs",
        "inline-test-module",
    ),
];

/// The repository root, from this crate's manifest directory.
pub fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The fixture book's repository-relative directory.
pub const BOOK_ROOT: &str = "docs/walkthroughs/ordinal-fs-tree";

#[derive(Clone, Copy)]
struct RootSpec {
    id: &'static str,
    path: &'static str,
    lines: usize,
    blocks: &'static [BlockSpec],
}

#[derive(Clone, Copy)]
struct BlockSpec {
    id: &'static str,
    owner: &'static str,
    first: usize,
    last: usize,
}

const ROOTS: &[RootSpec] = &[
    RootSpec {
        id: "source-crate-manifest",
        path: "crates/ordinal-fs-tree/Cargo.toml",
        lines: 112,
        blocks: &[
            BlockSpec {
                id: "manifest-package-and-library-dependency",
                owner: "orientation-k11",
                first: 1,
                last: 42,
            },
            BlockSpec {
                id: "manifest-cli-feature",
                owner: "syllabus-cli-k17",
                first: 43,
                last: 45,
            },
            BlockSpec {
                id: "manifest-library-cli-boundary",
                owner: "orientation-k11",
                first: 46,
                last: 61,
            },
            BlockSpec {
                id: "manifest-cli-binary",
                owner: "syllabus-cli-k17",
                first: 62,
                last: 65,
            },
            BlockSpec {
                id: "manifest-development-and-release",
                owner: "orientation-k11",
                first: 66,
                last: 112,
            },
        ],
    },
    RootSpec {
        id: "source-syllabus-cli",
        path: "crates/ordinal-fs-tree/bin/syllabus.rs",
        lines: 1741,
        blocks: &[BlockSpec {
            id: "syllabus-cli-source",
            owner: "syllabus-cli-k17",
            first: 1,
            last: 1741,
        }],
    },
    RootSpec {
        id: "source-library",
        path: "crates/ordinal-fs-tree/src/lib.rs",
        lines: 104,
        blocks: &[BlockSpec {
            id: "library-crate-surface",
            owner: "orientation-k11",
            first: 1,
            last: 104,
        }],
    },
    RootSpec {
        id: "source-conformance",
        path: "crates/ordinal-fs-tree/src/conformance.rs",
        lines: 629,
        blocks: &[BlockSpec {
            id: "reference-conformance-source",
            owner: "reference-domain-k13",
            first: 1,
            last: 629,
        }],
    },
    RootSpec {
        id: "source-error",
        path: "crates/ordinal-fs-tree/src/error.rs",
        lines: 510,
        blocks: &[BlockSpec {
            id: "filesystem-error-source",
            owner: "filesystem-interpreter-k16",
            first: 1,
            last: 510,
        }],
    },
    RootSpec {
        id: "source-name",
        path: "crates/ordinal-fs-tree/src/name.rs",
        lines: 706,
        blocks: &[BlockSpec {
            id: "name-seam-source",
            owner: "name-seam-k12",
            first: 1,
            last: 706,
        }],
    },
    RootSpec {
        id: "source-operations",
        path: "crates/ordinal-fs-tree/src/ops.rs",
        lines: 617,
        blocks: &[BlockSpec {
            id: "mutation-operations-source",
            owner: "mutation-algebra-k15",
            first: 1,
            last: 617,
        }],
    },
    RootSpec {
        id: "source-plan",
        path: "crates/ordinal-fs-tree/src/plan.rs",
        lines: 562,
        blocks: &[BlockSpec {
            id: "mutation-plan-source",
            owner: "mutation-algebra-k15",
            first: 1,
            last: 562,
        }],
    },
    RootSpec {
        id: "source-reference",
        path: "crates/ordinal-fs-tree/src/reference.rs",
        lines: 555,
        blocks: &[BlockSpec {
            id: "reference-domain-source",
            owner: "reference-domain-k13",
            first: 1,
            last: 555,
        }],
    },
    RootSpec {
        id: "source-report",
        path: "crates/ordinal-fs-tree/src/report.rs",
        lines: 186,
        blocks: &[BlockSpec {
            id: "mutation-report-source",
            owner: "mutation-algebra-k15",
            first: 1,
            last: 186,
        }],
    },
    RootSpec {
        id: "source-snapshot",
        path: "crates/ordinal-fs-tree/src/snapshot.rs",
        lines: 668,
        blocks: &[BlockSpec {
            id: "read-snapshot-source",
            owner: "read-path-k14",
            first: 1,
            last: 668,
        }],
    },
    RootSpec {
        id: "source-sought",
        path: "crates/ordinal-fs-tree/src/sought.rs",
        lines: 132,
        blocks: &[BlockSpec {
            id: "sought-object-answer",
            owner: "name-seam-k12",
            first: 1,
            last: 132,
        }],
    },
    RootSpec {
        id: "source-filesystem-module",
        path: "crates/ordinal-fs-tree/src/fs/mod.rs",
        lines: 826,
        blocks: &[
            BlockSpec {
                id: "filesystem-read-opening",
                owner: "read-path-k14",
                first: 1,
                last: 128,
            },
            BlockSpec {
                id: "filesystem-write-acquire",
                owner: "filesystem-interpreter-k16",
                first: 129,
                last: 155,
            },
            BlockSpec {
                id: "filesystem-read-acquire-and-guard",
                owner: "read-path-k14",
                first: 156,
                last: 202,
            },
            BlockSpec {
                id: "filesystem-writing-shape",
                owner: "filesystem-interpreter-k16",
                first: 203,
                last: 215,
            },
            BlockSpec {
                id: "filesystem-reading-api",
                owner: "read-path-k14",
                first: 216,
                last: 248,
            },
            BlockSpec {
                id: "filesystem-writing-api",
                owner: "filesystem-interpreter-k16",
                first: 249,
                last: 290,
            },
            BlockSpec {
                id: "filesystem-read-guard",
                owner: "read-path-k14",
                first: 291,
                last: 304,
            },
            BlockSpec {
                id: "filesystem-write-guard",
                owner: "filesystem-interpreter-k16",
                first: 305,
                last: 388,
            },
            BlockSpec {
                id: "filesystem-vacancy-api",
                owner: "filesystem-interpreter-k16",
                first: 389,
                last: 519,
            },
            BlockSpec {
                id: "filesystem-read-guard-api",
                owner: "read-path-k14",
                first: 520,
                last: 533,
            },
            BlockSpec {
                id: "filesystem-write-guard-api",
                owner: "filesystem-interpreter-k16",
                first: 534,
                last: 811,
            },
            BlockSpec {
                id: "filesystem-read-deref",
                owner: "read-path-k14",
                first: 812,
                last: 819,
            },
            BlockSpec {
                id: "filesystem-write-deref",
                owner: "filesystem-interpreter-k16",
                first: 820,
                last: 826,
            },
        ],
    },
    RootSpec {
        id: "source-filesystem-read",
        path: "crates/ordinal-fs-tree/src/fs/read.rs",
        lines: 407,
        blocks: &[BlockSpec {
            id: "read-filesystem-source",
            owner: "read-path-k14",
            first: 1,
            last: 407,
        }],
    },
    RootSpec {
        id: "source-filesystem-apply",
        path: "crates/ordinal-fs-tree/src/fs/apply.rs",
        lines: 488,
        blocks: &[BlockSpec {
            id: "filesystem-interpreter-source",
            owner: "filesystem-interpreter-k16",
            first: 1,
            last: 488,
        }],
    },
    RootSpec {
        id: "source-filesystem-remove",
        path: "crates/ordinal-fs-tree/src/fs/remove.rs",
        lines: 275,
        blocks: &[BlockSpec {
            id: "filesystem-removal-source",
            owner: "filesystem-interpreter-k16",
            first: 1,
            last: 275,
        }],
    },
    RootSpec {
        id: "source-filesystem-lock",
        path: "crates/ordinal-fs-tree/src/fs/lock.rs",
        lines: 91,
        blocks: &[BlockSpec {
            id: "filesystem-lock-source",
            owner: "filesystem-interpreter-k16",
            first: 1,
            last: 91,
        }],
    },
];

const PAGES: &[(&str, &str, &str, usize)] = &[
    ("orientation-k11", "orientation", "Orientation", 1),
    ("name-seam-k12", "name-seam", "Name seam", 2),
    (
        "reference-domain-k13",
        "reference-domain",
        "Reference domain",
        3,
    ),
    ("read-path-k14", "read-path", "Read path", 4),
    (
        "mutation-algebra-k15",
        "mutation-algebra",
        "Mutation algebra",
        5,
    ),
    (
        "filesystem-interpreter-k16",
        "filesystem-interpreter",
        "Filesystem interpreter",
        6,
    ),
    ("syllabus-cli-k17", "syllabus-cli", "Syllabus CLI", 7),
];

const EARLY_USES: &[(&str, &str, &str, &str)] = &[
    (
        "`Ordinal`, `Key`, `Found`, `Verdict`, `Species`, `EntryName`",
        "01-orientation.md#working-vocabulary",
        "name-seam-k12",
        "Ordinal is mutable sibling position, key is stable tree identity, observed file kind is not followed, verdict separates foreign, accepted, and refused names, species controls file versus directory shape, and EntryName is the consumer parsing and composition seam.",
    ),
    (
        "`manifest-cli-binary`",
        "01-orientation.md#package-contract",
        "syllabus-cli-k17",
        "The binary declaration is CLI-owned and deferred; it maps the demonstration executable to its external consumer source and requires the CLI feature.",
    ),
    (
        "`manifest-cli-feature`",
        "01-orientation.md#package-contract",
        "syllabus-cli-k17",
        "The optional parser dependency is activated by a later CLI-owned feature range, enabled by default while library consumers may disable default features.",
    ),
    (
        "`Sought`",
        "01-orientation.md#public-surface",
        "name-seam-k12",
        "Sought distinguishes a search match from a completed search that matched nothing; nothing is neither a mutation refusal nor an error, while accessors retain Option.",
    ),
    (
        "`Label`, `Status`, `reference::Parts`, `SyllabusName`",
        "01-orientation.md#insert-tour",
        "reference-domain-k13",
        "These values are the syllabus consumer's vocabulary and seam implementation, not library defaults.",
    ),
    (
        "`Snapshot`, `Entry`, `ReadGuard`",
        "01-orientation.md#insert-tour",
        "read-path-k14",
        "A snapshot is the immutable parsed tree captured under a guard, entries are borrowed views, and a read guard couples a shared lock, caller-spelled root, and snapshot.",
    ),
    (
        "`Target`, `NewEntry`, `Decision`, `Refusal`, `Plan`, `Effect`, `Report`",
        "01-orientation.md#insert-tour",
        "mutation-algebra-k15",
        "Target names the root or a stable key, new entry carries opaque parts and bytes that may be empty, every input yields refusal or a guarded ordered plan, and the report records landed effects in its documented orders.",
    ),
    (
        "`WriteGuard`, `Error`, `apply::Faults`, `apply::Run`",
        "01-orientation.md#insert-tour",
        "filesystem-interpreter-k16",
        "A write guard couples an exclusive lock and snapshot and is consumed by one mutation, errors distinguish refusal, clean rollback, partial rollback, and boundary failure, Faults is a test seam, and Run owns per-plan forward and undo state.",
    ),
    (
        "`Cli`, `Verb`, `Streams`, `Failure`",
        "01-orientation.md#insert-tour",
        "syllabus-cli-k17",
        "Parsed verbs drive dispatch, stdout is result data, stderr carries advisories and errors, and failure pairs operator-facing text with an exit category.",
    ),
];

/// The fixture book's `walkthrough.toml`, built from the same specs the pages
/// and ledger are built from, so the fixture cannot disagree with itself.
pub fn manifest_text() -> String {
    let mut out = String::from(
        "schema = 1\n\n[book]\nid = \"ordinal-fs-tree\"\ntitle = \"Ordinal filesystem tree\"\nsubject = \"crates/ordinal-fs-tree\"\n\n[corpus]\ninclude = [\"crates/ordinal-fs-tree/Cargo.toml\", \"crates/ordinal-fs-tree/src/**/*.rs\"]\n\n",
    );
    for (path, class) in CORPUS_ADD {
        out.push_str(&format!(
            "[[corpus.add]]\npath = \"{path}\"\nclass = \"{class}\"\nreason = \"fixture\"\n\n"
        ));
    }
    for (path, class) in CORPUS_EXCLUDE {
        out.push_str(&format!(
            "[[corpus.exclude]]\npath = \"{path}\"\nclass = \"{class}\"\nreason = \"fixture\"\n\n"
        ));
    }
    out.push_str("[[page]]\nfile = \"README.md\"\nid = \"contents\"\ntitle = \"Ordinal filesystem tree\"\nrole = \"contents\"\n\n");
    for (slice, id, title, order) in PAGES {
        let (file, _, _, _) = page(slice);
        let _ = order;
        out.push_str(&format!(
            "[[page]]\nfile = \"{file}\"\nid = \"{id}\"\ntitle = \"{title}\"\nrole = \"chapter\"\nslice = \"{slice}\"\n\n"
        ));
    }
    for (file, id, title) in [
        ("concept-index.md", "concept-index", "Concept index"),
        ("source-index.md", "source-index", "Source index"),
    ] {
        out.push_str(&format!(
            "[[page]]\nfile = \"{file}\"\nid = \"{id}\"\ntitle = \"{title}\"\nrole = \"lookup\"\n\n"
        ));
    }
    for root in ROOTS {
        out.push_str(&format!(
            "[[root]]\nid = \"{}\"\npath = \"{}\"\nlines = {}\n\n",
            root.id, root.path, root.lines
        ));
        for block in root.blocks {
            out.push_str(&format!(
                "[[block]]\nid = \"{}\"\nroot = \"{}\"\nowner = \"{}\"\nlines = \"{}-{}\"\n\n",
                block.id, root.id, block.owner, block.first, block.last
            ));
        }
    }
    for (symbols, first_use, owner, statement) in EARLY_USES {
        out.push_str(&format!(
            "[[early-use]]\nsymbols = \"{symbols}\"\nfirst-use = \"{first_use}\"\nowner = \"{owner}\"\nstatement = \"{statement}\"\n\n"
        ));
    }
    out.push_str("[guide]\nomitted = \"the fixture book is self-contained\"\n");
    out
}

/// Write the files this book's corpus exceptions name, so a temporary
/// repository derives the same corpus the real one does.
///
/// A `[[corpus.exclude]]` naming a path that is not there is `U002`: a stale
/// exception is a failure rather than a silent no-op, and a fixture repository
/// that omitted the excluded files would be exercising that refusal instead of
/// the book. The contents do not matter — the exclusion removes them before
/// anything reads them — only that the rule reaches a real file to exclude.
pub fn materialize_corpus_exceptions(repository: &std::path::Path) {
    for (path, _) in CORPUS_EXCLUDE.iter().chain(CORPUS_ADD) {
        let destination = repository.join(path);
        fs::create_dir_all(destination.parent().expect("exception path has a parent")).unwrap();
        if !destination.exists() {
            fs::write(destination, b"").unwrap();
        }
    }
}

pub fn manifest() -> Manifest {
    Manifest::load(BOOK_ROOT, &manifest_text()).expect("fixture manifest is schema-valid")
}

/// The fixture book's corpus, derived from the real repository.
///
/// Derived rather than restated even in the small synthetic snapshots: a
/// hand-written set would agree with the manifest by construction, and the
/// whole point of the derived set is that it is the one input the book's author
/// did not write.
pub fn derived_corpus() -> std::collections::BTreeSet<String> {
    book_validation::derive_corpus(&manifest(), &repository())
        .expect("the fixture book's corpus rule evaluates against the real repository")
}

/// A `--through` scope over the fixture book.
pub fn through(slice: &str) -> Scope {
    Scope::Through(
        manifest()
            .resolve_scoped(slice)
            .expect("fixture slice is in the book's scoped domain"),
    )
}

pub fn corpus(final_: bool) -> BookSnapshot {
    let repository = repository();
    let mut source_files = BTreeMap::new();
    let mut source_index = String::from(
        "# Source index\n<!-- book-page id=\"source-index\" role=\"lookup\" -->\n\n## Source roots\n\n| Root ID | Source path | Lines |\n|---|---|---|\n",
    );
    let mut pages: BTreeMap<String, String> = BTreeMap::new();

    for root in ROOTS {
        source_index.push_str(&format!(
            "| `{}` | `{}` | {} |\n",
            root.id,
            root.path,
            grouped(root.lines)
        ));
    }
    source_index.push('\n');

    for root in ROOTS {
        let bytes = fs::read(repository.join(root.path)).expect("read frozen source fixture");
        source_files.insert(root.path.into(), bytes.clone());
        source_index.push_str(&format!(
            "<!-- source-root «{}» source=\"{}\" lines=\"1-{}\" -->\n",
            root.id, root.path, root.lines
        ));
        for block in root.blocks {
            if final_ || block.owner == "orientation-k11" {
                source_index.push_str(&format!("<!-- insert «{}» -->\n", block.id));
                add_definition(&mut pages, root, block, &bytes);
            } else {
                source_index.push_str(&format!(
                    "<!-- defer «{}» owner=\"{}\" lines=\"{}-{}\" -->\n",
                    block.id, block.owner, block.first, block.last
                ));
            }
        }
        source_index.push_str("<!-- /source-root -->\n");
    }

    source_index.push_str("\n## Ownership blocks\n\n| Block ID | Root ID | Owner | Source lines | Count | State |\n|---|---|---|---|---|---|\n");
    for root in ROOTS {
        for block in root.blocks {
            let state = if final_ || block.owner == "orientation-k11" {
                "resolved"
            } else {
                "deferred"
            };
            source_index.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}-{}` | {} | `{}` |\n",
                block.id,
                root.id,
                block.owner,
                block.first,
                block.last,
                grouped(block.last - block.first + 1),
                state
            ));
        }
    }

    source_index.push_str("\n## Fragment index\n\n| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |\n|---|---|---|---|---|---|---|---|\n");
    for root in ROOTS {
        let children = root
            .blocks
            .iter()
            .map(|block| format!("`{}`", block.id))
            .collect::<Vec<_>>()
            .join(", ");
        source_index.push_str(&format!(
            "| `{}` | `source-index` | `{}` | `root` | `—` | `1-{}` | `—` | {} |\n",
            root.id, root.id, root.lines, children
        ));
        for block in root
            .blocks
            .iter()
            .filter(|block| final_ || block.owner == "orientation-k11")
        {
            source_index.push_str(&format!(
                "| `{}` | `{}` | `{}` | `literal` | `{}` | `{}-{}` | `{}` | `—` |\n",
                block.id,
                page(block.owner).1,
                root.id,
                block.owner,
                block.first,
                block.last,
                root.id
            ));
        }
    }

    source_index.push_str("\n## Early uses\n\n| Symbol family | First use | Owner | Minimum local statement | Status |\n|---|---|---|---|---|\n");
    for (symbols, first_use, owner, statement) in EARLY_USES {
        let status = if final_ { "explained" } else { "pending" };
        source_index.push_str(&format!(
            "| {symbols} | `{first_use}` | `{owner}` | {statement} | `{status}` |\n"
        ));
    }

    source_index.push_str(
        "\n<a id=\"owned-source-totals\"></a>\n## Owned source totals\n\nEvery line of the source roots is credited once, to the slice whose page owns\nit.\n\n| Slice | Page | Owned lines |\n|---|---|---:|\n",
    );
    for (slice, _, _, _) in PAGES {
        let owned: usize = ROOTS
            .iter()
            .flat_map(|root| root.blocks)
            .filter(|block| block.owner == *slice)
            .map(|block| block.last - block.first + 1)
            .sum();
        source_index.push_str(&format!(
            "| `{slice}` | `{}` | {} |\n",
            page(slice).0,
            grouped(owned)
        ));
    }
    source_index.push_str(&format!(
        "| **Total** | {} source roots | **{}** |\n",
        ROOTS.len(),
        grouped(ROOTS.iter().map(|root| root.lines).sum())
    ));

    let mut book_files = BTreeMap::from([(
        "docs/walkthroughs/ordinal-fs-tree/source-index.md".into(),
        source_index.into_bytes(),
    )]);
    book_files.extend(pages.into_iter().map(|(owner, body)| {
        let (filename, id, title, order) = page(&owner);
        let mut contents = format!(
            "# {title}\n<!-- book-page id=\"{id}\" slice=\"{owner}\" order=\"{order}\" -->\n"
        );
        if owner == "orientation-k11" {
            contents.push_str(
                "<a id=\"working-vocabulary\"></a>\n## Working vocabulary\n<a id=\"package-contract\"></a>\n## Package contract\n<a id=\"public-surface\"></a>\n## Public surface\n<a id=\"insert-tour\"></a>\n## Insert tour\n",
            );
        }
        contents.push_str(&body);
        (
            format!("docs/walkthroughs/ordinal-fs-tree/{filename}"),
            contents.into_bytes(),
        )
    }));
    let manifest = manifest();
    let mut book_entries: std::collections::BTreeSet<String> = book_files.keys().cloned().collect();
    book_entries.insert(manifest.manifest_path());
    let derived_corpus = book_validation::derive_corpus(&manifest, &repository)
        .expect("the fixture book's corpus rule evaluates against the real repository");
    BookSnapshot {
        manifest,
        derived_corpus,
        book_files,
        source_files,
        outbound_files: Default::default(),
        book_entries,
        non_regular_book_entries: Default::default(),
    }
}

fn page(owner: &str) -> (&'static str, &'static str, &'static str, usize) {
    let (_, id, title, order) = PAGES
        .iter()
        .find(|(slice, _, _, _)| *slice == owner)
        .expect("fixture owner has a canonical page");
    let filename = match *order {
        1 => "01-orientation.md",
        2 => "02-name-seam.md",
        3 => "03-reference-domain.md",
        4 => "04-read-path.md",
        5 => "05-mutation-algebra.md",
        6 => "06-filesystem-interpreter.md",
        7 => "07-syllabus-cli.md",
        _ => unreachable!("fixture contains only source-owning pages"),
    };
    (filename, id, title, *order)
}

fn grouped(value: usize) -> String {
    if value >= 1_000 {
        format!("{},{:03}", value / 1_000, value % 1_000)
    } else {
        value.to_string()
    }
}

fn add_definition(
    pages: &mut BTreeMap<String, String>,
    root: &RootSpec,
    block: &BlockSpec,
    source: &[u8],
) {
    let page = pages.entry(block.owner.into()).or_default();
    let body = source_lines(source, block.first, block.last);
    let language = if root.path.ends_with(".toml") {
        "toml"
    } else {
        "rust"
    };
    page.push_str(&format!(
        "<!-- fragment «{}» owner=\"{}\" source=\"{}\" lines=\"{}-{}\" parent=\"{}\" -->\n````{}\n",
        block.id, block.owner, root.path, block.first, block.last, root.id, language
    ));
    page.push_str(std::str::from_utf8(body).expect("source fixture is UTF-8"));
    page.push_str("````\n<!-- /fragment -->\n");
}

fn source_lines(source: &[u8], first: usize, last: usize) -> &[u8] {
    let starts: Vec<usize> = std::iter::once(0)
        .chain(
            source
                .iter()
                .enumerate()
                .filter_map(|(index, byte)| (*byte == b'\n').then_some(index + 1)),
        )
        .collect();
    &source[starts[first - 1]..if last < starts.len() {
        starts[last]
    } else {
        source.len()
    }]
}
