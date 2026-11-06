//! The **guaranteed core** — the whole of `${prompt}`.
//!
//! Three driver-authored parts and no methodology: an imperative naming the
//! `grove-<kind>` skill, the runtime facts, and grove's own signalling contract
//! (`docs/specs/module-decomposition.md`, decisions 9 and 10). What earns a
//! place here is settled by the **too-late test**: a sentence rides `${prompt}`
//! only if its failure mode is one the skill cannot repair, because by the time
//! the skill could speak the moment has passed. Three shapes pass — a fact only
//! the driver holds, the instruction to open the skill, and the mechanism the
//! session's last action drives — and the test is closed on the word *fact*:
//!
//! > A driver fact is a **launch-varying value** that the methodology cannot
//! > know at authoring time. Its static meaning, and every normative consequence
//! > of it, stay in the skill.
//!
//! That closure is why the runtime facts below are bare values. *This selection
//! is authoritative*, *do not probe for the version control* — both are rules
//! with counterparts in the shipped spine (`plugins/grove/skills/grove/SKILL.md`,
//! *What the driver settled before your session*), and restating either here
//! would be the second source this design exists to avoid.
//!
//! **The prose drift surface is zero bytes, and it is now zero by construction
//! rather than by assertion.** This module reads nothing: the whole template is
//! driver prose, so there is no embedded content for a Rust literal to drift
//! from and no per-kind content decision to keep in step. That is what
//! `prompt-names-the-kind-k18` deleted — `reference_file`'s nineteen-to-ten
//! `match`, `ending_file`'s nineteen-to-two one, the `${locations}` slot, and
//! with all three this module's dependency on `crate::methodology`. The driver
//! now interprets a kind **nowhere**: it renders the kind's own label into a
//! skill name and stops.
//!
//! **One coupling remains, and it is not closeable from here.** The prompt names
//! a `grove-<kind>` skill, and the user's chosen delivery route has to supply
//! it (`docs/adr/homebrew-installs-skills-and-ravel-delivers-them.md`). Nothing in this
//! workspace can check what a machine supplies, so the assertion available is
//! over the shipped set, and it lives in `tests/prompt.rs`. The residue is
//! answered in the prompt itself, because a missing skill cannot state it: the
//! load instruction says what to do when the named skill cannot be found or
//! read (`docs/specs/codex-skill-provisioning.md`, *Installation and
//! bootstrap*). Grove also states its version, so a session can see which
//! guidance it needs.

use crate::{Handle, Kind};

use jj_workspace::Workspace;

/// The prefix every kind skill's name carries.
///
/// This is a convention of skill names, not a plugin namespace. A kind skill
/// can ship in any plugin or skill directory, and the plugin that supplies it
/// decides its native namespace (`docs/specs/codex-skill-provisioning.md`,
/// *Launch and compatibility*). The prompt therefore names only the bare skill.
/// `tests/prompt.rs` holds this prefix against the shipped skill directories.
pub const SKILL_PREFIX: &str = "grove";

/// The skill a session of `kind` is told to load: the per-kind skill, named for
/// the kind's own label.
///
/// **This is a rendering, not an interpretation.** Nothing here branches on
/// which kind it is, which is the whole of what decision 5 asked for: grove
/// names a kind only where grove writes the leaf, and the last two places that
/// *interpreted* one were the two `match`es this replaces.
pub fn skill_name(kind: &Kind) -> String {
    format!("{SKILL_PREFIX}-{}", kind.label())
}

/// **Part 1 — the load instruction.** First, because it is the first action.
///
/// The wording descends from `docs/research/wording-micro-test.md`, *The winning
/// wording*, and carries the two elements of it that survive the plugin:
///
/// * **One imperative naming one target.** The driver resolved the kind before
///   the session existed, so the session performs no selection and has nothing
///   to defer. This is the element the test measured as load-bearing: the
///   control opened `SKILL.md` in 9 of 10 sessions and reached its kind's
///   *procedure* only after starting work, in every session of both arms. The
///   test's imperative named two targets because the methodology was one skill
///   with a routing table; the plugin gives each kind its own skill, so one
///   target is now the whole procedure and the property is preserved rather than
///   weakened.
/// * **An ordering clause that enumerates the tempting alternatives.** The
///   enumeration is what makes an ordering clause bite; "read this first"
///   without it is advice.
///
/// **The target is the bare skill name, with no namespace.** The prompt once
/// spelled a fixed `grove:` namespace beside it. A pack move can change the
/// plugin that supplies a kind skill without changing the kind, so a fixed
/// alias would point at nothing. The instruction tells the session to use the
/// namespace under which the skill is actually installed, and Grove keeps no
/// kind-to-plugin map. A name with "load" is a name request: with no native
/// copy, the agent can find the same name in directories Ravel supplies.
///
/// **"First" is qualified with "after any mandatory guidance".** Ravel's
/// directory delivery can place mandatory reads before the caller's block, and
/// an unqualified "first" would contradict that order.
///
/// **The missing-guidance rule rides here, because the missing skill cannot
/// state it.** A session that cannot find or read its skill reports that in a
/// visible reply, leaves the leaf live, records no teardown and runs the
/// launch's exact force-stop command. A report with no defined ending could
/// otherwise relaunch the same live leaf indefinitely. Part 3 keeps the
/// force-stop mechanism and its result meaning, so this part states only when
/// to use it. It still names the visible reply, because a load can fail before
/// the session reads part 3.
///
/// **The third measured element is gone with the thing it described.** The
/// provisioned directories rode this instruction by absolute path, and no
/// directory is provisioned any more. The gap is recorded rather than argued
/// away (`docs/specs/module-decomposition.md`, decision 9): a harness with a
/// skill-loading affordance is unaffected, one without loses its fallback, and
/// the reopen condition is a session that cannot reach the methodology by the
/// affordance alone.
const LOAD_INSTRUCTION: &str = "\
**Load the `{skill}` skill now.** Your kind is `{kind}`; Grove resolved
that before this session existed, so there is no selection for you to make.
This is the bare skill name. If your harness requires a plugin namespace, use
the namespace under which `{skill}` is actually installed. Do not assume one.

Do this **first**, after any mandatory guidance and before anything else you
might reach for: before reading the task file, before running any `grove-llm`
verb, before looking at `.grove/`, before inspecting the working tree, and
before answering a question.

If you cannot find or read `{skill}`, report that its guidance is missing or
unreadable. Give that report in a visible reply to the human, as its own step
before the force-stop call. Leave the task live: do not retire the leaf, record
a teardown or run the continue command below. Run this exact force-stop command
as your last action instead:
`{force_stop_command}`
";

/// **Part 2 — the runtime facts.** Three launch-varying values and no normative
/// tail; see this module's header for why the tail left.
///
/// The version is the third, and it is what
/// `docs/specs/module-decomposition.md`'s decision 10 inverts the compatibility
/// check onto: the machinery states what it is, and the methodology decides
/// whether that is good enough. A value in the prompt needs no command to
/// succeed and cannot fail — where a verb would need the CLI on `PATH` and would
/// fire only if the session thought to run it, which is exactly the deferred
/// read the micro-test measured.
const RUNTIME_FACTS: &str = "\
Grove mandate: the leaf selected for this session is `{handle}`.

Version control: {stated_vcs}.

Grove version: {version}.
";

/// **Part 3 — grove's own signalling contract.** Last, because it describes the
/// last action.
///
/// One text for every kind: Ravel owns process supervision and opaque result
/// transport, while Grove reads the public result after reap. The kind's
/// ending remains the sentence's object; the exact ending command is supplied.
/// A finish records teardown separately before publishing the same result.
/// A finish whose skill is unread can publish only that result after deleting
/// the tree, causing a relaunch. Decision 9 records that residue and its reopen
/// condition: a continue result after an unrecorded teardown.
///
/// **A force-stop states its reason in a visible reply, before the call.** An
/// agent writes its closing reply after its last tool call, and no step follows
/// a force-stop. In three trial runs, Claude Code therefore put an accurate
/// report in a thinking block, directly before the force-stop call
/// (`docs/common-loading-acceptance-record.md`). The rule passes the too-late
/// test, because it governs the mechanism of the last action. Only this
/// contract states that mechanism, and a session whose skill is missing can
/// read no other source. A normal ending asks for no reply.
const SIGNALLING_CONTRACT: &str = "\
**Signal.** Your kind's skill states how this session ends. After retirement and
commit (including any settled parent cascade), run this exact command as your
**last action — then do nothing else**:
`{ending_command}`
A finish session first runs `grove-llm record-teardown`, then the same command.
To force-stop the sequence without acknowledging completion, first state the
reason in a visible reply to the human, as its own step. Reasoning alone is not
that reply, because the human may not see it. No step follows the force-stop
call. Then, as your last action instead, run this exact command:
`{force_stop_command}`. Remaining live work is resumable by rerunning Grove;
force-stop does not retire a leaf.
Ravel finds the enclosing session from inherited process context, terminates and
reaps its managed group, and returns Grove's opaque result unchanged.
Grove interprets it only after reap and epoch invalidation. Driver interruption
takes precedence, then successful Ravel status with exactly `grove-force-stop`
stops without completion even if teardown was recorded. Otherwise a recorded
teardown finishes; absent teardown, successful status and exactly
`grove-continue` continue with fresh context only after the group is gone.
Without a supported result the
loop stops with the leaf live; ending a turn in an interactive harness may leave
it waiting at its prompt. A crash or Ctrl-C can end the harness without a result.
Outside a Ravel session, `end-session` refuses; it needs no session ID argument.
The ending command in this launch prompt governs this session. Legacy sessions
launched by Grove 22.1.0 through dispatch keep their original
`harness-dispatch exit` command and channel, including admission and teardown.
";

/// Everything one launch varies, and the whole of what composition reads.
///
/// The shape is `docs/specs/module-decomposition.md`'s, decision 9. `kind` is
/// borrowed rather than owned: it became a validated token holding a `String` at
/// `open-kind-k20`, and a mandate is read once and dropped.
pub struct Mandate<'a> {
    /// The selected leaf's stable handle. Held as the owning type rather than as
    /// a rendered string: a handle is a projection of a name and the name's
    /// owner does the projecting (`docs/specs/module-decomposition.md`,
    /// decision 4). This module composes text and spells no grammar of its own.
    pub handle: &'a Handle,
    /// The selected leaf's kind — rendered into a skill name, never interpreted.
    pub kind: &'a Kind,
    /// The resolved jj workspace the session will run in. The **value** the
    /// prompt states the version control from, so no session ever detects it
    /// (`docs/ARCHITECTURE.md#symmetric-vcs-rule`).
    pub workspace: &'a Workspace,
    /// Grove's published release version — the workspace's single version, which
    /// orders and means something to a human. `grove --version` renders the same
    /// value and remains as a fallback, not as the mechanism.
    pub version: &'a str,
    /// Exact shell command built from this launch's once-resolved Ravel path.
    pub ending_command: &'a str,
    /// Exact force-stop command built from the same resolved executable.
    pub force_stop_command: &'a str,
}

/// Compose the whole of `${prompt}` for one launch.
///
/// Three parts in the session's own timeline order — load instruction, runtime
/// facts, signalling contract — joined by a blank line, for every kind.
///
/// **Infallible**, which is the shape of the deletion rather than a convenience:
/// composition used to read the embed and could fail on a path the corpus did
/// not carry. It reads nothing now, so there is no launch left to fail.
pub fn compose(mandate: &Mandate<'_>) -> String {
    // The command is substituted last, so a path that contains a template
    // token is not expanded again.
    let load = LOAD_INSTRUCTION
        .replace("{skill}", &skill_name(mandate.kind))
        .replace("{kind}", mandate.kind.label())
        .replace("{force_stop_command}", mandate.force_stop_command);
    let facts = RUNTIME_FACTS
        .replace("{handle}", &mandate.handle.to_string())
        .replace("{stated_vcs}", &stated_vcs(mandate.workspace))
        .replace("{version}", mandate.version);

    [
        load,
        facts,
        // Substitute only template text; literal paths may contain either token.
        SIGNALLING_CONTRACT
            .split("{ending_command}")
            .map(|part| part.replace("{force_stop_command}", mandate.force_stop_command))
            .collect::<Vec<_>>()
            .join(mandate.ending_command),
    ]
    .join("\n")
}

/// The **value** that states this working tree's VCS to the session.
///
/// The fact is the driver's: the workspace resolved before this session existed.
/// Only the session re-derived it, and re-derived it badly — a harness banner
/// computed from `.git` alone reads a jj workspace as no repository at all, and
/// detection carried as skill instructions is skippable, so a session that never
/// loaded them commits with Git in a jj tree and bypasses the operation log.
///
/// **Two elements, and the third one left**: identity and the resolved root, and
/// no *do not probe for it*. That clause is a normative consequence of a value,
/// and the closed fact test hands every such consequence to the skill — the
/// spine's `stated-vcs-is-definitive` states it, and stating it here again
/// would be the second source the core exists to avoid.
///
/// Still deliberately **not** the commit-boundary commands — those live in the
/// methodology's Commit step, and a copy here would drift across the build
/// boundary (`docs/ARCHITECTURE.md#the-boundary-is-a-build-not-a-commit`).
fn stated_vcs(workspace: &Workspace) -> String {
    format!(
        "this working tree is jj-enabled (jj workspace root: `{}`)",
        workspace.root().display()
    )
}
