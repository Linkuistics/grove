// Grove task conventions, read through Ravel 0.1.0's measured public host.
// https://github.com/Linkuistics/Ravel/blob/v0.1.0/crates/ravel/worker/sdk/index.ts
import type { Context, ContextHost, Refused, SelectionRequest } from "ravel/sdk";
import { parseEvidence } from "./provenance.ts";
import type { Creator } from "./examples/review.ts";

export const version = "1";
export type ReviewKinds = { readonly [kind: string]: unknown };
const REVIEWS = "**Reviews:**";
const CREATOR = "**Creator:**";
const HANDLE = /^[a-z0-9]+(?:-[a-z0-9]+)*-k[1-9][0-9]*$/;
interface Line { readonly number: number; readonly text: string }

/**
 * Read only the supplied task. Markers start in column one and count even in
 * fences; indented/inline mentions do not. One Reviews line and one Creator
 * immediately beneath it are required. CRLF is accepted; values are verbatim.
 * Caller review facts conflict rather than overriding committed task evidence.
 */
export function groveContext(request: SelectionRequest, host: ContextHost, reviews: ReviewKinds): Context | Refused {
  const review = Object.hasOwn(reviews, request.kind);
  const given = request.context ?? { schemaVersion: 1 };
  if (request.taskFile === undefined) return review
    ? refuse("task_file_missing", `kind ${JSON.stringify(request.kind)} is a review but has no task file`,
      "pass --task-file for the review, or use the generic review selector with explicit caller facts") : given;
  if (review && (given.facts?.reviewedArtifact !== undefined || given.facts?.creator !== undefined)) return refuse(
    "reviewed_artifact_conflict", "caller review facts conflict with the task's Reviews/Creator lines",
    "leave facts.reviewedArtifact and facts.creator out of --context; the task names them");
  const { text, source } = host.readText(request.taskFile, request.limits.contextBytes);
  const found = markerLines(text);
  const file = `task file ${source.name}`;
  if (!review) {
    if (found.reviews.length !== 0) return refuse("review_kind_unlisted",
      `${file} declares ${REVIEWS} at line ${found.reviews[0]?.number}, but kind ${JSON.stringify(request.kind)} is not listed as a review`,
      "list the kind in reviews so the provider rule applies, or remove Reviews for a nonreview task");
    return { ...given, sources: [...(given.sources ?? []), source] };
  }
  if (found.reviews.length === 0) return refuse("reviews_line_missing", `${file} has no ${REVIEWS} line`,
    `add "${REVIEWS} <handle>" and a Creator line directly beneath it`);
  if (found.reviews.length > 1) return duplicate("reviews", found.reviews, file);
  const reviewsLine = found.reviews[0]!;
  const id = reviewsLine.text.slice(REVIEWS.length + 1);
  if (reviewsLine.text[REVIEWS.length] !== " " || !HANDLE.test(id)) return refuse("reviews_line_malformed",
    `line ${reviewsLine.number} of ${file} is not "${REVIEWS} <handle>"`,
    "use lowercase letters, digits and single dashes then -k and a key with no leading zero; one space, no trailing text");
  if (found.creator.length === 0) return refuse("creator_line_missing", `${file} reviews ${JSON.stringify(id)} but its creator is unknown. An earlier same-task attempt supplies no evidence`,
    `have the owner write "${CREATOR} declared <provider>" directly under Reviews, or copy the finishing session's recorded receipt`);
  if (found.creator.length > 1) return duplicate("creator", found.creator, file);
  const creatorLine = found.creator[0]!;
  if (creatorLine.number !== reviewsLine.number + 1) return refuse("creator_line_misplaced",
    `${file}'s Creator at line ${creatorLine.number} is not directly beneath Reviews at line ${reviewsLine.number}`,
    "move the one Creator line directly beneath Reviews");
  if (creatorLine.text.startsWith(`${CREATOR} run `)) return refuse("creator_legacy_unsupported",
    `${file} names an original-v22 dispatch run; the new consumer has no run lookup`,
    `have the owner write "${CREATOR} declared <provider>" or finish again under a receipt-producing policy; keep the original v22 driver's ending contract`);
  const creator = parseCreator(creatorLine);
  if (creator === undefined) return refuse("creator_line_malformed", `line ${creatorLine.number} of ${file} has invalid creator evidence`,
    `repair "${CREATOR} recorded <canonical receipt>" or have the owner write "${CREATOR} declared <provider>"; no duplicate/extra fields, newlines or NULs, at most 8192 receipt bytes`);
  // Spread the typed receipt into JSON data, the public Context's only payload.
  const fact = creator.evidence === "recorded" ? { ...creator, receipt: { ...creator.receipt } } : { ...creator };
  return { ...given, facts: { ...given.facts, reviewedArtifact: { id }, creator: fact }, sources: [...(given.sources ?? []), source] };
}

function parseCreator(line: Line): Creator | undefined {
  if (line.text[CREATOR.length] !== " ") return undefined;
  const value = line.text.slice(CREATOR.length + 1);
  if (value.startsWith("recorded ")) {
    const receipt = parseEvidence(value.slice("recorded ".length));
    return receipt === undefined ? undefined : { evidence: "recorded", provider: receipt.provider, receipt };
  }
  if (value.startsWith("declared ")) {
    const provider = value.slice("declared ".length);
    if (provider.trim() !== "" && !/[\r\n\0]/.test(provider)) return { evidence: "declared", provider };
  }
  return undefined;
}
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
function duplicate(marker: "reviews" | "creator", lines: readonly Line[], file: string): Refused {
  return refuse(`${marker}_line_duplicate`, `${file} has duplicate ${marker} lines at ${lines.map((line) => line.number).join(", ")}`,
    "keep one relationship and its finishing creator; indent examples, since markers count in fenced blocks too");
}
function refuse(code: string, message: string, remedy: string): Refused {
  return { status: "refused", code, message, remedy };
}
