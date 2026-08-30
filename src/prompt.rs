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
//! with all three this module's dependency on [`crate::methodology`]. The driver
//! now interprets a kind **nowhere**: it renders the kind's own label into a
//! skill name and stops.
//!
//! **One coupling remains, and it is not closeable from here.** The prompt names
//! a `grove-<kind>` skill, and that skill has to exist in the installed plugin.
//! Nothing in this workspace can check the *installed* set — a plugin is
//! installed per machine, by a marketplace the binary does not read — so the
//! assertion available is over the shipped set, and it lives in
//! `tests/prompt.rs`. The methodology's own answer to the residue is stated
//! where the cost falls: a kind exists **iff** a skill of that name exists
//! (`plugins/grove/skills/grove/SKILL.md`), and grove states the version it is
//! so a session can see which plugin it needs.

use grove_loop::{Handle, Kind};

use jj_workspace::Workspace;

/// The plugin the kind skills ship in, and the namespace a harness that
/// namespaces plugin components registers them under.
///
/// Stated once here because it appears twice in one sentence — bare and
/// namespaced — and the two spellings must name the same target.
/// `plugins/grove/README.md`, *The token a prompt names*, is the decision;
/// `tests/prompt.rs` holds this against the shipped plugin's own manifest.
pub const PLUGIN: &str = "grove";

/// The skill a session of `kind` is told to load: the plugin's per-kind skill,
/// named for the kind's own label.
///
/// **This is a rendering, not an interpretation.** Nothing here branches on
/// which kind it is, which is the whole of what decision 5 asked for: grove
/// names a kind only where grove writes the leaf, and the last two places that
/// *interpreted* one were the two `match`es this replaces.
pub fn skill_name(kind: &Kind) -> String {
    format!("{PLUGIN}-{}", kind.label())
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
/// **Both spellings of the one target, in one imperative** — bare, and
/// namespaced for a harness that namespaces plugin components. That is
/// `plugins/grove/README.md`'s decision, taken so grove branches on no harness
/// and grows no registry to branch on; a session reads one line and uses
/// whichever spelling its harness offers. Naming two *spellings* is not naming
/// two *targets*: there is still nothing to select.
///
/// **The third measured element is gone with the thing it described.** The
/// provisioned directories rode this instruction by absolute path, and no
/// directory is provisioned any more. The gap is recorded rather than argued
/// away (`docs/specs/module-decomposition.md`, decision 9): a harness with a
/// skill-loading affordance is unaffected, one without loses its fallback, and
/// the reopen condition is a session that cannot reach the methodology by the
/// affordance alone.
const LOAD_INSTRUCTION: &str = "\
**Load the `{skill}` skill now** — on Claude Code, where plugin skills are
namespaced, that is `{plugin}:{skill}`. Your kind is `{kind}`; Grove resolved
that before this session existed, so there is no selection for you to make.

Do this **first**, before anything else you might reach for: before reading the
task file, before running any `grove-llm` verb, before looking at `.grove/`,
before inspecting the working tree, and before answering a question.
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
/// **One text for every kind**, where two embedded files used to serve
/// them, and the trade is deliberate rather than an oversight. What the driver
/// holds — that it is watching a named file, that writing it ends the session,
/// that not writing it stops the loop — is a fact about the process tree no
/// skill can discover and none can repair once the moment has passed, so it
/// rides the one channel a session cannot skip. Which ending a kind takes is a
/// rule about that kind, and the shipped spine already assigns it: *whether it
/// passes the done flag when it signals … is inline in that kind's own
/// `grove-<kind>` skill* (`plugins/grove/skills/grove/SKILL.md`).
///
/// **The skill's ending is the sentence's object, and the default is
/// subordinate to it.** That is load-bearing rather than stylistic. The
/// eighteen-and-one split this replaces meant a `finish` prompt never carried
/// *run `grove-llm complete`* as an imperative — the instruction that, taken by
/// the one session that may have just deleted the task tree, relaunches the loop
/// onto a torn-down grove, and whose stated precondition (*the task is retired
/// and committed*) a completed teardown satisfies exactly. A default stated as
/// the main verb would put that imperative back in every prompt and rest
/// its repair on the skill being read, which is the argument this module's
/// too-late test rejects. Stated as *ordinarily*, subordinate to the kind's own
/// ending, no prompt ends on a bare imperative for the wrong action.
///
/// **The residue that remains, stated rather than argued away.** A `finish`
/// session whose skill is missing or unread still meets *ordinarily it is
/// `grove-llm complete`* and nothing contradicting it, where the old prompt
/// alone was fail-safe for that kind whatever was installed. Decision 10 already
/// accepts that a session can be launched pointing at a skill that is not
/// installed, so the two residues compound. What is bought is that grove
/// interprets no kind; what is paid is that `finish`'s safety now depends on
/// `grove-finish` being present, like every other rule. The reopen condition is a
/// `finish` session observed signalling `complete` after a teardown.
const SIGNALLING_CONTRACT: &str = "\
**Signal.** Your kind's skill states how this session ends, and that ending is
your **last action — then do nothing else**. Ordinarily it is **`grove-llm
complete`**, once the task is retired and committed (and any parent-chain
cascade is settled and included). That is how the self-driving loop ends this
session and starts the next task with fresh context: the verb only writes the
relaunch flag to a signal file (`GROVE_SIGNAL_FILE`) and returns. Ending the
session is the **loop driver's** job, not the verb's — the driver launched this
session and is watching for the signal file while it runs, so it applies grace →
SIGTERM → kill-grace → SIGKILL to its own child once the file appears
(driver-side watcher: the driver can always signal its child, unlike an in-agent
self-kill, which some harness sandboxes silently deny). Run outside a `grove`
loop (no `GROVE_SIGNAL_FILE`) the verb is a safe no-op that just tells you to
exit manually. A session that ends *without* signalling stops the loop instead —
a crash or a Ctrl-C is one such ending, and for at least one kind it is a stated
one — so the signal is what separates a task you finished from a session that
died.
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
    let load = LOAD_INSTRUCTION
        .replace("{skill}", &skill_name(mandate.kind))
        .replace("{plugin}", PLUGIN)
        .replace("{kind}", mandate.kind.label());
    let facts = RUNTIME_FACTS
        .replace("{handle}", &mandate.handle.to_string())
        .replace("{stated_vcs}", &stated_vcs(mandate.workspace))
        .replace("{version}", mandate.version);

    [load, facts, SIGNALLING_CONTRACT.to_string()].join("\n")
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
/// spine's `skill-stated-vcs-is-definitive` states it, and stating it here again
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
