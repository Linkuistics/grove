# Grove extraction release evidence

Grove **22.2.0** is [published](https://github.com/Linkuistics/grove/releases/tag/v22.2.0)
and installed with independently released Ravel **0.1.0**. This is the
mid-grove extraction checkpoint; the original 22.1.0 driver and remaining
Grove-owned session-control work stay live.

## Source and publication

Fetched main `89f947d5` was an ancestor of accepted extraction `e423bfe8`.
Source-only integration `96b6fd1e214dafee94a82f9a43ed8a2758406872` contains
that complete project tree and release notes, excluding `.grove/`. The accepted
tree supplies a positive control for the coordination-file enumeration.

Release source and tag `v22.2.0` are
`a79cd5aa41834b7425106dc9b9896e1fd17e03d9`. Source main subsequently advanced to
`972fd3180f58ecbcc593c60a612c50301bebe0c4` for the verification repair below,
then `96a7d855e8c8b1e616cea0ddaefa69c5c8da571b` to pin its verified source citation.
Tap main is synchronized at `20ee956baca0b9f97daf3eccd2d2413ece2a88b7`, following
the same-version test correction `bc73cb9f1aaf54632d8abadd537c070192286657` and
the initial 22.2.0 formula publication `4daa4aef05ad`. Ravel source/main and its
formula are unchanged. Main contains no live coordination tree.

Every downloaded public archive matches its built bytes and GitHub's asset
digest individually. The tag and archives were never replaced.

| Target/archive | SHA-256 |
|---|---|
| [aarch64-apple-darwin](https://github.com/Linkuistics/grove/releases/download/v22.2.0/grove-v22.2.0-aarch64-apple-darwin.tar.xz) | `081893c977cebb734ae90280148f15be5fa73d605c327b7c19c054f3f31ca9e3` |
| [aarch64-unknown-linux-gnu](https://github.com/Linkuistics/grove/releases/download/v22.2.0/grove-v22.2.0-aarch64-unknown-linux-gnu.tar.xz) | `aad8b5a9ddb94530716c34f96684092d3f05b446a48b650a53a2494474b82d69` |
| [x86_64-unknown-linux-gnu](https://github.com/Linkuistics/grove/releases/download/v22.2.0/grove-v22.2.0-x86_64-unknown-linux-gnu.tar.xz) | `1938c16715a0aa8f37a989b69b5bb54bd50eea4a318f0cb6825b5cf4f631c76b` |

## Verification and recovery

| Boundary | Observed result |
|---|---|
| Source-only integration | All 18 principal checks passed: 1,378 workspace nextest cases, nine doctests and six final/all books. All 1,387 distinct runtime identities matched k58 individually; all 46,957 measured inputs retained their digests. |
| Release task | `task release:minor` passed its own 18-check gate, cut once, built the three tagged archives, passed their manifests and target acceptance, and published main/tag/release/tap before installation. |
| Tagged target acceptance | macOS native; Linux arm64 native plus Cortex-A53 QEMU; Linux x64 Nehalem QEMU. Actual lifecycle, confined standalone publication, readable imports, direct/symlink fronts and separate Ravel prefixes passed. CPU/glibc, descendant, backend-refusal and filesystem/security controls fired. Staged inputs retained their individual digests. |
| Real installation | Explicit Homebrew-bin versions are Grove/helper 22.2.0 and Ravel 0.1.0. All 11 command/caller-asset payload files match the public native archive; installed viewer help works. |
| Formula recovery | Real `brew test grove` passes the installed fake lifecycle. The corrected template also passed actual private Homebrew install/test, Ruby syntax and release-pipeline controls. |
| Confined installed verification | The actual new keg passes direct and symlink lifecycle, confined standalone output publication and caller imports with independent Ravel, without host Bun/Node. |
| Original-driver preservation | All 31 controls, complete original/backup inventories, owned-link identities and inherited-PATH checks passed before and after the actual upgrade. Both actual-original-worker rehearsals passed canonical launch, bare temporary-channel exit, reaping, nested-context isolation and helper checks. |

The initial release task stopped at its installed formula test: Homebrew's
outer sandbox refused Ravel's nested macOS `sandbox-exec` probe before policy
selection. The earlier private formula seam called the actual test method
directly, so it did not encounter that outer sandbox. The installed Homebrew
test command's [pinned source](https://raw.githubusercontent.com/Homebrew/brew/eb56e2b1f7/Library/Homebrew/dev-cmd/test.rb)
was fetched from the official repository and matches installed source byte for
byte. Current local help was checked; there is no supported test-sandbox opt-out
for this default-prefix installation.

The repair keeps the formula's lifecycle check inside Homebrew's sandbox and
runs the existing actual confined standalone instrument separately afterward.
It changes the template, release recipe and recovery documentation; executable
runtime code and published archives remain unchanged. The same-version tap
correction and source fix were committed/pushed through jj, then `brew update`,
`brew test grove` and explicit installed smoke/version/help checks completed
the unfinished release boundary. No second cut or publication was attempted.

The raw terminal release log records its gate exits and progress, but the
terminal reporter did not retain individual nextest case rows. The per-case
comparison above is the integration gate's named record, not an inference
from the release task's totals. The default gate skips 31 workspace cases;
earlier complete ignored-case evidence retains its own scope. Supplemental
default compiler-dependency digests also agree. Release-before/after records
show only the planned version/changelog, tap formula, running decision log
and new installed Grove-path differences; owner configs, Ravel and original
keg bytes agree. The live shared history store is excluded, not restored.

Complete logs and records are in this session's `target/k13`: the integrated
gate and per-case/freeze comparisons, raw `release.log`, public release JSON
and downloaded archives, built digests and installed payload pins, recovery
formula/release controls, Homebrew test/update logs, new installed checks,
legacy controls/prepare/apply/verify logs, and before/after actual-worker
rehearsals. The measurement instrument is the retained `target/k58/measure.py`;
its manifest defines the measured paths. Reporting follows measurement.

## Live continuation

The applied preservation plan is
`/Users/antony/Development/grove.extract-harness-dispatch-to-top-level/.jj/legacy-driver-22.1.0`.
Its inventory equals [the recorded original](legacy-driver-inventory.json).
Every later release/recovery shell, including final finish, must source its
`release-env.sh`. Keep the canonical original keg, complete backup, owned
stable-PATH `harness-dispatch`/`grove-llm` links and cleanup exclusions through
final finish's last original signal. The [transition procedure](legacy-driver-transition.md)
owns post-driver cleanup; none ran here.

This session is reconciled onto both accepted extraction ancestry and corrected
released main. Two merge conflicts in the recovery guide/template were resolved
by restoring main's complete project tree outside `.grove/`. Exact checks found
no remaining conflict, identical project content, unchanged coordination paths
and bytes, and main as an ancestor. The coordination history, workspace, lease
and original driver remain; k8 stays live. This checkpoint records no teardown
and ends with its original `harness-dispatch exit`.

One narrow fresh-context integration/reconciliation review found no issue,
conditional on the exact checks above. Those checks passed. Graph evidence was
Tier 2 initially at generation `2026-10-13T16:19:31Z`, with final relied-on-path
coverage at `2026-10-13T16:57:15Z`. The recorded QEMU-script parse gap at line 208 was read directly. Graph
coverage is best-effort, and heuristic cross-language call edges were not used
as proof of the preservation instrument's behavior.
