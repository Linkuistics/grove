// Grove grove.rhai — Grove's task conventions, read through Ravel's measured
// reads. Runtime: Ravel 2.x, Rhai, SCHEMA_VERSION 4. Its Ravel 1.x counterpart
// is `grove.ts`, with the same refusals.
//
// `examples/grove-review.rhai` calls `grove_context` from its `load_context`.
// It imports `provenance.rhai` by its path, so keep the two files together.
//
// Rhai has no regular expressions, so `handle` checks the handle grammar
// character by character. Labels use the ECMAScript whitespace set of
// `provenance.rhai`, so a creator line gives the outcome of `grove.ts`. One
// difference remains: a recorded receipt that escapes an unpaired surrogate
// refuses here as `creator_line_malformed`. `provenance.rhai` gives the reason.

import "provenance" as provenance;

export const VERSION = "1";

const REVIEWS = "**Reviews:**";
const CREATOR = "**Creator:**";

/// Whether `id` matches `^[a-z0-9]+(?:-[a-z0-9]+)*-k[1-9][0-9]*$`.
fn handle(id) {
    let parts = id.split("-");
    if parts.len() < 2 {
        return false;
    }
    let key = parts.pop();
    if key.len() < 2 || !key.starts_with("k") || !"123456789".contains(key[1]) {
        return false;
    }
    for digit in key.sub_string(2) {
        if !"0123456789".contains(digit) {
            return false;
        }
    }
    for part in parts {
        if part == "" {
            return false;
        }
        for character in part {
            if !"abcdefghijklmnopqrstuvwxyz0123456789".contains(character) {
                return false;
            }
        }
    }
    true
}

/// The lines of `text` that start with each marker, with their numbers. A line
/// loses one trailing carriage return.
fn marker_lines(text) {
    let found = #{ reviews: [], creator: [] };
    let number = 0;
    for raw in text.split("\n") {
        number += 1;
        let line = if raw.ends_with("\r") { raw.sub_string(0, raw.len() - 1) } else { raw };
        if line.starts_with(global::REVIEWS) {
            found.reviews.push(#{ number: number, text: line });
        } else if line.starts_with(global::CREATOR) {
            found.creator.push(#{ number: number, text: line });
        }
    }
    found
}

/// The character of `text` directly after `marker`, or `()` at its end.
fn after(text, marker) {
    if text.len() > marker.len() { text[marker.len()] } else { () }
}

/// The creator evidence of `line`, or `()` when it is malformed.
fn parse_creator(line) {
    if after(line.text, global::CREATOR) != ' ' {
        return ();
    }
    let value = line.text.sub_string(global::CREATOR.len() + 1);
    if value.starts_with("recorded ") {
        let receipt = provenance::parse_evidence(value.sub_string("recorded ".len()));
        return if receipt == () { () } else { #{ evidence: "recorded", provider: receipt.provider, receipt: receipt } };
    }
    if value.starts_with("declared ") {
        let provider = value.sub_string("declared ".len());
        if !provenance::blank(provider) && !provenance::multiline(provider) {
            return #{ evidence: "declared", provider: provider };
        }
    }
    ()
}

fn refuse(code, message, remedy) {
    #{ status: "refused", code: code, message: message, remedy: remedy }
}

fn duplicate(marker, lines, file) {
    let numbers = "";
    for line in lines {
        if numbers != "" {
            numbers += ", ";
        }
        numbers += `${line.number}`;
    }
    refuse(`${marker}_line_duplicate`, `${file} has duplicate ${marker} lines at ${numbers}`,
        "keep one relationship and its finishing creator; indent examples, since markers count in fenced blocks too")
}

/// The selection context of `request` for a `load_context`, or a refusal.
/// `reviews` is the review table: its keys are the kinds that are reviews.
///
/// Read only the supplied task. Markers start in column one and count even in
/// fences; indented or inline mentions do not. One Reviews line and one
/// Creator immediately beneath it are required. CRLF is accepted, and values
/// are verbatim. Caller review facts conflict rather than overriding committed
/// task evidence.
fn grove_context(request, reviews) {
    let review = reviews.contains(request.kind);
    let given = if request.context == () { #{ schemaVersion: 1 } } else { request.context };
    let kind = ravel::json_encode(request.kind);
    if request.taskFile == () {
        return if review {
            refuse("task_file_missing", `kind ${kind} is a review but has no task file`,
                "pass --task-file for the review, or use the generic review selector with explicit caller facts")
        } else {
            given
        };
    }
    let facts = given.get("facts");
    if review && facts != () && (facts.contains("reviewedArtifact") || facts.contains("creator")) {
        return refuse("reviewed_artifact_conflict", "caller review facts conflict with the task's Reviews/Creator lines",
            "leave facts.reviewedArtifact and facts.creator out of --context; the task names them");
    }
    let read = ravel::read_text(request.taskFile, request.limits.contextBytes);
    let found = marker_lines(read.text);
    let file = `task file ${read.source.name}`;
    let sources = given.get("sources") ?? [];
    sources.push(read.source);
    if !review {
        if found.reviews.len() != 0 {
            return refuse("review_kind_unlisted",
                `${file} declares ${global::REVIEWS} at line ${found.reviews[0].number}, but kind ${kind} is not listed as a review`,
                "list the kind in reviews so the provider rule applies, or remove Reviews for a nonreview task");
        }
        given.sources = sources;
        return given;
    }
    if found.reviews.len() == 0 {
        return refuse("reviews_line_missing", `${file} has no ${global::REVIEWS} line`,
            `add "${global::REVIEWS} <handle>" and a Creator line directly beneath it`);
    }
    if found.reviews.len() > 1 {
        return duplicate("reviews", found.reviews, file);
    }
    let reviews_line = found.reviews[0];
    let id = if after(reviews_line.text, global::REVIEWS) == () { "" } else { reviews_line.text.sub_string(global::REVIEWS.len() + 1) };
    if after(reviews_line.text, global::REVIEWS) != ' ' || !handle(id) {
        return refuse("reviews_line_malformed",
            `line ${reviews_line.number} of ${file} is not "${global::REVIEWS} <handle>"`,
            "use lowercase letters, digits and single dashes then -k and a key with no leading zero; one space, no trailing text");
    }
    if found.creator.len() == 0 {
        return refuse("creator_line_missing",
            `${file} reviews ${ravel::json_encode(id)} but its creator is unknown. An earlier same-task attempt supplies no evidence`,
            `have the owner write "${global::CREATOR} declared <provider>" directly under Reviews, or copy the finishing session's recorded receipt`);
    }
    if found.creator.len() > 1 {
        return duplicate("creator", found.creator, file);
    }
    let creator_line = found.creator[0];
    if creator_line.number != reviews_line.number + 1 {
        return refuse("creator_line_misplaced",
            `${file}'s Creator at line ${creator_line.number} is not directly beneath Reviews at line ${reviews_line.number}`,
            "move the one Creator line directly beneath Reviews");
    }
    if creator_line.text.starts_with(`${global::CREATOR} run `) {
        return refuse("creator_legacy_unsupported",
            `${file} names an original-v22 dispatch run; the new consumer has no run lookup`,
            `have the owner write "${global::CREATOR} declared <provider>" or finish again under a receipt-producing policy; keep the original v22 driver's ending contract`);
    }
    let creator = parse_creator(creator_line);
    if creator == () {
        return refuse("creator_line_malformed", `line ${creator_line.number} of ${file} has invalid creator evidence`,
            `repair "${global::CREATOR} recorded <canonical receipt>" or have the owner write "${global::CREATOR} declared <provider>"; no duplicate/extra fields, newlines or NULs, at most 8192 receipt bytes`);
    }
    let facts = if facts == () { #{} } else { facts };
    facts.reviewedArtifact = #{ id: id };
    facts.creator = creator;
    given.facts = facts;
    given.sources = sources;
    given
}
