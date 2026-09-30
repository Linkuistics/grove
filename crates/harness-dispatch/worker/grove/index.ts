// harness-dispatch/grove — the Grove adapter. It reads the review relationship a
// Grove task file declares and returns it as the generic reviewed artifact that
// harness-dispatch/examples/review's rule reads.
//
// A Grove review leaf names what it reviews in two lines of its own body:
//
//   **Reviews:** parser-k12
//   **Creator:** run 5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34
//
// `**Reviews:**` names the reviewed producer by its Grove handle. `**Creator:**`
// names the producer's original creator: `run <run ID>` is the harness-dispatch
// run of the session that finished it, which that session read from its own
// HARNESS_DISPATCH_RUN_ID; `declared <origin>` is the owner's word for a
// producer finished without such a run. Those are Grove's documented task
// conventions (the plugin's TASK-FORMAT.md, and Grove's
// docs/adr/a-review-carries-its-creator-reference.md). This module depends on
// them and on harness-dispatch/sdk, and on nothing else, so it can move to
// Grove's side of an extraction unchanged.
//
// It is an explicit import, never part of ordinary dispatch. Use it through
// harness-dispatch/examples/grove-review, which composes it with the review
// rule over Grove's session kinds, or from a `loadContext` of your own:
//
//   import { groveContext } from "harness-dispatch/grove";
//   import { lookUpCreator } from "harness-dispatch/examples/review";
//   loadContext(request, host) {
//     const context = groveContext(request, host, reviews);
//     return "status" in context ? context : lookUpCreator(context, host);
//   }
//
// WHAT IT READS. The one task file the caller supplied with `--task-file`, on
// every invocation, through the host's measured read, so inspection and the run
// record show its digest. It takes the kind from the request, never from the
// file's name, and reads no other file: not the tree around it, its briefs, its
// siblings, or any record of a running session. It resolves no handle and
// selects no leaf.
//
// THE GRAMMAR. A marker line begins with `**Reviews:**` or `**Creator:**` at its
// first character; a mention inside a line, or an indented one, is not a
// marker line. Every marker line counts wherever it is, a fenced example
// included, since this module has no markdown parser to skip one. A marker line
// is exactly the marker, one space and the value, with nothing after it but the
// CR of a CRLF ending:
//
//   - `**Reviews:** <handle>`, a Grove handle: lowercase letters, digits and
//     single dashes, then `-k` and a key with no leading zero;
//   - `**Creator:** run <run ID>`, the ID in the canonical form harness-dispatch
//     writes, or `**Creator:** declared <origin>`, whose origin is the rest of
//     the line, verbatim. Nothing normalises it: the review rule matches it
//     exactly against the catalog's origins.
//
// THE RULE. For a kind the policy lists as a review, the task file must hold
// exactly one `**Reviews:**` line and exactly one `**Creator:**` line, each well
// formed, and they become the context's `reviewedArtifact`. For any other kind,
// a `**Reviews:**` line refuses, so a review of a kind the policy does not list
// cannot take a static route without the rule. Every failure is a refusal that
// names the task file, the line and its remedy; nothing is guessed. A task file
// that cannot be read refuses the selection too, as harness-dispatch reports
// any source a loader requires and cannot read.

import type { Context, ContextHost, Creator, Refused, SelectionRequest } from "harness-dispatch/sdk";

/**
 * This adapter's version: what it reads and how. Inspection and the run record
 * report it beside the worker's own version whenever a policy imports it.
 */
export const version = "1";

/**
 * The kinds a policy lists as reviews, each by its exact name as an own key:
 * a review-rule table such as harness-dispatch/examples/review's `reviews`,
 * whose values this adapter does not read.
 */
export type ReviewKinds = { readonly [kind: string]: unknown };

const REVIEWS = "**Reviews:**";
const CREATOR = "**Creator:**";
/** A Grove work-item handle: `<slug>-k<key>`. */
const HANDLE = /^[a-z0-9]+(?:-[a-z0-9]+)*-k[1-9][0-9]*$/;
/** A run ID as HARNESS_DISPATCH_RUN_ID gives it. */
const RUN_ID = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;

/** A marker line: its number, from 1, and its text without its line ending. */
interface Line {
  readonly number: number;
  readonly text: string;
}

/**
 * The context for a Grove session: the caller's context, or an empty one, with
 * the reviewed artifact the task file declares when `request.kind` is one of
 * `reviews`; or the refusal that says why there is none. Call it from
 * `loadContext`, since only its host reads. It returns a refusal rather than
 * throwing, and a `loadContext` that returns it refuses the selection with its
 * code and remedy.
 */
export function groveContext(request: SelectionRequest, host: ContextHost, reviews: ReviewKinds): Context | Refused {
  const kind = request.kind;
  const review = Object.hasOwn(reviews, kind);
  const given: Context = request.context ?? { schemaVersion: 1 };
  const path = request.taskFile;

  if (path === undefined) {
    if (!review) return given;
    return refuse(
      "task_file_missing",
      `kind ${quote(kind)} is a review, but no task file was supplied, ` +
        "so the artifact it reviews and that artifact's creator are unknown",
      "pass the review's task file with --task-file, as Grove's dispatch command does with ${task_file}; " +
        "a caller without a Grove task file names the reviewed artifact in a --context document " +
        "to a policy built on harness-dispatch/examples/review instead",
    );
  }
  if (review && given.reviewedArtifact !== undefined) {
    return refuse(
      "reviewed_artifact_conflict",
      `the caller's context names reviewed artifact ${quote(given.reviewedArtifact.id)}, ` +
        `and kind ${quote(kind)} takes its reviewed artifact from the task file's ${REVIEWS} and ${CREATOR} lines`,
      "leave reviewedArtifact out of the --context document: the task file's lines name the artifact and its creator",
    );
  }

  // The whole file, whatever its size within the context budget: its text
  // never enters the context, only its measured digest.
  const { text, source } = host.readText(path, request.limits.contextBytes);
  const file = `task file ${source.name}`;
  const found = markerLines(text);

  if (!review) {
    const line = found.reviews[0];
    if (line === undefined) return given;
    return refuse(
      "review_kind_unlisted",
      `${file} declares ${quote(shown(line))} at line ${line.number}, ` +
        `but kind ${quote(kind)} is not a review kind this policy lists`,
      `list ${quote(kind)} as a review kind in your copy of the policy, so the provider rule applies to it, ` +
        `or remove the ${REVIEWS} line if this task is not a review`,
    );
  }

  // The reviewed artifact: exactly one well-formed `**Reviews:**` line.
  const reviewsLine = only(found.reviews);
  if (reviewsLine === "none") {
    return refuse(
      "reviews_line_missing",
      `${file} has no ${REVIEWS} line, and kind ${quote(kind)} is a review, so the artifact it reviews is unknown`,
      `add a line "${REVIEWS} <handle>" under the task's heading, naming the producer it reviews by its Grove handle, ` +
        `with its "${CREATOR}" line directly under it`,
    );
  }
  if (reviewsLine === "several") {
    return refuse(
      "reviews_line_duplicate",
      `${file} has ${found.reviews.length} ${REVIEWS} lines, at lines ${numbers(found.reviews)}, ` +
        "and a review reviews one artifact",
      `keep one "${REVIEWS} <handle>" line; reword or indent the others, since a line that begins with ` +
        `${REVIEWS} counts wherever it is, in a fenced block too`,
    );
  }
  const id = reviewsLine.text.slice(REVIEWS.length + 1);
  if (reviewsLine.text[REVIEWS.length] !== " " || !HANDLE.test(id)) {
    return refuse(
      "reviews_line_malformed",
      `line ${reviewsLine.number} of ${file}, ${quote(shown(reviewsLine))}, is not "${REVIEWS} <handle>"`,
      `write "${REVIEWS} <handle>" with the reviewed producer's Grove handle, such as parser-k12: lowercase ` +
        "letters, digits and single dashes, then -k and its key, with one space before it and nothing after it",
    );
  }

  // Its original creator: exactly one well-formed `**Creator:**` line.
  const creatorLine = only(found.creator);
  if (creatorLine === "none") {
    return refuse(
      "creator_line_missing",
      `${file} reviews ${quote(id)} but has no ${CREATOR} line, so the origin of its creator is unknown. ` +
        "No run is looked up by its task: a run of the same task in the record store does not stand in for the line",
      `the session that finished ${id} writes "${CREATOR} run <run ID>" from its HARNESS_DISPATCH_RUN_ID, ` +
        `directly under the ${REVIEWS} line; if it finished without harness-dispatch, write ` +
        `"${CREATOR} declared <origin>" there yourself, naming the provider origin that made it as your catalog labels it`,
    );
  }
  if (creatorLine === "several") {
    return refuse(
      "creator_line_duplicate",
      `${file} has ${found.creator.length} ${CREATOR} lines, at lines ${numbers(found.creator)}, ` +
        `and ${quote(id)} has one original creator`,
      `keep the one line that names the session that finished ${id}: "${CREATOR} run <run ID>", or ` +
        `"${CREATOR} declared <origin>" for one finished without harness-dispatch`,
    );
  }
  const creator = parseCreator(creatorLine);
  if (typeof creator === "string") {
    return refuse(
      "creator_line_malformed",
      `line ${creatorLine.number} of ${file}, ${quote(shown(creatorLine))}, is not ` +
        `"${CREATOR} run <run ID>" or "${CREATOR} declared <origin>": ${creator}`,
      `write "${CREATOR} run <run ID>", with the ID exactly as HARNESS_DISPATCH_RUN_ID gave it, ` +
        `or "${CREATOR} declared <origin>", with one space after each word`,
    );
  }

  return { ...given, reviewedArtifact: { id, creator } };
}

/** The creator a well-formed `**Creator:**` line names, or why it names none. */
function parseCreator(line: Line): Creator | string {
  if (line.text[CREATOR.length] !== " ") return `one space must follow ${CREATOR}`;
  const value = line.text.slice(CREATOR.length + 1);
  if (value.startsWith("run ")) {
    const run = value.slice("run ".length);
    if (RUN_ID.test(run)) return { run };
    return "a run is named by its ID in the canonical form: 36 lowercase hexadecimal digits and hyphens";
  }
  if (value.startsWith("declared ")) {
    const declared = value.slice("declared ".length);
    if (declared.trim() !== "") return { declared };
    return "a declaration names an origin";
  }
  return "the value begins with neither run nor declared";
}

/** Every `**Reviews:**` and `**Creator:**` line in `text`, in order. */
function markerLines(text: string): { reviews: Line[]; creator: Line[] } {
  const reviews: Line[] = [];
  const creator: Line[] = [];
  text.split("\n").forEach((raw, index) => {
    const line = { number: index + 1, text: raw.endsWith("\r") ? raw.slice(0, -1) : raw };
    if (line.text.startsWith(REVIEWS)) reviews.push(line);
    else if (line.text.startsWith(CREATOR)) creator.push(line);
  });
  return { reviews, creator };
}

/** The one line in `lines`, or whether there are none or several. */
function only(lines: readonly Line[]): Line | "none" | "several" {
  if (lines.length === 0) return "none";
  if (lines.length > 1) return "several";
  return lines[0] as Line;
}

function numbers(lines: readonly Line[]): string {
  const all = lines.map((line) => String(line.number));
  return all.length === 2 ? all.join(" and ") : `${all.slice(0, -1).join(", ")} and ${all.at(-1)}`;
}

/** A line as a refusal quotes it: cut short, so that a long one cannot crowd out the rest. */
function shown(line: Line): string {
  return line.text.length > 160 ? `${line.text.slice(0, 160)}…` : line.text;
}

function quote(text: string): string {
  return JSON.stringify(text);
}

function refuse(code: string, message: string, remedy: string): Refused {
  return { status: "refused", code, message, remedy };
}
