# Human read of the comparison samples

This record holds the short human read that `sample-reading-k11` held. The
session stated the frame on 2026-11-21, and the human reported on 2026-11-25.
The read is seam 3 of the documentation-guidance grove. The seam-2 report in
[README.md](README.md) named the six samples. This record sits beside that
report and does not change it.

The read follows `reader-test.md` of the requirements pack at revision
`636b36242410`. In each pair, the human reads as a labelled stand-in for the
reader that the case's contract declares.

## How the read was set up

No sample's contract declares a reader checkpoint for this read. Each value
below therefore comes from the reader and format parts of the case's contract,
and from the seam-2 report's question for the pair.

### Pair A: the EditorConfig reference page

- **Samples, in reading order:** S2 (revised), then S1 (current).
- **Stand-in for:** a developer who is about to write an EditorConfig plugin.
  They know INI files and shell globs. EditorConfig is new to them.
- **Reader tasks (find and use):** find one key entry and one glob-form entry.
  Use each entry alone, and say whether the answer needed text outside the
  entry. Before the lookups, the human says how well they know EditorConfig.
- **Entry surface:** the raw file, at the entry.
- **Viewing environment:** the VS Code editor. The contract declares the raw
  file.

### Pair B: the AIL page on what a signature leaves unsaid

- **Samples, in reading order:** S3 (current), then S4 (revised).
- **Stand-in for:** a language designer who is new to API binding. The human
  knows the domain, so they read as the reviewer that section 8 of the plan
  describes.
- **Reader tasks (understand):** mark each passage that helps (+) or hinders
  (−). After the first page, name each obligation that the C header leaves
  unstated, with the failure that it prevents.
- **Entry surface:** the rendered page, from its top.
- **Viewing environment:** the VS Code Markdown preview, with the reading
  folder open.

### Pair C: the Courier design exchange

- **Samples, in reading order:** S6 (revised), then S5 (current, turns 1 and 2).
- **Stand-in for:** the Courier owner who asked for the change. They know
  timeouts. Segments, delivery receipts, the receipt delay and idempotency keys
  are new to them.
- **Reader tasks (understand and use):** after each transcript, state the
  decision, the option that you would choose and its cost, in your own words.
  Say whether the session told you enough to choose.
- **Entry surface:** the transcript. The Human parts are scripted picks from a
  model proxy. The frame told the human that these parts are not their words.
  S5 stops before `## Turn 3`.
- **Viewing environment:** the frame named none. The human opened the reading
  folder in VS Code before the read.

### Reading copies

The stored outputs hold only the files that a run changed. Sixteen distinct
links in each D2 sample point to rule files that the run did not change. Those
links break where the samples are stored. The read therefore uses copies. Each
copy lays one arm's stored files over the case's fixture, without the fixture's
`.grove/` and `tools/` directories.

In the copies, every link of S1 to S4 resolves except one. Both D2 samples link
to `docs/design/2026-09-22-interface-language.md`, which the fixture never
held. The transcripts hold no links. Each copy has the digest of its stored
sample:

| Sample | Stored file under `outputs/` | SHA-256 |
| --- | --- | --- |
| S1 | `d1-draft/current/files/docs/editorconfig/guide/reference.md` | `5163f8ed44ab8681fe76df70d7227f50a92f0755bfdcb162ca2e4a94baae6056` |
| S2 | `d1-draft/revised/files/docs/editorconfig/guide/reference.md` | `f180f147ba244e177163f6ac2633578a8d668d5b0b1d49434f09ffdb0684b7c7` |
| S3 | `d2-copy-edit/current/files/semantic/specification/interface-language/explanation/unsaid.md` | `3c34c3ca563f425d167c4454394ffaf37f440806f4bbc0198eac854069bebcc5` |
| S4 | `d2-copy-edit/revised/files/semantic/specification/interface-language/explanation/unsaid.md` | `f60bbd3924b88a3060d3f37f10027417fa05ab30c0c79f697956d524f48e2d43` |
| S5 | `e1-design-exchange/current/transcript.md` | `f3dab7a7b2ad6baf6629b5a948433406428c80e2d08ec44703d97afe531d7df5` |
| S6 | `e1-design-exchange/revised/transcript.md` | `783ef3974f50537af51584b283669cc35fd72af9ceef13c272719d4357e81b85` |

### Figures

No sample embeds an image or links to one. Neither transcript sends the reader
to a page that a server shows. The samples carry their figures in the page's
own text:

- S1 has one table of keys and values. S2 has no table.
- S3 and S4 each have one C code block and one table of familiar terms.
- S6 has a text timeline sketch, a TOML block and three tables. S5 has none.

Each code block closes, and each table has its delimiter row. The editor shows
these figures as text, and the preview renders them. No figure needs a step to
load.

### Order

In each pair, the arm that the human reads second gains from the first reading.
The order gives that gain to the current arm in two of the three pairs.

### What the human heard before the read

The current arm of E1 ran twice. The first attempt hung on a preview server
that the design skill's visual step started.
[`failed/e1-current-attempt-1.md`](failed/e1-current-attempt-1.md) records it.
S5 comes from the second attempt.

The frame left these points unsaid:

- **The repair in S6.** Turn 3 of S6 holds a message that the first runner
  lost. The transcript restores the message and labels the repair at the top
  of the turn.
- **Why S5 stops at turn 2.** Turns 3 to 10 of the current exchange are a proxy
  loop.
- **The seam-2 report's notes on what to look for.** These notes stayed out of
  the tasks, so that they did not steer what the human noticed.

## What the human reported

The human sent this report in one message on 2026-11-25. It answers the first
step of pair A and gives a judgement on the pairs. It holds no answer to the
reader tasks.

<!-- tc: off because the human's report is quoted verbatim -->

> I don't know EditorConfig, but the pattern is familiar. What am I evaluating - I think the revised version is more structured and complete and easy enough to read, however the original was not bad.
>
> I read most of the pairs.
>
> I do like the revised version of the rest of the prose.

<!-- tc: on -->

### What the report shows

Each item below holds one class of evidence. No item comes from the seam-2
model proxy.

- **Familiarity.** The human does not know EditorConfig. The reader that pair A
  declares is also new to it. The human also said that "the pattern is
  familiar", and the report does not say which pattern.
- **Human acceptance.** The human judged the revised version "more structured
  and complete and easy enough to read", and the original "not bad". The
  judgement follows the EditorConfig answer, so the session reads it as the
  judgement on pair A. The human likes "the revised version of the rest of the
  prose". The session reads that phrase as pairs B and C.
- **Task performance.** The human reported no task answer. Task performance
  stays unobserved for all six samples. The acceptance above is not evidence of
  it.
- **Coverage.** The human "read most of the pairs". The report names no sample,
  location or entry surface. The samples and parts that the human read stay
  unknown.
- **Difficulties.** The human located no difficulty in a sample.
- **The purpose of the read.** The human asked what the read evaluates. The
  question concerns the frame, not a sample. The session answered after the
  report. Its answer said that the read asks whether each revised version
  serves its reader better, worse or the same. It also said that the find,
  understand and use tasks make that question concrete.

One human read the samples. The record therefore holds no agreement between
independent readers.

### How the read closed

The session asked whether to close the read now or to add a short task check.
It also asked the human to say if it took a sentence as being about the wrong
pair. The human replied "close now" and did not dispute either reading. Task
performance therefore stays unobserved.

### Corrections

The human named no correction, so the read routes none under the bounded
correction rule. No sample needs a next read.
