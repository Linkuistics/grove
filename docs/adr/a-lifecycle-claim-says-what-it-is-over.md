# A lifecycle claim says what it is over

An `SY-` claim states its own quantifier rather than borrowing one from the
words it happens to use. Two consequences bind.

**A claim binds the world only where it says so, and where it does it names what
bounds the world there.** The semantic contract's §*Actions* puts `crash`,
`hand-edit`, `foreign-write`, `topology-change` and `confirm` in an Environment
group whose guard column reads *none* — Grove can neither refuse, prevent nor
undo one. So a claim about **what an action leaves** is about what a *Grove*
action leaves, and a claim about **which states are reachable** is about the
states Grove's own admitted actions reach. A claim that means to bind the world
says so and names the `EN-` row or §*States* property that makes the wider
reading checkable, as `SY-05.b` does at `EN-14`.

**A *lifecycle transition* is a step that advances the grove's own lifecycle
stage — not a member of §*Actions*' Lifecycle group.** The group is where the
guards live; the transitions are `initialise-root` and the append that completes
an interrupted scaffold, `allocate-finish-leaf`, `recover`, and the driver's own
advance of the task tree between sessions. §*Actions* also gained the
`validate-config` row the group had always been short: `SY-02` ordered the layout
proof *before configuration validation* and `SY-04.b` required one, while the
table named only `layout-preflight` and put one action under two jobs.

## The trade-off this settles

**The catalogue supplied a word and no definition, and two independently built
model families filled the silence with sets that have no member in common.**
`models/system/lifecycle.als` read `SY-04` over the seven-member Lifecycle group
and witnessed all seven of it, one *alone in an iteration* each;
`models/system/lifecycle.qnt` read it over the stage-changing steps, which that
group contains **none** of. Both were green, and neither could be caught by its
own suite — the same failure
[`a-refusal-leaves-nothing-standing`](a-refusal-leaves-nothing-standing.md)
records for `Refused` and `Blocked`, in an obligation no enumeration had flagged
as underdetermined.

The same silence cost two claims their literal truth. `SY-04.b`'s *an invalid
configuration leaves the working tree byte-identical* is false of a hand edit,
and `SY-13`'s *from any stable state* ranges over `Legacy`, `Foreign` and
`Malformed` — each reached by a hand edit, left by a hand edit, and therefore a
**sink**, so both of that claim's obligations were false rather than weak. Both
families narrowed both, each declared the narrowing, and neither could repair the
text: the catalogue is their shared subject and the independence protocol held.
A narrowing declared twice by two independent readers is the contract being
wrong, not the models being cautious.

**The referee for the transition reading is the *so that* clause and the shipped
product, not a preference between columns.** `SY-04` says *so an invalid
configuration leaves the working tree byte-identical*, and a gate in front of
`close-epoch` or `release-lease` buys that nothing — neither writes a tree. A
claim whose justification reaches only part of its own quantifier is stated too
wide. And the loop
(`crates/grove-loop/src/loop_driver.rs`) reads the configuration twice an
iteration — once before the tree transition and once before the launch — which is
[`complete-session-configuration`](complete-session-configuration.md)'s
*validated in full — before every tree mutation and again before every launch*.
That record needed no change; it had already said what `SY-04.b` did not.

## The alternatives, each rejected on a stated cost

- **Read *transition* as §*Actions*' Lifecycle group.** This is not a straw man:
  it is what one column did, it is what the word suggests, and it is arguably the
  more natural reading of a table whose section is called *Lifecycle*. Rejected
  because it gates `release-lease` and `close-epoch` on a configuration validity
  that buys their consequence nothing, and because it made a driver holding a
  lease under an invalid configuration unable to release it — a dead end that
  dissolves under the stage reading. **Reopen** if a Lifecycle-group action ever
  writes the task tree, since the *so that* clause would then reach it.

- **Admit process death as an exit** — the other repair on offer for that dead
  end. Rejected because `crash` is the world's (§*Actions*; `CONTEXT.md`'s
  *Admitted action*), so admitting it changes what **every** reachability claim
  quantifies over, and a sweep in which the loop may always die finds no dead end
  anywhere. That is the same argument this catalogue already makes for refusing
  to count a hand edit as an exit. **Reopen** only if Grove gains an exit it takes
  deliberately rather than one the kernel performs on it.

- **A silent default — *unless a claim says otherwise, it is about Grove*.**
  Rejected because it makes every under-specified claim quietly narrower and
  unfalsifiable in exactly the direction nobody is looking, which is the hazard
  §*Actions*' existing rule already answers: an obligation ranges over what its
  scope admits **and says so in its own text**. The rule here is about how a
  claim is worded, and each claim carries its own qualification.

- **A blanket *`SY-` claims are about Grove only*.** Rejected because it is
  false: `SY-05.b` binds the world deliberately and rests on `EN-14`. What
  distinguishes the sound case is that it names what bounds the world; that
  naming is the rule rather than the exclusion.

- **Fold the world-reached refusal states into `SY-13`'s terminal
  dispositions.** Rejected by the catalogue before this record existed, and
  rightly — *it would let the claim be satisfied by a tree nobody can act on* —
  but declining it left the claim false rather than repairing it. Narrowing the
  quantifier is what repairs it, and it is sound only with its companion, so
  *Grove never manufactures one of the others* is `SY-13.a`'s first conjunct
  rather than an assumption, with a control that kills it.

## What checks it

Every clause above is a command rather than a paragraph, which is the point of
recording it here rather than only in the catalogue.

- `models/system/lifecycle.qnt`: `mutant_config_licence` restores the
  recorded-verdict reading of `SY-04.b` and kills it, with the weak conjunct
  asserted green beside it; `mutant_grove_manufactures_legacy` makes
  `initialise-root` write a `Legacy` root and kills `SY-13.a`'s first conjunct;
  `mutant_block_named_by_all` runs `SY-14.b`'s naming sweep over the literal
  admitted set and kills it, with `SY-14.a` green beside it;
  `mutant_literal_sy13` restores `SY-13`'s literal quantifier and kills it.
- `models/system/lifecycle.als`: `SY_04a` is stated over the trace and over the
  transitions, carrying a third conjunct that exists only to catch a frame hole —
  *nothing but a counted transition or the iteration boundary moves the flag*. It
  found four in one sitting, including a predicate that framed the launched
  session's fields by hand.

**The models under `## What checks it` have been retired, and nothing checks the
claim mechanically now.** `models/system/lifecycle.qnt`, `models/system/lifecycle.als`
and their controls were deleted with the campaign's apparatus
(`delete-formal-models-k29`), as was the catalogue they were written from,
`docs/specs/semantic-contract.md`. This record is now the statement of the rule
rather than a pointer to its enforcement: a lifecycle claim still has to say what
it is stated over, and a reviewer applies it by reading. The decision above survived the instrument that found it, which is the outcome
that campaign was run to test, and `docs/formalism-findings.md` keeps the record
of how it was found.
