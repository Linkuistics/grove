# root-charter-conversion-k323

## Goal

Convert the root brief `.grove/_BRIEF.md` to the controlled-English writing
rules as its own change, before the finish handoff extends it.

## Context

- The technical-communication skill and its conversion procedure.
- The parent brief, Notes: "The root brief does not yet follow the writing
  rules, so convert it first as its own change."

## Done when

- The checker reports no hard finding for `.grove/_BRIEF.md`, or each
  departure carries a marked reason.
- The conversion changes no meaning. It adds no fact and loses no
  information, hedge or caveat. A reviewer can compare it section by section.
- The commit holds only this conversion.

## Decisions (running log)

- The checker reported no hard finding before the conversion. The earlier
  conversion in pack-membership-k171 and later careful edits kept to the
  countable rules. The conversion therefore treats the rules that the checker
  cannot count: one idea per sentence, clear pronouns, verbs for actions, and
  conditions first.
- One fresh-context reviewer compared the old and new text for meaning. It
  reported 15 findings. Each finding was valid or cost nothing to correct, and
  the rewrite keeps the old wording closer at each place. The ambiguous
  sentence "Every policy must not have to remember a separate helper" now reads
  "Policies must not have to remember a separate helper", which has the reading
  "no policy".
- The commit holds the conversion and the retirement rename of this leaf only.
