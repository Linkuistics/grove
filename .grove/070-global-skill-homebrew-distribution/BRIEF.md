# 070-global-skill-homebrew-distribution — brief

**Kind:** planning (node)

## Goal

Make `brew install grove` the **sole install gesture** and **perform the flip**
(ADR-0034). One binary (`grove`) **embeds grove's full retained methodology and
extracts it on launch** to the global `~/.claude/skills/grove/` — no fetch, no
`VERSION.md` drift. New-scheme prose lands in `content/`; the new binary is
installed; this grove (and `grove-general-improvements`) migrate to dotted-decimal
by adoption; the project-local skill mirrors are removed. This is where the world
goes new-format and distribution collapses to one gesture (ADR-0031, ADR-0034).

## Done when

- `brew install grove` lands the binaries; first `grove do` provisions the global
  skill — the separate `grove install` gesture is no longer needed (030).
- The `grove` binary embeds `content/` and extracts it idempotently to
  `~/.claude/skills/grove/` on launch, skill always matching binary (010).
- `content/` prose describes the dotted-decimal scheme + migrate-on-adoption; no
  stale `NNN-slug` scheme language remains (020).
- The new binary is installed, this grove's tree is adoption-migrated to
  new-format, and the global skill is live and verified (040).
- The three project-local `.claude/skills/grove/` mirrors are removed and the
  precedence note is documented (050).

## Decomposition

Ordered for safety — **provision the global skill before removing any mirror**
(else a session is left skill-less), and the **live flip is last and user-gated**:

- `010-embed-skill-and-provision` (work) — `grove` embeds `content/` at compile
  time and extracts it to `~/.claude/skills/grove/` idempotently on `grove do`.
  The VERSION.md-drift-free replacement for fetch+materialise. *Old fetch/install
  machinery is not deleted here — 090 does that; 010 establishes the replacement.*
- `020-new-scheme-prose` (work) — rewrite `content/` (SKILL.md, prompts, format
  guides) from `NNN-slug` to dotted-decimal + migrate-on-adoption.
- `030-homebrew-sole-gesture` (work) — `brew install grove` provisions everything;
  drop the separate `grove install` step; adjust the formula/release artifact for
  the embed model; declare any deps.
- `040-install-and-flip` (work, **USER-GATED**) — build + install the new binary;
  the next `grove do` adoption-migrates this tree and extracts the global skill;
  verify. The world flips new-format here.
- `050-remove-project-skill-mirrors` (work) — global skill now live, so delete the
  three `.claude/skills/grove/` mirrors + document precedence (enterprise >
  personal > project). Runs *after* 040 (executed by the new binary on the
  migrated tree).

## Pointers

- ADRs a session here must read: **ADR-0034** (the flip — migrate-on-adoption, no
  dual reader; this grove is itself flipped), **ADR-0031** (distribution collapses
  to one global skill + one brew gesture), **ADR-0006** (the grove/grove-llm
  audience split — affirmed here, see Notes), **ADR-0033** (the dotted-decimal
  scheme the prose adopts).
- Live provisioning path being replaced: `src/install.rs` → `src/fetch.rs` →
  `src/extract.rs`, stamped by `src/version_md.rs`, driven by the separate
  `grove install` gesture. New path lives in the `grove` binary (`src/main.rs`,
  `src/loop_driver.rs`/`src/launch.rs` for the `grove do` launch point).
- Prose source of truth: `content/` (canonical; today also mirrored into
  `.claude/skills/grove/`, removed by 050).
- Distribution mechanics: the spike's distribution section in
  `docs/research/loop-substrate-options.md` (personal `~/.claude/skills/` is read
  live by Claude Code; a system CLI ships via Homebrew).

## Notes

### Settled decisions (grilling, 2026-06-21)

- **Two binaries, `grove` provisions.** The ADR-0006 audience split is affirmed,
  not reversed: `grove` (human / loop driver) owns embed+extract because it has the
  natural launch point (`grove do`); `grove-llm` (LLM verbs) gets **no** provisioning
  logic. The formula keeps `bin.install "grove","grove-llm","rmux"`.
- **Provisioning authority is ADR-0034** ("binary embeds the skill and extracts it
  on launch"). The *mechanism* (e.g. `include_dir!` vs a build.rs tar blob) is an
  implementation choice for 010 — not ADR-worthy, no new ADR at this planning step.

### Inert until the flip

Every source/prose change in 010–030 is **inert for this grove** — it is driven by
the *installed old* binary, which keeps reading this old `NNN-slug` tree until the
new binary is installed at 040. Build and land 010–030 freely; the world flips only
at the 040 install step. `restart ≡ continuation` (ADR-0032) holds throughout.

### Defers to 090 (do not do here)

The actual **deletion** of the old fetch/install/materialise + `VERSION.md` code
(`src/install.rs`, `src/fetch.rs`, `src/extract.rs`, `src/uninstall.rs`,
`src/version_md.rs`) is **090**. 070 establishes the embed+extract replacement and
stops *relying* on the old path for distribution; 090 removes it. Likewise the dead
old-verb modules (root BRIEF "Dead code to sweep") are 080/090's to sweep.
