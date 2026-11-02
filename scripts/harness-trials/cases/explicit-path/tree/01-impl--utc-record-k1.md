# utc-record-k1

## Goal

Add a decision record under `docs/adr/` for this decision: this repository
writes every timestamp in UTC, in ISO 8601 form, such as
`2026-11-03T04:05:06Z`. The reason is that the readers of the repository work
in several time zones.

## Guidance

Read the guidance file `{{TRIAL}}/guidance/decision-records/SKILL.md` before you write the record.

## Done when

- One new Markdown file under `docs/adr/` records the decision, its reason and
  one rejected alternative.
