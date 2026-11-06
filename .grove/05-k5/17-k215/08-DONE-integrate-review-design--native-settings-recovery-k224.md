# native-settings-recovery-k224

**Integrates:** native-settings-recovery-k223


## Goal

Triage the findings of `native-settings-recovery-k223` against the reviewed
design. Apply the valid corrections before `homebrew-delivery-plan-k217`
cuts implementation leaves from it.



## Context

- Read the findings from the review's own committed task file, resolved by
  its handle. That record names the reviewed commits and source locations.
- The producers are `native-settings-recovery-k221` and
  `hooks-installation-writes-k222`. Their running logs hold the decisions.
- The design runs the owner's switch against the real HOME inside finish.
  Grove's review procedure left the design and the model unchanged.

## Done when

- Each finding has a disposition supported by the requirements and source.
- Valid corrections and their affected consumers are reconciled, with
  verification appropriate to each correction.
- Any further design work that needs a producer of its own is represented
  before planning. The remaining design is coherent enough to plan.

## Notes

This leaf owns triage, corrections and the verification after them. Its charter
does not require accepting a finding merely because the review recorded it.

## Decisions (running log)

**The findings (2026-11-06).** The findings are in the review's task file,
committed as `066a121b90b`. That commit changed only `.grove/`. The cited
documents therefore still hold the reviewed text of `2c17bc993d19`, and each
cited line was read again here.

**F1 (2026-11-06).** A real issue in k222's design. Item 1 of "Claude Code's
own writes" reads the metadata of `settings.json`. Item 2 later reads the
digest of the held inode.

An extended attribute or an ACL that another program adds in place between
those two readings is in the digest. The command then drops it from the live
file, and item 6 finds a held copy that equals its record. No report names the
lost attribute.

The repair checks the metadata in the same reading that the record's digest
covers, and reads that inode through the held link. A metadata write after
that reading lands in the held inode and flags it. The model leaves metadata
to the rehearsal, so no definition of the model changes.

**F2 (2026-11-06).** A real issue in k222's design: a contract stated
unclearly. The accepted case claims a bound of two intervals of 30 to 38 ms,
and "a later run has none". A run that stops after the registration, or after
a held link and before a command's rename, leaves the plugin absent.

The next run then runs the commands again. An accepted outside uninstallation
does the same. The duration is an observation on an idle machine, and no
mechanism outside Claude Code bounds it.

**F2 keeps the acceptance and drops the bound (2026-11-06).** The loss needs
another program that renames the file inside an interval. The acceptance
therefore rests on the programs that can rename the file, and the bound was
never its protection.

The repair states the retry semantics and an unbounded interval, and it keeps
the measured duration as evidence only. A timeout that ends a command could
bound the interval, but it would add a partial state of the installation. That
is a redesign and not a repair. The staging seam still gives the owner the
case again when another process replaces the file.

**F1 repairs (2026-11-06).** "Claude Code's own writes" now writes the
settings record first and reads the held inode once through the held link. The
metadata stop uses that same reading. A stop as well as a crash before the
command's rename now leaves a held link that the next run erases. A new seam
adds an attribute and an ACL just before a held link, and after the digest's
reading. The model's list of what it leaves to prose names the reading.

**F2 repairs (2026-11-06).** The accepted case now states its exposure. Each
run of a command exposes one interval. Nothing bounds its duration, and no
durable count limits the runs. The case names the stops and the outside
uninstallation that repeat the commands. The measured duration is evidence on
an idle machine.

"Concurrent writers", "The recovery model", the model's comment, the Homebrew
evidence and the harness views drop the bound. The rehearsal rerun of
`settings-writes.sh` now reads the write method only. A new seam stops the
program after the registration and counts the commands of each run.

**The accepted case loses metadata too (2026-11-06).** A rename by another
program also brings that program's ACL and extended attributes, and the
command's new file drops them. The accepted case now says so. The measured
write method already establishes it.

**Verification (2026-11-06).** The model file changed only in a comment, so
`task model:owner-switch` ran again. It printed 241 passes and no failure in 6
minutes 5 seconds, and its three subjects kept their digests. The evidence
document records that run.

The prose checker reports no hard finding in each edited document. One earlier
finding in the Homebrew evidence, "deny-delete", now carries a departure,
because it names an ACL permission.

The test `reference_navigation` of `grove` passed all 13 cases. It does not
resolve an anchor inside one document: a mutated anchor passed it. A scratch
script resolved the 47 in-page anchors of the owner design instead, and the
same mutation made it report one. The edited `diagrams.json` parses as JSON.

**No new producer (2026-11-06).** Both repairs keep the mechanisms of k221 and
k222. The design stays coherent enough to plan, and the planning leaf now
names k222 and this integration.
