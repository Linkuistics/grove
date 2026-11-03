# confinement-withdrawal-k143

**Reviews:** confinement-withdrawal-k142

## Goal

Adversarially review the confinement withdrawal design of
confinement-withdrawal-k142 before ravel-confinement-removal-k145 and
grove-confinement-removal-k146 implement it.

## Context

- The design commits of confinement-withdrawal-k142, in this repository and on
  the Ravel bookmark `skill-directory-delivery` in
  `Ravel.skill-directory-delivery`. Follow that repository's VCS discipline.
- The decision log of confinement-withdrawal-k142.

The human decided the withdrawal on 2026-11-03. The decision itself is
closed. Review how the design states it and what the design leaves out.

## Done when

- Each finding names its file, its line and a smallest useful correction.

## Notes

Look in particular for these failures:

- A dependent of confinement that the design does not list, in either
  repository. Examples are a test, a script, a book excerpt, a fixture, a
  floor case and an acceptance row.
- A guarantee that the design silently drops, such as a protection of the
  owner's configuration or of a skill snapshot.
- A remaining promise in either product that a sandbox denies an access.
- A refusal or a remedy for a removed flag that the design does not specify.
