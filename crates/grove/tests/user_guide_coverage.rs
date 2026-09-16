//! The user guide, against the inventory it is obliged to cover.
//!
//! `docs/specs/user-guide-coverage.md` is a **standard**, not a description: it
//! was written and committed before `docs/USAGE.md` was edited against it, so
//! that "complete" names a checkable set rather than a judgement
//! (`user-guide-k11`'s brief). A standard nothing reads is a standard that
//! drifts, and the drift is silent in both directions — a row added to the
//! inventory that the guide never grows, and a guide section deleted while its
//! row stays.
//!
//! So the two documents are made to **agree**, in the idiom
//! `corpus_exception_inventory.rs` already uses for a book's corpus exceptions:
//! the inventory names the rows and the guide's own coverage map answers them,
//! and neither is trusted about itself. That is the whole reason this is a
//! repository test rather than a paragraph in the guide claiming completeness.
//!
//! Three agreements, and each one fails a different mistake:
//!
//! 1. **Every row is answered exactly once.** A row the map omits is uncovered;
//!    a row the map names twice is a copy-paste that hides which section is
//!    really responsible.
//! 2. **Every stable entry point exists in the explicit anchor form.** The
//!    walkthrough specification lets a book reserve an anchor from that table
//!    and cite it from its `README.md`, so an anchorless guide leaves the next
//!    five books unable to conform. A renderer-generated heading slug does not
//!    satisfy it — a slug changes silently when a heading is retitled, which is
//!    exactly the failure the explicit form exists to stop.
//! 3. **Every anchor the map points at resolves.** The map's links are
//!    same-document fragments, which `every_repository_markdown_reference_resolves`
//!    deliberately skips ("the second resolves within the rendering page"), so
//!    without this they are the one class of link in the guide nothing checks.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

mod support;

const INVENTORY: &str = "docs/specs/user-guide-coverage.md";
const GUIDE: &str = "docs/USAGE.md";

/// The row ids the inventory declares, in the order it declares them and with
/// **every occurrence kept**, read from the leading cell of every table row
/// whose first cell is `G`/`L`/`J` followed by digits.
///
/// A scan rather than a table parse: the inventory is prose with tables in it,
/// and the row id is the one thing about it that has a fixed shape. Anything
/// else here would be this test having an opinion about the standard's layout.
///
/// **Occurrences, not a set, and that is the whole point of the signature.**
/// Collapsing these into a `BTreeSet` on the way out would make a second,
/// *different* obligation accidentally filed under an existing id vanish behind
/// the first — and the guide's single map entry for that id would go on
/// satisfying every equality check, so one obligation would disappear in the one
/// place the test exists to watch. The guide's side already keeps occurrences
/// for exactly this reason (see [`coverage_map`]); the standard's side is not
/// entitled to less. [`unique_inventory_rows`] is where they are collapsed,
/// after uniqueness has been asserted.
fn inventory_row_occurrences(text: &str) -> Vec<String> {
    let mut rows = Vec::new();
    for line in outside_fences(text) {
        let Some(rest) = line.trim().strip_prefix('|') else {
            continue;
        };
        let Some((first, _)) = rest.split_once('|') else {
            continue;
        };
        let cell = first.trim();
        if is_row_id(cell) {
            rows.push(cell.to_owned());
        }
    }
    rows
}

/// The declared row ids as a set, refusing a duplicate rather than absorbing it.
///
/// Returns the offending ids on the error side so the caller can name them; a
/// panic here would put the message in the wrong place and make the rule
/// untestable from a unit test.
fn unique_inventory_rows(text: &str) -> Result<BTreeSet<String>, Vec<String>> {
    let mut rows = BTreeSet::new();
    let mut duplicated = BTreeSet::new();
    for row in inventory_row_occurrences(text) {
        if !rows.insert(row.clone()) {
            duplicated.insert(row);
        }
    }
    if duplicated.is_empty() {
        Ok(rows)
    } else {
        Err(duplicated.into_iter().collect())
    }
}

fn is_row_id(cell: &str) -> bool {
    let mut characters = cell.chars();
    let Some(prefix) = characters.next() else {
        return false;
    };
    matches!(prefix, 'G' | 'L' | 'J')
        && characters.clone().count() > 0
        && characters.all(|character| character.is_ascii_digit())
}

/// The anchors the inventory's *Stable entry points* table reserves: the
/// backticked first cell of every row in it.
///
/// The section is found by its heading rather than by position, because the
/// table's *place* in the standard is not something the guide should depend on.
fn reserved_anchors(text: &str) -> BTreeSet<String> {
    let mut anchors = BTreeSet::new();
    let mut inside = false;
    for line in outside_fences(text) {
        if let Some(heading) = line.strip_prefix("## ") {
            inside = heading.trim() == "Stable entry points";
            continue;
        }
        if line.starts_with("## ") {
            inside = false;
        }
        if !inside {
            continue;
        }
        let Some(rest) = line.trim().strip_prefix('|') else {
            continue;
        };
        let Some((first, _)) = rest.split_once('|') else {
            continue;
        };
        let cell = first.trim();
        if let Some(anchor) = cell.strip_prefix('`').and_then(|c| c.strip_suffix('`')) {
            anchors.insert(anchor.to_owned());
        }
    }
    anchors
}

/// The guide's coverage map: row id → the fragment its link points at.
///
/// Rows are read from the whole document rather than from the map's section
/// alone, so a second table claiming to answer a row is caught by the
/// answered-exactly-once assertion instead of being quietly ignored.
fn coverage_map(text: &str) -> BTreeMap<String, Vec<String>> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in outside_fences(text) {
        let Some(rest) = line.trim().strip_prefix('|') else {
            continue;
        };
        let Some((first, remainder)) = rest.split_once('|') else {
            continue;
        };
        let cell = first.trim();
        if !is_row_id(cell) {
            continue;
        }
        let fragment = remainder
            .split_once("](#")
            .and_then(|(_, tail)| tail.split_once(')'))
            .map(|(fragment, _)| fragment.to_owned());
        map.entry(cell.to_owned()).or_default().extend(fragment);
    }
    map
}

/// Every explicit `<a id="…"></a>` line that immediately precedes a heading,
/// outside a fenced block.
///
/// The three conditions are the contract, not this test's strictness:
/// `book-validation`'s own outbound-link check reads a guide by exactly this
/// rule, so an anchor that passes here is one a book can reserve.
fn explicit_heading_anchors(text: &str) -> BTreeSet<String> {
    let lines: Vec<&str> = text.lines().collect();
    let fenced = fence_mask(&lines);
    let mut anchors = BTreeSet::new();
    for (index, line) in lines.iter().enumerate() {
        if fenced[index] {
            continue;
        }
        let Some(anchor) = line
            .strip_prefix("<a id=\"")
            .and_then(|rest| rest.strip_suffix("\"></a>"))
        else {
            continue;
        };
        if lines
            .get(index + 1)
            .is_some_and(|next| next.starts_with('#') && next.contains(' '))
        {
            anchors.insert(anchor.to_owned());
        }
    }
    anchors
}

/// Every heading's GitHub-generated slug, so the map may point at an ordinary
/// subsection that reserves no anchor of its own.
fn heading_slugs(text: &str) -> BTreeSet<String> {
    let lines: Vec<&str> = text.lines().collect();
    let fenced = fence_mask(&lines);
    let mut slugs = BTreeSet::new();
    for (index, line) in lines.iter().enumerate() {
        if fenced[index] {
            continue;
        }
        let level = line.bytes().take_while(|byte| *byte == b'#').count();
        if level == 0 || line.as_bytes().get(level) != Some(&b' ') {
            continue;
        }
        let mut slug = String::new();
        for character in line[level + 1..].trim().chars() {
            match character {
                ' ' => slug.push('-'),
                character if character.is_alphanumeric() || matches!(character, '-' | '_') => {
                    slug.extend(character.to_lowercase());
                }
                _ => {}
            }
        }
        slugs.insert(slug);
    }
    slugs
}

fn fence_mask(lines: &[&str]) -> Vec<bool> {
    let mut mask = Vec::with_capacity(lines.len());
    let mut fence: Option<usize> = None;
    for line in lines {
        let backticks = line.bytes().take_while(|byte| *byte == b'`').count();
        match fence {
            Some(opened) => {
                mask.push(true);
                if backticks >= opened && line.trim_end().len() == backticks {
                    fence = None;
                }
            }
            None => {
                if backticks >= 3 {
                    fence = Some(backticks);
                    mask.push(true);
                } else {
                    mask.push(false);
                }
            }
        }
    }
    mask
}

fn outside_fences(text: &str) -> Vec<&str> {
    let lines: Vec<&str> = text.lines().collect();
    let fenced = fence_mask(&lines);
    lines
        .into_iter()
        .zip(fenced)
        .filter_map(|(line, inside)| (!inside).then_some(line))
        .collect()
}

fn read(root: &Path, relative: &str) -> String {
    fs::read_to_string(root.join(relative))
        .unwrap_or_else(|error| panic!("reading {relative}: {error}"))
}

#[test]
fn the_guide_answers_every_inventory_row_exactly_once() {
    let root = support::repo_root();
    let rows = unique_inventory_rows(&read(&root, INVENTORY)).unwrap_or_else(|duplicated| {
        panic!(
            "{INVENTORY} declares these row ids more than once, \
             so one obligation is hidden behind another: {duplicated:?}"
        )
    });
    let map = coverage_map(&read(&root, GUIDE));

    assert!(
        rows.len() > 30,
        "the inventory should declare the whole command and journey surface, found {}",
        rows.len()
    );

    let answered: BTreeSet<String> = map.keys().cloned().collect();
    let uncovered: Vec<&String> = rows.difference(&answered).collect();
    assert!(
        uncovered.is_empty(),
        "{INVENTORY} rows with no entry in {GUIDE}'s coverage map: {uncovered:?}"
    );

    let unknown: Vec<&String> = answered.difference(&rows).collect();
    assert!(
        unknown.is_empty(),
        "{GUIDE}'s coverage map names rows {INVENTORY} does not declare: {unknown:?}"
    );

    let duplicated: Vec<&String> = map
        .iter()
        .filter(|(_, targets)| targets.len() > 1)
        .map(|(row, _)| row)
        .collect();
    assert!(
        duplicated.is_empty(),
        "{GUIDE}'s coverage map answers these rows more than once, so no section owns them: {duplicated:?}"
    );
}

#[test]
fn every_reserved_entry_point_carries_an_explicit_anchor() {
    let root = support::repo_root();
    let reserved = reserved_anchors(&read(&root, INVENTORY));
    let carried = explicit_heading_anchors(&read(&root, GUIDE));

    assert!(
        !reserved.is_empty(),
        "{INVENTORY} must reserve at least one stable entry point"
    );

    let missing: Vec<&String> = reserved.difference(&carried).collect();
    assert!(
        missing.is_empty(),
        "{GUIDE} must carry each reserved entry point as an `<a id=\"…\"></a>` line \
         immediately preceding its heading; missing: {missing:?}"
    );
}

#[test]
fn every_coverage_map_link_resolves_within_the_guide() {
    let root = support::repo_root();
    let guide = read(&root, GUIDE);
    let map = coverage_map(&guide);
    let mut targets = explicit_heading_anchors(&guide);
    targets.extend(heading_slugs(&guide));

    let dangling: Vec<String> = map
        .iter()
        .flat_map(|(row, fragments)| fragments.iter().map(move |fragment| (row, fragment)))
        .filter(|(_, fragment)| !targets.contains(*fragment))
        .map(|(row, fragment)| format!("{row} → #{fragment}"))
        .collect();
    assert!(
        dangling.is_empty(),
        "{GUIDE}'s coverage map points at fragments that match no anchor or heading: {dangling:?}"
    );

    let unlinked: Vec<&String> = map
        .iter()
        .filter(|(_, fragments)| fragments.is_empty())
        .map(|(row, _)| row)
        .collect();
    assert!(
        unlinked.is_empty(),
        "every coverage-map row must link to the section that answers it; these do not: {unlinked:?}"
    );
}

/// The counterexample the reader of this file most needs to see fail.
///
/// `the_guide_answers_every_inventory_row_exactly_once` reads its name as a
/// statement about *both* documents, but the equality checks alone cannot make
/// it one: two rows filed under a single id are one member of any set, so the
/// guide's single map entry answers both and every difference comes back empty.
/// The duplicate has to be refused where it is read, and this is the test that
/// says so about the reading rather than about the repository's current files —
/// which, being correct, cannot exercise the rule at all.
#[test]
fn a_duplicated_inventory_row_id_is_refused_rather_than_absorbed() {
    let standard = "\
| Row | Verb | Obligation |\n\
|---|---|---|\n\
| L1 | `root-init` | Worked. |\n\
| L2 | `pick` | Worked. |\n\
\n\
| Row | Journey | End state |\n\
|---|---|---|\n\
| L2 | a second, different obligation under a used id | Worked. |\n";

    assert_eq!(
        inventory_row_occurrences(standard),
        vec!["L1", "L2", "L2"],
        "occurrences must survive the scan, or the duplicate is gone before it can be refused"
    );
    assert_eq!(
        unique_inventory_rows(standard),
        Err(vec!["L2".to_owned()]),
        "a repeated row id must be reported, not collapsed into the id it repeats"
    );

    let sound = "| Row | Verb | Obligation |\n|---|---|---|\n| L1 | `root-init` | Worked. |\n";
    assert_eq!(
        unique_inventory_rows(sound).map(|rows| rows.into_iter().collect::<Vec<_>>()),
        Ok(vec!["L1".to_owned()])
    );
}

#[test]
fn inspection_options_in_binary_help_are_explained_by_the_guide() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_grove"))
        .args(["config", "show", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    let guide = read(&support::repo_root(), GUIDE);
    let section = guide
        .split("## Inspecting configuration before launch\n")
        .nth(1)
        .unwrap()
        .split("\n## ")
        .next()
        .unwrap();
    let options: BTreeSet<_> = help
        .lines()
        .flat_map(|line| line.split_whitespace())
        .filter(|word| word.starts_with("--"))
        .map(|word| word.trim_end_matches(','))
        .collect();
    assert!(options.contains("--json"));
    for option in options {
        assert!(
            section.contains(option),
            "inspection option {option} is absent from its guide section"
        );
    }
}
