use std::{collections::BTreeMap, fs, path::PathBuf};

use book_validation::BookSnapshot;

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
        lines: 116,
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
                last: 116,
            },
        ],
    },
    RootSpec {
        id: "source-syllabus-cli",
        path: "crates/ordinal-fs-tree/bin/syllabus.rs",
        lines: 1_128,
        blocks: &[BlockSpec {
            id: "syllabus-cli-source",
            owner: "syllabus-cli-k17",
            first: 1,
            last: 1_128,
        }],
    },
    RootSpec {
        id: "source-library",
        path: "crates/ordinal-fs-tree/src/lib.rs",
        lines: 94,
        blocks: &[BlockSpec {
            id: "library-crate-surface",
            owner: "orientation-k11",
            first: 1,
            last: 94,
        }],
    },
    RootSpec {
        id: "source-conformance",
        path: "crates/ordinal-fs-tree/src/conformance.rs",
        lines: 636,
        blocks: &[BlockSpec {
            id: "reference-conformance-source",
            owner: "reference-domain-k13",
            first: 1,
            last: 636,
        }],
    },
    RootSpec {
        id: "source-error",
        path: "crates/ordinal-fs-tree/src/error.rs",
        lines: 342,
        blocks: &[BlockSpec {
            id: "filesystem-error-source",
            owner: "filesystem-interpreter-k16",
            first: 1,
            last: 342,
        }],
    },
    RootSpec {
        id: "source-name",
        path: "crates/ordinal-fs-tree/src/name.rs",
        lines: 700,
        blocks: &[BlockSpec {
            id: "name-seam-source",
            owner: "name-seam-k12",
            first: 1,
            last: 700,
        }],
    },
    RootSpec {
        id: "source-operations",
        path: "crates/ordinal-fs-tree/src/ops.rs",
        lines: 543,
        blocks: &[BlockSpec {
            id: "mutation-operations-source",
            owner: "mutation-algebra-k15",
            first: 1,
            last: 543,
        }],
    },
    RootSpec {
        id: "source-plan",
        path: "crates/ordinal-fs-tree/src/plan.rs",
        lines: 568,
        blocks: &[BlockSpec {
            id: "mutation-plan-source",
            owner: "mutation-algebra-k15",
            first: 1,
            last: 568,
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
        lines: 152,
        blocks: &[BlockSpec {
            id: "mutation-report-source",
            owner: "mutation-algebra-k15",
            first: 1,
            last: 152,
        }],
    },
    RootSpec {
        id: "source-snapshot",
        path: "crates/ordinal-fs-tree/src/snapshot.rs",
        lines: 650,
        blocks: &[BlockSpec {
            id: "read-snapshot-source",
            owner: "read-path-k14",
            first: 1,
            last: 650,
        }],
    },
    RootSpec {
        id: "source-filesystem-module",
        path: "crates/ordinal-fs-tree/src/fs/mod.rs",
        lines: 393,
        blocks: &[
            BlockSpec {
                id: "filesystem-read-opening",
                owner: "read-path-k14",
                first: 1,
                last: 86,
            },
            BlockSpec {
                id: "filesystem-write-acquire",
                owner: "filesystem-interpreter-k16",
                first: 87,
                last: 105,
            },
            BlockSpec {
                id: "filesystem-read-acquire-and-guard",
                owner: "read-path-k14",
                first: 106,
                last: 131,
            },
            BlockSpec {
                id: "filesystem-write-guard",
                owner: "filesystem-interpreter-k16",
                first: 132,
                last: 154,
            },
            BlockSpec {
                id: "filesystem-read-guard-api",
                owner: "read-path-k14",
                first: 155,
                last: 168,
            },
            BlockSpec {
                id: "filesystem-write-guard-api",
                owner: "filesystem-interpreter-k16",
                first: 169,
                last: 378,
            },
            BlockSpec {
                id: "filesystem-read-deref",
                owner: "read-path-k14",
                first: 379,
                last: 386,
            },
            BlockSpec {
                id: "filesystem-write-deref",
                owner: "filesystem-interpreter-k16",
                first: 387,
                last: 393,
            },
        ],
    },
    RootSpec {
        id: "source-filesystem-read",
        path: "crates/ordinal-fs-tree/src/fs/read.rs",
        lines: 179,
        blocks: &[BlockSpec {
            id: "read-filesystem-source",
            owner: "read-path-k14",
            first: 1,
            last: 179,
        }],
    },
    RootSpec {
        id: "source-filesystem-apply",
        path: "crates/ordinal-fs-tree/src/fs/apply.rs",
        lines: 471,
        blocks: &[BlockSpec {
            id: "filesystem-interpreter-source",
            owner: "filesystem-interpreter-k16",
            first: 1,
            last: 471,
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

pub fn corpus(final_: bool) -> BookSnapshot {
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut source_files = BTreeMap::new();
    let mut source_index = String::new();
    let mut pages: BTreeMap<String, String> = BTreeMap::new();

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

    let mut book_files = BTreeMap::from([(
        "docs/ordinal-fs-tree/book/source-index.md".into(),
        source_index.into_bytes(),
    )]);
    book_files.extend(pages.into_iter().map(|(owner, page)| {
        (
            format!("docs/ordinal-fs-tree/book/{owner}.md"),
            page.into_bytes(),
        )
    }));
    BookSnapshot {
        book_files,
        source_files,
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
