# Wording micro-test — does the guaranteed core get a session to read the skill?

Design validation for [the skill delivers the
methodology](../adr/skill-delivers-the-methodology.md), whose residue is paid for
entirely by trigger strength. Run once, before any of the rewrite shipped, so the
design's answer to the **first** observed failure could still be changed for
free. Nothing re-runs it; it is not a gate.

The house rule it discharges is
`plugins/linkuistics/skills/authoring-conventions/SKILL.md`, *Test the wording,
cheaply*.

## What was measured

**Observable:** did the session open the skill **and its kind's reference file**
*before acting* — before touching the task tree, the working tree, `git`, or a
`grove-llm` verb.

Three secondary readings, all recorded because two of them turn out to be at
ceiling and would otherwise be mistaken for the result:

| reading | what it tells you |
|---|---|
| opened the skill before acting | near ceiling in every arm — **does not discriminate** |
| opened the kind's reference file before acting | the discriminator |
| emitted the receipt token | harness-independent proof the reference file was read *at some point*; the token appears nowhere but inside that file |

A tool call that bundled a skill read with a working-tree action was scored as
**acting** — the session had already reached for the work. The rule was applied
identically to every arm and both vendors.

## Arms

- **Control** — the pre-mandate launcher, recovered from VCS rather than
  reconstructed: `content/prompts/continue.md` at `usqyyptkwrnv-` (the parent of
  *impl: deliver the composed mandate and reduce the launcher to framing*), plus
  the two driver-authored fact sentences `mandate_prompt`/`stated_vcs` appended
  to it. 1,168 bytes — the spec's "~1.1 kB", and the arm the field failure was
  actually measured on.
- **Variant** — the designed core, all five elements of §*Trigger strength* (2).
  3,001 bytes.
- **Ablated** — the variant minus the rationalization table and the *this prompt
  is not a summary* clause; the imperative, the ordering clause and the absolute
  paths only. 1,969 bytes. **A supplementary third arm, beyond the two the leaf
  required**, run because the leaf asks for the reason the apparatus is kept to
  be part of the recorded result, and an ablation is the cheapest way to make
  that reason evidence instead of assertion.

## Targets

A wording result does not transfer across targets, so both of this workstream's
vendors were run, each on the session kind it actually carries in
`~/.config/grove/config.kdl`.

| target | command | kind under test | reference file |
|---|---|---|---|
| claude / opus, effort medium | `claude -p --model opus --effort medium` | `impl` | `references/impl.md` |
| codex-cli 0.147.0, profile `sol-high` (gpt-5.6-sol, reasoning high) | `codex exec --profile sol-high` | `review-impl` | `references/review.md` |

Five fresh-context repetitions per arm per target: **30 sessions**, plus one
pilot excluded from the counts because it was run to validate the instrument.

## Result

| arm | target | skill before acting | **reference file before acting** | read the reference file at all |
|---|---|---|---|---|
| control | claude | 4/5 | **0/5** | 5/5 |
| control | codex | 5/5 | **0/5** | 5/5 |
| variant | claude | 5/5 | **5/5** | 5/5 |
| variant | codex | 5/5 | **5/5** | 5/5 |
| ablated | claude | 5/5 | **5/5** | 5/5 |
| ablated | codex | 5/5 | **5/5** | 5/5 |

**Variant beats control: 10/10 against 0/10, on both targets.** The design's
answer to the first observed failure holds, and the tree proceeds.

### The failure the control reproduces is ordering, not ignorance

This is the finding that changes how the rest of the design should be read, and
it was not what the arms were designed to separate.

The control did **not** fail to open the skill — it opened `SKILL.md` in 9 of 10
sessions, and every control session in both arms eventually read its kind's
reference file (receipt 10/10). What it failed to do, in **every** session, was
read the *procedure* before starting work: read the conditions, begin
bootstrapping, and pick the kind's reference file up later — four times bundled
into the same shell call as a `grove-llm` verb or a `.grove/` read.

So "sessions demonstrably did not read it" is, on this instrument, more precisely
*sessions read the routing file and acted before reaching the procedure*. The
variant's first element — one imperative naming **both** targets, so the session
performs no selection and has nothing to defer — is what closes it, and the
element the spec predicted would carry the weight is the element that does.

### Three elements are established; two are not

The ablated arm scores 10/10 — identical to the full variant. Everything measured
here is carried by the imperative, the ordering clause with its enumeration, and
the absolute provisioned paths. The rationalization table and the *not a summary*
clause added nothing detectable.

**Decision: the core ships ablated**, at 1,969 bytes. The house no-op test says a
sentence that does not change behaviour against the default goes, and the
too-late test admits the load instruction rather than persuasion prose that
supports it.

**This is a stronger claim than the evidence, and the gap is stated rather than
hidden.** The variant is at ceiling, so the ablation can only detect a *large*
negative effect; a marginal contribution from the two cut elements is
unfalsifiable here. The spec's classification argument is untouched by this
result — the failure is a discipline failure, and prohibition plus rationalization
remains its prescribed form — so what is recorded is not "the apparatus is idle"
but "the apparatus is unmeasured, and unmeasured prose does not ride the one
channel a session cannot skip."

**Reopen condition.** Reinstate the rationalization table, the *not a summary*
clause, or both, if the human-watched acceptance run (`delivery-acceptance-k11`)
shows sessions treating the core as an abridged methodology and working from it,
or reaching the procedure late under the real corpus. That run is the first
setting in which the two cut elements would face pressure this instrument never
applied.

## Limitations

Named rather than argued away; each is a reason a future disagreement should land
on the instrument and not on the counts.

1. **Headless proxy.** Grove launches interactive TTY sessions; these ran under
   `claude -p` and `codex exec`, the only automatable instrument. Accepted
   deliberately before the run.
2. **A 25-line stand-in skill.** The leaf and spec both specify this — the real
   corpus is not needed to test whether a session opens one — but it makes
   reading nearly free, which is the likeliest source of the ceiling on the
   coarse reading.
3. **The skill was named `grovekit`, not `grove`**, and the substitution was
   applied to both arms symmetrically. The real `grove` skill is already
   provisioned globally in `~/.claude/skills` and `~/.codex/skills`, so a
   stand-in of the same name would have made "opened the skill" ambiguous between
   a 25-line fixture and a 49 KiB incumbent.
4. **The stand-in was provisioned project-level** (`<worktree>/.claude/skills/`,
   `<worktree>/.codex/skills/`) rather than into the harness's global root, so no
   part of the run mutated the user's home. Discovery was verified for both
   vendors before the run.
5. **The sessions' global skill environment was the user's real one**, which
   includes `using-superpowers` — a skill whose stated purpose is to force skill
   invocation before any response, and which codex sessions were observed reading
   first. This raises both arms equally, and it is the same environment the field
   failure was measured in, so it is part of the target rather than a confound to
   remove.
6. **Kind differs by vendor** (`impl` on claude, `review-impl` on codex), because
   each vendor was run on the population it actually carries. Comparison is
   within-vendor; no cross-vendor claim is made.
7. **n = 5 per cell.** The margin (10/10 against 0/10) is wide enough that this
   is not the binding limitation, but no smaller effect is resolvable.

## The winning wording

What `guaranteed-core-k9` lifts. Three parts, in the session's own timeline
order; `<kind>`, `<ref>`, `<locations>`, `<handle>` and `<stated-vcs>` are driver
substitutions, and part 3 is the embedded corpus's own signal file inlined
verbatim.

### 1. The load instruction

```markdown
**Load the `grove` skill now, and read its `<ref>`.** Your kind is
`<kind>`; Grove resolved that before this session existed, so there is no
selection for you to make — those two files, in that order.

Do this **first**, before anything else you might reach for: before reading the
task file, before running any `grove-llm` verb, before looking at `.grove/`,
before inspecting the working tree, and before answering a question.

The skill was provisioned for you at <locations>. If your harness offers no
skill-loading affordance, read `SKILL.md` and `<ref>` under one of those
directories directly, by path.
```

`<locations>` is the list of provisioned directories by absolute path, computed
by the same registry that wrote them.

### 2. The runtime facts

```markdown
Grove mandate: the leaf selected for this session is `<handle>`.

Version control: <stated-vcs>.
```

Values only. The normative consequences that today's prose carries — *this
selection is authoritative, do not call `grove-llm pick`*, and *do not probe for
the version control, and disregard a harness banner that disagrees* — leave the
core under the spec's closed fact test and become the skill's obligation. Both
were carried by the stand-in `SKILL.md` in this run.

### 3. The session ending

`content/`'s signal file, inlined byte-exact. Not reproduced here: one source,
and quoting it in a second document is the drift this design exists to avoid.

## Reproducing

The fixture, both original arms, the ablation, the runner and the scorer are not
committed — the leaf makes this an experiment read once by a human, not a suite.
The control is recoverable from VCS as described above; everything else is
reproduced from *The winning wording* and the arm descriptions.
