# legacy-driver-transition-k50


## Goal
Prepare and test safe preservation of the original installed 22.1.0 driver
through k13's mid-grove upgrade, without changing the live installation here.

## Context

The parent's exact complete-keg, cleanup, canonical-front, stable-PATH and
original grove-llm obligations are authoritative. The current session always
ends with its original harness-dispatch exit. Ravel is not its replacement.

## Done when

- Reusable preparation/refusal tests record original paths/digests, complete keg contents, automatic-cleanup suppression, non-overwriting stable-PATH link ownership and post-driver cleanup. Preserve original admission/teardown executable too.
- Isolated upgrade/unlink uses actual original 22.1.0 front/worker to launch/reap a next fake session/write its old temporary store; bare exit signals only the temporary legacy channel. Missing worker/keg or existing link refuses safely. Independent Ravel context remains isolated.
- Hand k13 exact application/verification/recovery instructions and a tested instrument; do not alter Homebrew installation or real PATH/config/history/authority. Retain original bytes through final finish's last signal.

## Notes
