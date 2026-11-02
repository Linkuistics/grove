---
name: decision-records
description: Numbered decision records in the MADR layout — one file per decision under docs/decisions/, a four-digit sequence number, a status line, and supersession instead of edits. Use when writing, revising, citing, or reviewing ADRs / decision records.
---

# Decision records

A decision record captures one decision, its context and its consequences.
The records form a log: each record keeps the decision as it was taken, and a
later decision supersedes it instead of an edit.

## Where a record goes

- Write each record to `docs/decisions/NNNN-<title>.md`. `NNNN` is the next
  free four-digit number, starting at `0001`.
- Never renumber a record, and never reuse a number.

## The template

```markdown
# NNNN. <Title>

Status: Accepted (YYYY-MM-DD)

## Context

<The forces that make the decision necessary.>

## Decision

<The decision, in one or two sentences.>

## Considered options

- <The chosen option>
- <A rejected option, and why it lost>

## Consequences

<What becomes easier and what becomes harder.>
```

## Changing a decision

Do not edit the body of an accepted record. Write a new record, set its status
to "Accepted", and set the old record's status to "Superseded by NNNN".
