# Proof-only probe — the two-stage arm, simulated

This is the comparator the [preregistration](preregistration.md)'s *Part 2*
specifies: **a defect list a context produces from the draft under the proof
charter, having seen nothing else**. The pilot runs one arm — six stages in
sequence over one book — so without this list every middle stage's contribution
would read as marginal by default and *six beat two* would be unfalsifiable.

`measurement-report-k45` materialised the producer and commits its output here
verbatim, together with the exact inputs supplied and a statement of what was
withheld. The preregistration requires that the probe be committed whether or not
it is convenient.

<a id="producer"></a>
## The producer, and that it was independent

**An independent context, materialised by this session as a separate agent with
its own conversation**, before this session opened any post-draft state of the
book. *Part 2*'s fallback — the report session producing the probe itself, with
every verdict annotated `probe not independent` and no `Keep` licensed — **was
not triggered and does not apply**.

The ordering was deliberate and is checkable from this session's own sequence: the
draft-state book was extracted from the draft commit and handed to the producer
**before** the report session read a single stage record or a single post-draft
diff. Had the producer been unobtainable, the fallback could still have been
executed cleanly from that position.

<a id="inputs"></a>
## The inputs supplied, and only these

The producer was given one directory and told it could read nothing outside it.

| Input | What it was |
|---|---|
| `book/` | the complete text of the book as of the **draft** commit `nurklzvurwwp` — all seven chapter pages, `README.md`, `concept-index.md`, `source-index.md` and `walkthrough.toml`, extracted at book-directory digest `0e9d3f1e7d356c33509327dfe4df584ffa6ca531e531049098cdcf6b923aca14` |
| `source/` | the four corpus files at the digests the preregistration froze — `Cargo.toml`, `src/lib.rs`, `src/jj.rs`, `src/refusal.rs` |
| `taxonomy.md` | the taxonomy table from *Part 1*, **with its *Charter owner* column removed** |
| `PROMPT.txt` | the frozen prompt, issued unchanged |

**The frozen prompt, as issued:**

```text
You are performing the proof stage over a code walkthrough book. Your charter is:
"The final read for errors the book contains or that its drafting missed, and the
last check that the book is whole." You have the book, the source files it
documents, and a taxonomy of reader-facing defect classes.

Do not edit anything. Produce a list of the defects you would fix under that
charter. For each one give: the taxonomy class code; the anchored subject (the
nearest enclosing anchor or heading on the page, or the named fragment, ledger
row or worked-example anchor it concerns); a verbatim citation of the text that
exhibits the defect; and one sentence saying what a reader loses to it.

List every defect you would fix, including small ones. Do not list anything you
would not fix. Do not propose improvements that close no listed defect class.
```

The only text accompanying it was an operational envelope: the three paths above,
the instruction to read nothing outside that directory and no git or jj
repository, the instruction to read every page and every source file in full
before listing anything, the output path, and the output shape (a numbered list
with the four fields under bold labels).

<a id="withheld"></a>
## What was withheld

Everything *Part 2* names, and nothing was withheld beyond it:

- **any state of the book after the draft commit** — the producer's copy is the
  draft state, and the five stage commits were not in its directory;
- **every stage record** — `0-draft.md` through `5-proof.md`;
- **this preregistration**, beyond the taxonomy table and the prompt — including
  the *Charter owner* column, which names the pipeline the probe must not know
  about;
- **the decision rule**, the threshold of three, and the quantities;
- **the six-versus-two question**, and the fact that a pipeline is under
  evaluation at all;
- the repository itself, its tests, its specifications and its `.grove/` tree.

The producer was not told that a proof stage had already run, that four other
stages existed, or that its output would be matched against anything.

<a id="coverage"></a>
## Coverage: the probe is complete rather than partial

*Validity* distinguishes a complete probe from a **partial** one — "one that
terminated early or covers only some pages" — and the distinction changes the
arithmetic, so it is settled here by evidence rather than assumed.

The probe terminated normally and reports a whole-book read: it states that it
verified fragment-by-fragment byte identity against all four source roots, that
the roots expand to gap-free files totalling 698 lines, that no fragment is
orphaned or duplicated, and that every intra-book link and anchor resolves — none
of which is possible from a partial read. Its 21 entries land on eight of the ten
book files: `01-orientation.md`, `03-subprocess-seam.md`, `04-namespace.md`,
`05-scope-and-commit.md`, `06-refusal.md`, `07-what-jj-owns.md`, `README.md` and
`source-index.md`.

**Two files carry no entry: `02-the-gate.md` and `concept-index.md`** (and
`walkthrough.toml`, which is a manifest rather than prose). On the probe's own
account that is a page it read and found nothing to fix on, not a page it did not
reach. The measurement report nevertheless applies *Validity*'s partial-probe
interval to `02-the-gate.md` defensively, as though it were uncovered, and
reports that no verdict moves under it.

<a id="the-list"></a>
## The list, verbatim

Everything below this line is the producer's output as written, unedited, with
**one formatting accommodation and no textual change**: entry 2's citation quotes
a Markdown link out of the book, and an unfenced copy of that link in this
directory resolves against the wrong directory and fails the repository's
link-integrity check. Its citation is therefore carried in a fenced block. The
bytes of the citation are the producer's; only the block delimiters around them
are this session's.

---

# Proof-stage defect list — `jj-workspace` walkthrough

Mechanical checks performed first and passing (recorded here only so the list is
read as exhaustive, not as partial): every literal fragment on every page is
byte-identical to the frozen source; the four source roots expand to complete,
gap-free files (44 + 343 + 81 + 230 = 698 lines); no fragment is orphaned or
duplicated; the source-index fragment table matches every on-page declaration;
every intra-book link and anchor resolves. The defects below are all textual.

1. **Class:** `T2`
   **Anchored subject:** `03-subprocess-seam.md#worked-invocation`, the `rendered("jj", args)` block
   **Citation:** "the message contains spaces, so a shell would split it into six arguments where the child received one"
   **What a reader loses:** The carried message `rate-limit-k3: refuse a request over the burst ceiling` is eight shell words, not six, so a reader who counts — which is exactly what the sentence invites — concludes they have misunderstood the argv/command-line distinction the whole section is teaching.

2. **Class:** `T2`
   **Anchored subject:** `04-namespace.md#the-reservation`
   **Citation:**

   ```text
   "`control_dir` is one of five methods on `Workspace`, and it sits between the two accessors [*The gate*](02-the-gate.md#the-value-and-the-gate) owns and the scope-and-commit operations the next chapter owns."
   ```

   **What a reader loses:** `Workspace` has six public methods (`resolve`, `root`, `main_repo`, `control_dir`, `is_tracked`, `commit`) and eight in total with `fileset` and `relative`, so a reader trying to reconcile "five" with chapter 1's "four operations … plus two accessors" is left believing one of the two counts is a method they have not yet met.

3. **Class:** `T2`
   **Anchored subject:** `04-namespace.md#worked-reservation`, the three-refusal block
   **Citation:** "**Three refusals, in the order the checks run.**"
   **What a reader loses:** The block then shows `nested/deeper` (the separator guard), then `""` (the empty guard), then `repo` (the owned-name guard), but `validated_namespace` runs empty first, separator second, `.`/`..` third and owned-name fourth — so the reader carries a wrong guard order into `#the-validation`, whose heading is "Four refusals, in the order they run" and whose entire argument is that the order is load-bearing.

4. **Class:** `T2`
   **Anchored subject:** `06-refusal.md#the-namespaces-two`
   **Citation:** "*it refers to the workspace itself*"
   **What a reader loses:** The frozen source's third reason string is `"it names a directory other than itself"`, quoted correctly in chapter 4 and analysed there at length ("*it names a directory other than itself* reads exactly for the parent and only loosely for `.`"), so a reader who trusts this chapter's list is holding a message text the crate never produces and cannot match it to chapter 4's argument about it.

5. **Class:** `T2`
   **Anchored subject:** `06-refusal.md#the-cause-chain`
   **Citation:** "A consumer walking `source()` from a failed commit gets two links and stops; the third and fourth causes exist only as bytes in a string."
   **What a reader loses:** `source()` on a `CommitNotRecorded` yields the `CommandFailed` refusal and then `None` — one link, exactly as the same chapter measured at `#worked-refusal` ("The `source()` chain is exactly one link long … `.source() -> None`") — so the reader is handed two contradictory depths for the one structure the chapter says it measured rather than described, and the "third and fourth causes" mis-numbers jj's own `Caused by:` lines.

6. **Class:** `T2`
   **Anchored subject:** `06-refusal.md#the-opaque-type`
   **Citation:** "Its parts are declared here and read in the sections that follow, in the order the file writes them."
   **What a reader loses:** `«refusal-imports»` is the file's second block (lines 14–18) but is read last, at `#no-error-crate`, after lines 215–230 — so a reader using the promise to navigate `refusal.rs` alongside the chapter loses the file position of the one block the chapter uses to close the empty-dependency-table argument.

7. **Class:** `T2`
   **Anchored subject:** `01-orientation.md#what-it-declines`
   **Citation:** "That is the whole crate: four operations, a value, and one error type, over roughly seven hundred lines with no dependencies."
   **What a reader loses:** The crate has three public types — `Workspace`, `Commit` and `Refusal` — as `#public-surface` states two sections later ("There are three public types."), so the book's opening inventory silently drops `Commit`, the return value the carried operation produces on the same page.

8. **Class:** `T2`
   **Anchored subject:** `05-scope-and-commit.md#the-path-algebra`, fragment `«relative-strip-or-canonical-parent»`
   **Citation:** "The crate's own suite reaches this fallback only through `a_deletion_is_committable_after_the_path_is_gone`, which passes a *relative* path and therefore takes the textual branch"
   **What a reader loses:** A test that takes the textual branch does not reach the fallback at all, so the sentence asserts and denies coverage of the canonicalising branch in one breath, and the reader cannot tell whether the branch measured just above is tested or untested.

9. **Class:** `T2`
   **Anchored subject:** `06-refusal.md#worked-refusal`, the consumer's-half block
   **Citation:** "`grove-loop` re-exports `Commit` and `Workspace` from this crate and not `Refusal` (`crates/grove-loop/src/lib.rs:81`), and its three call sites each do one of the three things an opaque error allows."
   **What a reader loses:** The three sites listed are `session_config.rs:194`, `crates/grove-llm/src/cli.rs:452` and `session_config.rs:354` — the second is in `grove-llm`, not `grove-loop` — so a reader checking the boundary claim against the paths given finds the possessive false and cannot tell which crate the book means by "the consumer".

10. **Class:** `T2`
    **Anchored subject:** `06-refusal.md#no-remedy-of-its-own`
    **Citation:** "The first shape is in this file ten times. The second shape is in it zero times."
    **What a reader loses:** The "first shape" is exemplified as a statement of jj's *offer* ("*Make the tree jj-enabled and rerun: `jj git init --colocate`*"), and the same chapter later states that `CommandFailed`'s arm "has no remedy paragraph" and that `OutputNotText` "says what happened and stops", so the count of ten cannot be verified against the ten arms the reader is about to be shown — and chapter 7 repeats the figure ("That shape appears ten times and its opposite zero times") on the strength of it.

11. **Class:** `D5`
    **Anchored subject:** `source-index.md#early-uses`, ledger row `Commit`
    **Citation:** "| `Commit` | `01-orientation.md#commit-tour` | `no-transactions` | What a taken commit returns: a change id rather than a commit id, because a change id still names the work after a rewrite. | `explained` |"
    **What a reader loses:** The minimum local statement is discharged at `#public-surface` ("`Commit` is what a taken commit returns. It carries a change id rather than a commit id…"), not at `#commit-tour`, where the name appears only as `Commit { change_id: … }` in a trace — so a reader who follows the ledger's anchor to check the debt finds nothing there.

12. **Class:** `D1`
    **Anchored subject:** `source-index.md#early-uses`, ledger row `control_dir`
    **Citation:** "| `control_dir` | `01-orientation.md#commit-tour` | `no-consumer-vocabulary` | A namespace is one plain directory name the consumer supplies; …"
    **What a reader loses:** `control_dir` is first used two sections earlier, at `#what-it-declines` ("Grove calls `control_dir(\"grove\")` once per loop and keeps a lock file and a signal file in the directory it gets back"), and its statement is discharged at `#public-surface`, so the row covers neither the actual first use nor the place the statement is made, and a reader meeting the name in the opening consumer's-half block has no local account of it.

13. **Class:** `D1`
    **Anchored subject:** `source-index.md#early-uses`, ledger row `is_tracked`
    **Citation:** "| `is_tracked` | `01-orientation.md#the-six-refusals` | `no-transactions` | The one probe whose answer depends on the working copy, and so the one that lets jj snapshot before answering. | `explained` |"
    **What a reader loses:** `is_tracked` is used two sections earlier at `#crate-thesis` — "`is_tracked` deliberately does not, because its answer *does* depend on the working copy" — and again at `#public-surface`, so those uses are covered by no row, and the reader meeting the crate's most carefully argued asymmetry has no statement of what the probe is.

14. **Class:** `D2`
    **Anchored subject:** `01-orientation.md#public-surface`
    **Citation:** "There are four operations, all reached through a `Workspace`, plus two accessors that answer from fields already held. `resolve` walks up from a path to the closest ancestor holding `.jj/` and is the only constructor. `control_dir` takes a namespace and returns a directory reserved for it. `is_tracked` asks whether the workspace holds a path. `commit` takes a path-scoped commit and returns a `Commit`. `root` and `main_repo` are getters."
    **What a reader loses:** This section sits before the chapter's worked example at `#commit-tour` and primarily enumerates three public types and six public operations, so the reader meets the crate as a list of names to memorise before meeting a single one of them doing anything.

15. **Class:** `C1`
    **Anchored subject:** `README.md`, the second paragraph
    **Citation:** "The book covers all of it: the manifest, the gate, the child-process seam, the namespace, the commit algebra, and the ten refusals."
    **What a reader loses:** "Refusal" is the book's spine term for the six chapter-level subtractions ("the last chapter turns the six refusals into a test"), and here the same word is used for chapter 6's ten `Kind` variants without marking the shift, so the reader begins the book with two incompatible counts of the thing the book is organised around.

16. **Class:** `C2`
    **Anchored subject:** `04-namespace.md#the-reserved-list`
    **Citation:** "only the second is achievable without a interface the foreign system does not offer"
    **What a reader loses:** The article is wrong ("a interface"), which stalls the reader inside the sentence that states the chapter's general shape.

17. **Class:** `C2`
    **Anchored subject:** `05-scope-and-commit.md#the-path-algebra`, fragment `«relative-strip-or-canonical-parent»`
    **Citation:** "The exact boundary that buys was measured against jj 0.44.0 rather than reasoned about, using a workspace reached through a symlink and a path that had been deleted:"
    **What a reader loses:** "that buys" has no object, so the sentence introducing the chapter's only two-case measurement does not say what was bought, and the reader must infer the subject of the measurement from the block below it.

18. **Class:** `C2`
    **Anchored subject:** `05-scope-and-commit.md#worked-commit`
    **Citation:** "**The second ending: a caller that names no paths at all.**"
    **What a reader loses:** Nothing on the page is labelled a first ending — chapters 2 and 3 both label theirs explicitly ("**First ending: the tree as it is.**", "**First ending: jj ran and succeeded.**") — so the reader scrolls back looking for a heading that does not exist.

19. **Class:** `A1`
    **Anchored subject:** `01-orientation.md#commit-tour`, the paragraph after the trace
    **Citation:** "Five things in that trace are the whole book. The walk is the **filesystem's**, not jj's, so nothing in the environment can redirect it — *The gate*. The second spawn passes `--ignore-working-copy` and the first does not, and the difference is not an oversight — *Scope and commit*. Both spawns are built at one seam that fixes the working directory and strips four environment variables — *The subprocess seam*. The string `\"grove\"` arrives from outside and the crate never looks at what it means — *The namespace it will not name*. And every arrow in that trace has a second ending in which a `Refusal` is returned instead — *Refusal*."
    **What a reader loses:** This is a five-row mapping from a line of the trace to the chapter that owns it — the book's own table of contents against the carried operation — carried as a run-on paragraph appended to an unrelated sentence about argument lists, so the reader must hold five pairings in their head where a list or a two-column table would show them.

20. **Class:** `A2`
    **Anchored subject:** `07-what-jj-owns.md#applying-the-test`, the six-row verdict table
    **Citation:** "## The test applied to all six\n\n| # | Guarantee the consumer needs | Where the guarantee comes from | Strong enough? | Checkable? | Verdict |"
    **What a reader loses:** The chapter's decisive table follows its heading with no adjacent statement of its role — nothing says the two clauses of the test are the two middle columns, or that the row-4 verdict is the finding the rest of the chapter is about — so the reader meets six verdicts before being told how to read one.

21. **Class:** `C5`
    **Anchored subject:** `05-scope-and-commit.md#the-commit`, fragment `«commit-return»`
    **Citation:** "This second call's refusal is **not** wrapped, and that is the other half of the distinction. The `?` propagates the seam's refusal as it stands, so a consumer that receives `CommandFailed` from `jj log` is receiving a refusal about a command, while `CommitNotRecorded` is a refusal about state. A consumer cannot match on either — the type is opaque — but it can print them, and the two messages do not say the same thing."
    **What a reader loses:** That is the whole of what precedes the fragment and it is about the *previous* fragment, so `«commit-return»` arrives with no introducing paragraph at all — nothing says what these five lines are, why they are here, or what to look at in them — and the reader meets the crate's only success-path allocation as unannounced code.
