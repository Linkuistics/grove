# ravel-git-acquire-k76

**Reviews:** ravel-git-acquire-k70

## Goal

Read the online Git acquisition of Ravel commit `1898dc8603a0` adversarially.
Read it against `docs/specs/skill-discovery.md` at `822c0ed2`, "Exact Git
acquisition and offline reuse" without the cache hit, "Directory descriptor
and loading contract" and "Failures". Produce findings, not fixes. The parent
`1d50dae5d29e` is the base of the diff.

## Context

- The work is in the jj workspace `Ravel.skill-directory-delivery`, on the
  unpushed bookmark `skill-directory-delivery`. The new code is
  `crates/ravel/src/skills/git.rs`. The resolver changes are in
  `skills/mod.rs` and `skills/snapshot.rs`.
- The public fixtures are `crates/ravel/tests/skills_git.rs`. The running log
  of `ravel-git-acquire-k70` records the observed Git behaviour, the nine
  mutation controls and the checks on macOS and Linux arm64.
- Offline reuse is `ravel-git-offline-k71`. Deadlines, cancellation, child
  reaping and bounded redacted diagnostics are `ravel-git-bounds-k72`. Report a
  defect in those areas only when this commit makes a later leaf harder or
  unsafe.

## Specific doubts

- The system and global Git configuration stay in effect for the owner's
  credentials. Can a key there still run a command or a prompt? Examples are
  `url.<base>.insteadOf`, `include.path`, `includeIf`, `core.sshCommand`,
  `uploadpack.packObjectsHook` for a local repository and `fetch.bundleURI`.
  Does `protocol.allow=never` still hold after an `insteadOf` rewrite?
- `transfer.fsckObjects` is off. A crafted tree can hold a duplicate name,
  `.`, `..`, `.git`, a name with `/` or an odd mode. `Acquisition::name`,
  the reserved-name skip and the collision check must refuse or skip each
  one. Does a case exist that writes outside the extraction directory?
- `fetch` classifies a format mismatch by the text `mismatched algorithms`
  under `LC_ALL=C`. Other failures are transport failures. Can a missing
  commit, a refused want or a server error receive the wrong remedy?
- The direct fetch uses `--depth=1`. Its failure is followed by a full fetch
  of `+refs/*:refs/fetched/*` into the same repository. Can a shallow file
  from a partial first fetch hide an object from `cat-file`?
- `Git::run` lists the inherited descriptors before the spawn and marks each
  one above 2 close-on-exec in `pre_exec`. A thread of Ravel could open a
  descriptor between the list and the fork. Is that reachable, and does it
  matter here?
- Refusals carry up to 1000 bytes of Git's standard error, unredacted, until
  `ravel-git-bounds-k72`. Settings refuse a URL with credentials. Can the
  excerpt still show a secret, such as a header or a token in a redirect?
- The protected-overlap check of a local repository percent-decodes a
  `file://` path and canonicalises it. Git can resolve the same locator to a
  different path. Find a locator that reaches a protected path and passes.
- No control proves the new session without a controlling terminal, because
  the test process has no terminal. Judge whether `setsid` alone is
  sufficient for SSH and for Git's credential prompt.

## Done when

- Each doubt above and each other defect found has a finding with evidence,
  or a recorded reason why it does not hold.
- The findings are committed in this leaf. Cut an integration only if a
  finding is worth acting on.

## Notes

Insert any integration before `ravel-git-offline-k71`, which edits
`git.rs` next.

## Decisions (running log)

**Review boundary.** This is an inspection-only review of Ravel commit
`1898dc8603a0befd77f8b39f43e8b0abc7dc4da8` against parent
`1d50dae5d29e` and specification `822c0ed2`. No test, build, lint, format or
acquisition probe runs in this session. The producer's check receipts are
recorded evidence, not checks independently repeated by this reviewer.

**Source evidence.** Tier 2 graph discovery used project
`Users-antony-Development-AgenticOrchestration-Ravel.skill-directory-delivery`.
Its recorded generation is `2026-11-02T05:30:19Z`. Coverage reports `git.rs`
and `tests/skills_git.rs` as untracked by that generation, the changed resolver
files as stale, and `docs/` as excluded. The review therefore reads their
source directly. The unchanged held-directory implementation also has graph
snippet and call-trace evidence. No claim depends on absent graph results.

**Locator admission.** The enabled acquisition path admits credential-bearing
query strings and fragments and copies the configured URL into its descriptor
and refusals. This is separate from the scheduled redaction of Git stderr in
k72: redacting stderr cannot repair admitted source provenance. The local
overlap guard also checks a different interpretation of a file URL from Git's.
These are actionable findings for an integration before k71.

**Transport configuration.** Upstream Git source distinguishes protocol-specific
permission from the default permission, and `fetch.bundleURI` from
`transfer.bundleURI`. The command-line settings cover neither distinction.
Local `upload-pack` also reads its pack-objects hook from the retained global
configuration. These are separate actionable holes in k70's transport hygiene,
not the missing deadline or redaction already assigned to k72.

**Remaining doubts.** Six source-derived findings warrant integration. The
format classifier misses protocol-specific diagnostics. The extraction path,
shallow-file concern, inherited-descriptor sweep and stock Git/OpenSSH prompt
suppression have no additional demonstrated defect within this leaf's scope;
their reasoning and evidence limits are recorded below. Stderr redaction stays
with the already scheduled k72, while admitted URL credentials need earlier
triage because they reach retained provenance.

**Integration placement.** Created `ravel-git-acquire-k77` as the adjacent
integration before `ravel-git-offline-k71`. It consumes this review by handle
and owns triage and verification. The pinned-delivery parent still has live
work, so this retirement closes no parent. No Ravel file or bookmark changed
in this review.

## Findings

All Ravel file and line references below are at commit `1898dc8603a0`.
These are source-derived counterexamples, not executed reproductions.
P1 means a protected-resource or credential boundary fails; P2 means an
acquisition restriction or actionable refusal is wrong.

### F1 — P1: Check the repository Git actually opens for protected overlap

Locations: `crates/ravel/src/skills/git.rs:1007` and
`crates/ravel/src/skills/mod.rs:456`.

Let `H` be an isolated absolute HOME with a valid bare repository at
`H/.config/ravel/repo`. Configure its real commit and the locator obtained
by percent-encoding every slash of that absolute path, then prepending
`file://`. For example, with `H=/tmp/ravel-owner`, the locator is
`file://%2Ftmp%2Fravel-owner%2F.config%2Fravel%2Frepo`.
The settings parser accepts it. `local_path` searches for a literal slash
after `file://` before decoding anything. It finds none and returns `None`,
so `plan` performs no overlap refusal. Git decodes the whole URL before
splitting it, and reaches the protected repository. Ravel can then publish
its committed files as readable resources.

The underlying gap is broader than that encoding. A nonexisting `/tmp/alias`
can be resolved by Git as `/tmp/alias.git`, where the latter is a stable
symlink to the protected repository. `canonicalize` of the configured spelling
fails and is silently ignored. An ordinary directory outside the protected
tree with a `.git` file pointing to that repository also passes the check of
the directory itself. Git's repository-opening code follows that file.
Finally, an HTTPS locator rewritten to a protected local repository by global
`url.*.insteadOf` never enters `local_path` at all.

These are stable inputs, with no concurrent replacement. The existing fixture
at `tests/skills_git.rs:949` covers direct paths and direct symlinks, which
canonicalisation does catch. Integration needs controls for the effective
repository path, including these alternate ways to reach it, at both public
seams. A successful snapshot must not turn protected origin content into a
read grant. Evidence: Git's
[URL parsing](https://github.com/git/git/blob/v2.51.0/connect.c#L1013-L1076),
[repository opening](https://github.com/git/git/blob/v2.51.0/path.c#L698-L788)
and [URL rewrites](https://github.com/git/git/blob/v2.51.0/remote.c#L39-L93).

### F2 — P1: Refuse credentials before retaining or reporting the locator

Location: `crates/ravel/src/skills/mod.rs:705`; admission is
`crates/ravel/src/skills/settings.rs:298` and direct failure reporting is
`crates/ravel/src/skills/git.rs:206`.

Configure a normal full pin with URL
`https://example.invalid/pack.git?access_token=OWNER_SECRET`. The parser checks
only user information before the first slash. It accepts the credential in
the query. A fragment such as `#access_token=OWNER_SECRET` also passes.
When acquisition fails, `fetch` inserts the URL directly into both its message
and remedy, and `unavailable` adds it as `source`. When acquisition succeeds,
the new Git source record copies the complete URL into the retained descriptor,
`skills resolve --json` and `request.skills`. The descriptor becomes readable
to the harness and delegates.

This violates the reviewed specification's refusal of query and fragment
credentials and its credential-free source provenance. The parser predates
this commit, but k70 removes the unconditional Git refusal and introduces the
successful persistence path. Redacting Git stderr in k72 cannot repair these
direct copies or a published descriptor. Integration needs admission and
public-boundary controls before any Git child, with a secret marker absent
from refusals and retained output. Review settings-error rendering too, since
simply rejecting a URL by echoing its complete value still discloses it.

### F3 — P2: `protocol.allow=never` is a default, not an allowlist

Location: `crates/ravel/src/skills/git.rs:985`.

Keep a global `protocol.ext.allow=always` and a global URL rewrite from the
configured HTTPS locator to `ext::/tmp/ravel-git-helper`, where that path is
an executable. Ravel retains the global configuration, sets the default
protocol permission to `never`, and enables HTTPS, SSH and file. Git first
checks the protocol-specific permission. Its retained `ext` permission wins
without consulting the default, so it can start the external transport helper
and its command. The same gap admits other explicitly enabled protocols,
including HTTP after a rewrite.

Thus the new specification statement that every other transport is disabled
is false. `insteadOf` does not inherently bypass Git's transport check; the
remaining protocol-specific setting bypasses Ravel's intended restriction.
Integration needs a control with both the rewrite and a global protocol
permission, not only a disallowed direct URL or the absence of a helper.
Evidence: Git's
[permission selection](https://github.com/git/git/blob/v2.51.0/transport.c#L1015-L1065)
and [helper admission](https://github.com/git/git/blob/v2.51.0/transport-helper.c#L1241-L1254).

### F4 — P2: `fetch.bundleURI` still acquires from another location

Location: `crates/ravel/src/skills/git.rs:994`.

Set global `fetch.bundleURI` to a controlled bundle or bundle-list URL and
configure an unrelated skill repository. Each `fetch` consults that setting
and fetches the bundle before it handles the named repository. The only
bundle setting Ravel overrides is `transfer.bundleURI`, which controls a
different route. It does not disable the explicit fetch setting.

This introduces an extra network or local read and imports objects from a
location outside the configured acquisition. It also contaminates the object
store used to decide whether the required commit exists: after a successful
advertised-ref fetch, `subdir_tree` checks only whether that store contains the
pin, not where it came from. An unrelated bundle holding the pin can therefore
supply it when the configured repository does not.

The no-bundle statement in the new source documentation and specification
needs a real control for `fetch.bundleURI`. This matters before k71 gives the
result an acquisition key and reuses it offline. Evidence: Git's
[fetch entry point](https://github.com/git/git/blob/v2.51.0/builtin/fetch.c#L2355-L2359)
and [bundle import](https://github.com/git/git/blob/v2.51.0/bundle-uri.c#L780-L808).

### F5 — P2: Local upload-pack can still execute a Git hook

Location: `crates/ravel/src/skills/git.rs:979`.

Set a global `uploadpack.packObjectsHook` to a wrapper that records a marker
and then runs its arguments. Resolve an ordinary local repository through
either an absolute path or `file://`. Git starts a local `upload-pack`, which
reads the global hook and substitutes it for `git pack-objects` when it needs
to send the pack. The empty `core.hooksPath` affects directory-based hooks;
it does not override this configuration-based hook.

The marker can be written even though resolution succeeds. This contradicts
k70's no-hook outcome and the book's new unconditional rule. No hostile system
configuration is needed: an ordinary global caching wrapper for pack creation
is sufficient. The public no-hook fixture tests reference-transaction,
post-checkout, template and smudge behaviour, so its mutation controls do not
cover this path. Integration should cover the local upload-pack process and
its configuration separately from the fetch client's hook directory.
Evidence: Git's
[pack hook contract](https://github.com/git/git/blob/v2.51.0/Documentation/config/uploadpack.adoc#L39-L52)
and [hook execution](https://github.com/git/git/blob/v2.51.0/upload-pack.c#L263-L280).

### F6 — P2: Object-format failures do not all use the recognised message

Location: `crates/ravel/src/skills/git.rs:195`, also line 202.

Use global `protocol.version=0`, a SHA-1 repository and a syntactically valid
64-digit pin. The private destination is SHA-256. Git's protocol-0 path
reports that the server does not support the destination's object format;
it does not use the `mismatched algorithms` text of the protocol-2 path.
Both fetch attempts therefore reach Ravel's generic transport refusal,
with a source-level location and a restore-access remedy, instead of its
revision-level format remedy. Protocol 2 without an advertised format has
another unsupported-algorithm message. `LC_ALL=C` does not make those code
paths share one message.

The existing format fixture exercises a 40-digit pin against SHA-256 under
the default protocol. It does not cover the reverse direction or protocol 0.
Integration should verify the supported protocol and format combinations and
avoid asserting a diagnosed repository format from an arbitrary substring of
server stderr. A server error can itself contain the recognised text and
trigger a false diagnosis. No wrong commit is accepted in this case; the
defect is the promised diagnosis and remedy. Evidence: Git's
[protocol-specific format checks](https://github.com/git/git/blob/v2.51.0/fetch-pack.c#L1135-L1136)
and [protocol-2 variants](https://github.com/git/git/blob/v2.51.0/fetch-pack.c#L1264-L1272).

## Disposition of the specific doubts

- **System and global configuration:** F1, F3, F4 and F5 cover effective
  locator changes, protocol permission, explicit bundles and the pack hook.
  `include.path` and `includeIf` read more configuration; they are not by
  themselves command execution. They can supply the same problematic keys.
  `core.sshCommand` does execute an owner-selected transport command, as Git's
  [SSH selection](https://github.com/git/git/blob/v2.51.0/connect.c#L1079-L1088)
  shows. The contract deliberately retains owner SSH settings and credential
  helpers. Their existence alone is not an additional defect. Arbitrary
  owner helpers are not made noninteractive merely by a Git setting; claims
  about their prompts must stay bounded to the supported helper behaviour.
- **Crafted tree and extraction escape:** `name` at `git.rs:481` refuses
  non-UTF-8, control characters, empty names, `.`, `..` and embedded `/`.
  Exact `.git` and `.jj` entries are skipped before any write. Repeated
  names and case collisions meet exclusive creation or `mkdir` and refuse.
  Directories are opened relative to held descriptors with `O_NOFOLLOW`;
  files use `O_EXCL|O_NOFOLLOW`; symlink creation does not follow the target.
  A later entry cannot traverse an earlier symlink because names contain no
  slash and a duplicate directory creation fails. Capture then validates
  link chains before publication. I found no source-name route that writes
  outside extraction. Git can canonicalise raw mode permission variations
  before `ls-tree` prints them; Ravel materialises only its explicit file,
  directory and link cases, with fixed safe permissions. Other reported
  modes refuse. This is not evidence that every malformed Git object has
  been exercised. See Git's
  [tree decoding](https://github.com/git/git/blob/v2.51.0/tree-walk.c#L14-L42).
- **Format and transport remedies:** F6 addresses the incorrect format
  diagnosis. An ordinary missing commit after a successful advertised fetch
  gets the revision remedy at `git.rs:231`. A fetch that cannot complete
  remains a transport failure. I found no substitution of a nearby commit.
- **Partial shallow fetch:** A shallow boundary changes ancestry traversal,
  not direct object lookup by `cat-file`. Extraction reads the exact commit
  and its trees and blobs directly. A successful depth-one fetch needs those
  contents, while a partial failed transfer that lacks them cannot pass the
  later reads. Git's shallow-fetch code also uses a lock and rolls it back
  on a connectivity failure. I found no demonstrated case in which the
  first attempt's shallow metadata hides a required object that the fallback
  otherwise fetched. This is a static conclusion, not a partial-transport
  fault-injection result. Evidence: Git's
  [shallow connectivity handling](https://github.com/git/git/blob/v2.51.0/fetch-pack.c#L1984-L1997).
- **Descriptor race:** The public CLI resolves synchronously before worker
  or harness launch. Each Git stderr drain starts after that child's fork
  and is joined before the next Git call (`git.rs:890`, `git.rs:923`). It
  cannot open a competing non-close-on-exec descriptor during the next sweep.
  Ravel's directory and file opens use close-on-exec. I found no reachable
  production opener that defeats the inherited-descriptor sweep in this
  CLI path. This reasoning would need revisiting if acquisition became a
  parallel or embedded API; it is not a general race-free fork guarantee.
- **Secrets in stderr:** Yes. A server can place arbitrary text in an error,
  including a secret from a redirect or credential context, and `excerpt`
  merely replaces control characters. Limiting it to 1000 displayed bytes
  does not redact it. This is already explicit k72 work, so it is not a new
  finding. F2 concerns direct provenance copies that stderr redaction cannot
  fix. No real credential was read or used for this review.
- **Protected local locator:** F1 supplies concrete inputs that pass the
  guard and reach a protected repository.
- **Terminal isolation:** `setsid` removes the controlling terminal, but
  alone would not prevent GUI askpass or arbitrary helper prompts. The
  implementation also gives Git null stdin, sets `GIT_TERMINAL_PROMPT=0`,
  sets an empty `GIT_ASKPASS`, and sets `SSH_ASKPASS_REQUIRE=never`. Git's
  [prompt code](https://github.com/git/git/blob/v2.51.0/prompt.c#L42-L69)
  and stock OpenSSH's
  [passphrase code](https://github.com/openssh/openssh-portable/blob/V_10_0_P1/readpass.c#L115-L180)
  support that combination for their built-in prompts. The test's `notty`
  assertion has no positive controlling-terminal control, as the producer
  acknowledged, so it does not experimentally prove the setsid property.
  I found no separate stock Git/OpenSSH terminal bypass. Arbitrary configured
  helper commands need a separate supported-behaviour claim; they are not
  sandboxed by this code.

## Review limits and handoff

The producer records passing macOS `task check`, 13 Git fixture cases,
documentation checks and Linux arm64 checks, plus nine failing mutation
controls. This review inspected that record and the fixtures; it did not
repeat those commands. None of those controls covers F1's encoded or effective
repository locator, F2's URL credentials, or the configuration routes in F3
through F5. The format fixture has the narrower coverage described in F6.

Upstream citations are pinned Git 2.51.0 and OpenSSH 10.0p1 source evidence.
They support the static counterexamples and prompt reasoning; they are not
receipts for executing the installed Apple Git 2.54 or Homebrew Git 2.56.
Integration owns reconciliation with the actual supported executables and
all reproductions, fixes and post-fix verification. The offline cache,
deadlines, cancellation, descendant reaping, extraction bounds and stderr
redaction remain the already scheduled k71/k72 work.
