# discover-namespace-k17


## Goal
Discover an existing consumer namespace at an exact workspace without creating
anything, walking ancestors or following a secondary repository pointer.



## Context
Use Workspace::control_dir and validated_namespace in jj-workspace. The public
discovery operation takes a location directly so Workspace::resolve cannot
spawn jj for a secondary workspace. The loop owns descriptor/path validation
when it later opens runtime records; this operation returns a discovery path,
not a retained identity or ownership proof.

## Done when
- The public discovery operation shares namespace validation and path derivation
  with creation. Missing exact .jj or namespace returns None; malformed paths
  and inspection errors return a refusal. Exact workspace aliases work.
- Public tests cover ordinary/secondary workspaces, non-jj and missing namespace,
  subdirectories, aliases, invalid namespaces and nondirectory substitution.
  Filesystem snapshots include administration bytes and show no writes.
- Update workspace documentation and its complete-source walkthrough, source
  and concept indexes and manifest. Run workspace tests and bash scripts/check.sh.

## Notes
The parent retains runtime record opens, locking, tree lifetime and viewer
behavior. This is independently testable through the workspace library seam.

## Decisions (running log)

- Add Workspace::discover_control_dir(location, namespace), without constructing
  a Workspace or asking for its main repository. Share a private control_path
  helper with creation. Directory metadata is a sample, so runtime consumers
  must validate their opened descriptors; no lock or identity guarantee is
  attached to the returned PathBuf.
- Test absence, existing namespaces and refusal before implementation; then
  repair the namespace explanation and exact fragments and run the principal
  gate. The reviewed parent spec supplies the design and authorization.
- Discovery uses metadata only, distinguishing a missing entry from a dangling
  symlink and refusing nondirectories without opening them. The existing opaque
  control-directory refusal now names type and permissions, with write access
  required only for creation. No admission or driver protocol changed.
- Six new public discovery tests passed after failing against the absent
  implementation; the existing secondary-workspace test also exercises the
  new operation. Exact book reconstruction passes over 797 lines. The final
  principal run follows the completed code and documentation edits.
