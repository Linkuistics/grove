# grove-native-cleanup-k33

## Goal

Supply the optional manual cleanup procedure for installations left by the
removed Codex provisioner, within the same unreleased native increment.

## Context

Consume grove-native-provisioning-k21 and the delivery specification's existing
installations and active-readers contract. Use its evidence document's cleanup
consumer map. This leaf changes guidance and its disposable verification only.

## Done when

- Write the optional procedure in `docs/USAGE.md` and link it from plugin
  guidance. Require stopped writers/readers and evidence of ownership and lack
  of retained references before any removal. Path shape alone is insufficient.
- Exercise the procedure in disposable legacy layouts: stopped writers/readers,
  owned and foreign links, aliases, uncertain provenance and retained snapshot
  references. Record which entries remain and which evidenced owned entries can
  be removed. Preserve the owner's real installations.
- Verify the documentation links and applicable documentation/conformance
  checks. Record commands and observations on this task. The parent cannot
  close or publish the native increment without this procedure.

## Notes

Add no automatic cleanup, cleanup CLI, owner-configuration change or release.
