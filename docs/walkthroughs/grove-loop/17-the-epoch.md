# Which calls the lease admits
<!-- book-page id="the-epoch" slice="which-calls-are-admitted" order="17" -->
[Previous: One live driver per working tree](16-the-lease.md) | [Contents](README.md) | [Next: Which files take part](18-which-files.md)

<a id="which-calls-are-admitted"></a>
## The rule: the epoch decides, and a handoff is ordered rather than raced

Chapter 16 read the production half of `driver_lease.rs` and argued it
mechanism by mechanism, because at 12% comment prose the source would not argue
for itself. This chapter reads the other half of the same file, and the
instruction inverts.

> **The epoch decides which `grove-llm` calls a live driver admits, and a
> handoff is ordered rather than raced.** A driver publishes an epoch record
> naming its working tree, its nonce and the one channel it is about to spawn a
> session on. An agent-side call carrying that channel is admitted under a
> *shared* lock it then holds for the whole operation; the driver's invalidation
> takes the same lock *exclusively*. Two locks on one file are the whole ordering
> mechanism, and no deadline is any part of it: an admitted call is never cut
> short. The *waiting* side is bounded — a replacement gives up after thirty
> seconds rather than proceeding — but that bound produces a refusal, not a way
> past the lock.

The [session epoch](../../../CONTEXT.md#session-epoch) is the glossary's name
for that record, and the [guide's account of the driver
lease](../../USAGE.md#usage-driver-lease) is where an operator meets its
symptoms. Neither is where the ordering is decided. It is decided here, in a
`#[cfg(test)]` module, by the admission and launch-ownership scenarios that hold the file's contract in
place — and the contract they hold is the one the decision record
`one-live-driver-per-working-tree` states.

The carried example reaches its most consequential step. A driver is replaced
under a running one.

```text
a driver holds <worktree>, and has spawned a session on
  <worktree>/.jj/grove/signal-1111…

  grove-llm pick, ambient GROVE_SIGNAL_FILE=…/signal-1111…
                                       -> admitted: shared lock on session.epoch,
                                          held for the whole operation

  a replacement driver starts in the same worktree:
                                       -> blocks: invalidation wants the same
                                          file exclusively

  the admitted call returns, its guard drops
                                       -> replacement acquires, rewrites the
                                          record with its own nonce

  the old session calls again:
                                       -> "session epoch is inactive"
```

Nothing in that sketch cuts anything short. The admitted call is not given a
window in which to finish; it finishes, and the replacement waits. The wait is
not unbounded — `EPOCH_HANDOFF_TIMEOUT` is thirty seconds — but reaching that
bound produces a refusal rather than a seizure, so the invariant survives it.
That is what *ordered rather than raced* buys, and the two tests that hold it
are the two this chapter has to run in a subprocess to observe. The bound itself
turns out to be pinned by nothing, which the last section counts.

<a id="eighteen-not-nine"></a>
## Admission tests and launch-ownership controls

The launch-owner controls appear first and are explained below. The
remaining eighteen admission tests are, in file order:
`lease_path_replacement_retries_until_the_locked_descriptor_is_current`,
`lease_path_replacement_fails_closed_after_eight_attempts`,
`acquired_driver_descriptors_are_close_on_exec`,
`activation_and_invalidation_replace_one_stable_epoch_record`,
`epoch_acquisition_retries_open_lock_path_replacement_in_event_order`,
`an_orphaned_epoch_guard_times_out_post_reap_once_at_the_fixed_bound`,
`the_epoch_contention_diagnostic_names_the_lock_mode_and_operation`,
`manual_agent_operations_need_no_driver_epoch`,
`only_a_nonempty_loop_control_value_is_ambient_context`,
`an_admitted_old_operation_finishes_before_replacement_invalidates_new_calls`,
`replacement_keeps_the_old_lease_record_until_it_owns_epoch_handoff`,
`ambient_context_from_another_worktree_names_both_roots`,
`an_inactive_epoch_is_reported_without_claiming_a_session_is_active`,
`a_rotated_epoch_refuses_the_old_signal_path`,
`an_epoch_signal_path_round_trips_record_separator_bytes`,
`a_successful_liveness_probe_releases_the_lease_before_validation`,
`an_active_epoch_without_a_live_lease_is_stale` and
`a_malformed_epoch_is_stale`.

**The book's structure brief says nine, and names one that is not here.** Its
chapter 17 section introduces its list with *the whole inline test module* and
then gives nine names, of which
`an_alias_equivalent_second_owner_is_refused_immediately` is not in this block at
all: it is `crates/grove-loop/tests/driver_lease.rs` line 398, an out-of-process
integration test, and `crates/grove-loop/tests/` is this book's evidence rather
than its corpus. So the brief is wrong in both directions at once — nine where
there were eighteen admission tests, and one name belonging to a file no chapter owns. This page
is the enumeration, and it is the one place a later page should take chapter 17's
test count from; the brief is corrected by its own work item rather than here,
on the precedent chapter 16 set for the same class of error.

The count is not a detail of bookkeeping. This chapter's whole charter is to
say what each reproduced test establishes, so a list that is half the block
would have left nine scenarios unexplained and the original ownership ledger's 564 lines
still claiming to be covered.

<a id="three-per-cent-and-a-blind-instrument"></a>
## Three per cent, and the instrument that cannot see this block

Counting lines whose first non-space characters are `//` — the rule chapter 15
adopted and chapter 16 kept — the original admission block was **19 comment lines in 564**, or 3.4%.
The fair comparison is against the other inline test modules rather than against
production text, and counted the same way they are `task_name.rs` 142 in 694
(20.5%), `task_tree.rs` 158 in 1,008 (15.7%), `tree_lifecycle.rs` 256 in 1,649
(15.5%) and `loop_driver.rs` 6 in 69 (8.7%). This block is the barest of the
five by a factor of two and a half against the next, and the barest thing in the
corpus outright; the production half immediately above it is 12.6%. Every one of
those five agrees with the structure brief's own figures at this rounding, which
is worth recording because chapters 15 and 16 each found a root where they did
not.

Where chapter 16 had comments that argued badly, this chapter has almost no
comments at all, and eighteen test names doing the work an argument should do. A
test name is a label. It asserts nothing, and it cannot be wrong in a way that
fails a build.

Chapter 16 leaned on `cargo doc --no-deps --document-private-items -p
grove-loop`, which reports twenty-six warnings over this crate and names
`driver_lease.rs` in none of them, and it was careful to say that the clean
result was evidence about the seventy of its 103 comment lines written as `///`
and about nothing else. For this block the instrument is not merely narrow. It
is **structurally blind**, and the reason is one attribute.

`cargo doc` compiles with `cfg(test)` off, so every item in this module is
never compiled and never read. The block's nineteen comment lines are four runs:
three written `///` — at lines 822, 839 and 1100 — and one written `//`, at 864.
Only the run at 839 carries intra-doc links, and it carries two — the
rustdoc-linked spellings of `admit_session` and `signal_path_from`. Both resolve,
and no tool in this repository has ever checked that they do.

That was measured rather than assumed, in a copy of the workspace, with a
control in each direction. Planting a link to a nonexistent symbol *inside* this
module leaves the crate at its usual **twenty-six** warnings. Planting the
identical construct in the production module header at the top of the same file
takes it to **twenty-seven**, and the new warning names `driver_lease.rs:2` and quotes the
dangling target. The instrument works; it cannot see here.

That is a second blind spot, orthogonal to the one chapter 16 met. Chapter 16's
was about which *marker* a comment uses — `cargo doc` sees `///` and `//!` and
never plain `//`. This one is about which *cfg* the item sits under, and it takes
out whole files' worth of the corpus at a stroke: the five roots carrying inline
test modules are 3,984 lines, 38% of this book, and `cargo doc` has never read a
byte of any of them. A clean run over this crate is a statement about its
production code.

<a id="the-block-declared"></a>
## The block, declared

The current module is declared here as one composite whose children are the
module's own items in file order: two lines of module opening, six items of
launch-owner controls, scaffolding, the admission tests, and the closing brace. Everything below
reads them in that same order, because a test module has no conceptual order to
prefer to the file's.

<!-- fragment «lease-tests» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="975-2079" parent="source-driver-lease" -->
<!-- insert «epoch-tests-module-open» -->
<!-- insert «epoch-tests-launch-owner» -->
<!-- insert «epoch-tests-workspace-fixture» -->
<!-- insert «epoch-tests-imports» -->
<!-- insert «epoch-tests-ambient-fixture» -->
<!-- insert «epoch-tests-fork-guard» -->
<!-- insert «epoch-tests-replace-locked» -->
<!-- insert «epoch-tests-retry-until-current» -->
<!-- insert «epoch-tests-fails-closed» -->
<!-- insert «epoch-tests-close-on-exec» -->
<!-- insert «epoch-tests-stable-record» -->
<!-- insert «epoch-tests-event-order» -->
<!-- insert «epoch-tests-orphaned-timeout» -->
<!-- insert «epoch-tests-contention-text» -->
<!-- insert «epoch-tests-manual-operations» -->
<!-- insert «epoch-tests-nonempty-ambient» -->
<!-- insert «epoch-tests-old-finishes» -->
<!-- insert «epoch-tests-record-until-handoff» -->
<!-- insert «epoch-tests-foreign-worktree» -->
<!-- insert «epoch-tests-inactive-reported» -->
<!-- insert «epoch-tests-rotated-signal» -->
<!-- insert «epoch-tests-separator-bytes» -->
<!-- insert «epoch-tests-probe-releases» -->
<!-- insert «epoch-tests-active-no-lease» -->
<!-- insert «epoch-tests-malformed» -->
<!-- /fragment -->

<a id="six-items-before-the-first-test"></a>
## Six items before the first test

The module opens on its attribute and its name, and nothing else is on these
two lines.

<!-- fragment «epoch-tests-module-open» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="975-976" parent="lease-tests" -->
````rust
#[cfg(test)]
mod tests {
````
<!-- /fragment -->

The first item is a fixture, and it is declared **above** the imports that
make it compile. `Path` and `Workspace` both arrive from `use super::*` seven
lines below; Rust does not care, and a reader looking for the imports first will
meet a function that appears to reference nothing.

<a id="launch-owner-controls"></a>
## Launch ownership controls

The real-runner control executes a configured shell command and a missing
executable, then injects epoch acquisition and mandatory-write failures. Every
unlaunched or reaped attempt must release both witnesses. The unwind control keeps the
lease outside the panic boundary: Started-only unwind retains the pin, whereas
Reaped releases it before a later panic. A replaced root cannot overwrite an
unreaped pin on a second preparation attempt.

The wait control holds a real shared epoch guard and waits for the writer's
contention notification. It replaces the task root before releasing that guard;
the waiting preparation must refuse activation. This catches a check performed
only before acquisition, without using elapsed time as proof of contention.
The event table separately checks no events, Started only, and Started/Reaped,
including an error return and subsequent epoch invalidation.

Independent shared probes now check both witness descriptors at Started, Reaped,
failed spawn and lease drop. Foreign directory contention leaves a real configured
launch and admission usable. The wait barrier also proves neither witness is
prepared while an old reader holds the epoch. The private-before-directory
release operation is exercised in the paired-witness controls at the end of this
chapter. Two-platform process-death and forced-reuse observation evidence remains
owned by the later observer increment.

The publication controls compare the epoch's tree and private identities with
actual descriptor metadata, decode its namespace-local basename, and check the
nonce/signal binding. Admission accepts absent, unsupported and malformed
extensions, including invalid UTF-8, while rejecting wrong signal paths and
corrupt mandatory path fields. A limited writer fails partway through the real
extension serializer to prove mandatory activation survives. The real-run event
control writes a completion signal at Started and checks that both locks remain
held until Reaped; successful spawn leaves exactly eight bytes, failed spawn none.

<!-- fragment «epoch-tests-launch-owner» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="977-1517" parent="lease-tests" -->
````rust
    pub(super) fn witness_selection() -> crate::Selection {
        crate::Selection {
            path: PathBuf::from(".grove/01-impl--work-k1.md"),
            handle: crate::Handle::parse("work-k1").unwrap(),
            kind: crate::Kind::new("impl").unwrap(),
        }
    }

    #[test]
    fn witnessed_epoch_binds_open_objects_and_admission_ignores_extensions() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        fs::write(temp.path().join(".grove/_BRIEF.md"), "root").unwrap();
        let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let tree = root.directory().metadata().unwrap();
        let signal = lease.control_dir.join("signal-test\nnonce=injected");
        lease
            .prepare_launch(root, &witness_selection(), &signal)
            .unwrap();
        let epoch_path = lease.control_dir.join(EPOCH_FILE_NAME);
        let record = fs::read_to_string(&epoch_path).unwrap();
        let path = lease.launch.as_ref().unwrap().path().unwrap();
        let private = fs::metadata(path).unwrap();
        for (field, expected) in [
            ("observation-tree-device", tree.dev()),
            ("observation-tree-inode", tree.ino()),
            ("observation-witness-device", private.dev()),
            ("observation-witness-inode", private.ino()),
        ] {
            assert_eq!(
                record_field(&record, field)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
                expected
            );
        }
        let basename =
            decode_path(record_field(&record, "observation-witness-name-hex").unwrap()).unwrap();
        assert_eq!(basename.components().count(), 1);
        assert_eq!(lease.control_dir.join(basename), path);
        let mandatory = parse_epoch_record(&record).unwrap();
        assert_eq!(mandatory.process.nonce, lease.nonce);
        assert_eq!(mandatory.signal_path.as_deref(), Some(signal.as_path()));
        lease.supervise_launch(|event| event(keyed_launch::LaunchEvent::Started));
        assert!(matches!(
            crate::try_observe(temp.path(), &[None]).activity,
            crate::ActivityObservation::Running(_)
        ));
        let prefix = record.split("observation-version=").next().unwrap();
        for extension in ["", "observation-version=999\n", "observation-version=1\nobservation-key=bad\nobservation-key=2\nobservation-handle-hex=zz\n"] {
            fs::write(&epoch_path, format!("{prefix}{extension}")).unwrap();
            assert!(admit_session(temp.path(), "test", ambient(&signal)).is_ok(), "{extension}");
            assert!(admit_session(temp.path(), "test", ambient(&signal.with_extension("stale"))).is_err());
        }
        let mut bytes = prefix.as_bytes().to_vec();
        bytes.extend_from_slice(b"observation-kind-hex=\xff\n");
        fs::write(&epoch_path, bytes).unwrap();
        assert!(admit_session(temp.path(), "test", ambient(&signal)).is_ok());
        for field in ["worktree-path-hex", "signal-path-hex"] {
            let valid = record_field(prefix, field).unwrap();
            let corrupt =
                prefix.replace(&format!("{field}={valid}"), &format!("{field}=\u{fffd}0"));
            fs::write(&epoch_path, corrupt.as_bytes()).unwrap();
            assert!(admit_session(temp.path(), "test", ambient(&signal)).is_err());
            let mut corrupt_bytes = prefix
                .replace(&format!("{field}={valid}"), &format!("{field}=X0"))
                .into_bytes();
            let offset = corrupt_bytes
                .windows(field.len() + 2)
                .position(|bytes| bytes == format!("{field}=X").as_bytes())
                .unwrap();
            corrupt_bytes[offset + field.len() + 1] = 0xff;
            fs::write(&epoch_path, corrupt_bytes).unwrap();
            assert!(admit_session(temp.path(), "test", ambient(&signal)).is_err());
        }
    }

    #[test]
    fn witnessed_epoch_partial_publication_preserves_mandatory_activation() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
        let signal = lease.control_dir.join("signal-test");
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        lease
            .prepare_launch_using(
                root,
                &signal,
                |path| acquire_epoch_file(path, LockMode::Exclusive, "test"),
                witnesses::LaunchWitnesses::prepare,
                |launch, epoch| {
                    // Exhaust capacity partway through the real extension serializer.
                    struct Limited<'a>(&'a mut File, usize);
                    impl Write for Limited<'_> {
                        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                            if self.1 == 0 {
                                return Err(std::io::Error::other("injected full device"));
                            }
                            let n = self.0.write(&bytes[..bytes.len().min(self.1)])?;
                            self.1 -= n;
                            Ok(n)
                        }
                        fn flush(&mut self) -> std::io::Result<()> {
                            self.0.flush()
                        }
                    }
                    launch.publish(&mut Limited(epoch, 30), &witness_selection())
                },
            )
            .unwrap();
        assert!(admit_session(temp.path(), "test", ambient(&signal)).is_ok());
        let record = fs::read_to_string(lease.control_dir.join(EPOCH_FILE_NAME)).unwrap();
        assert!(
            record.ends_with("observation-version=1\nobservat"),
            "{record}"
        );
        let result = lease.supervise_launch(|event| {
            event(keyed_launch::LaunchEvent::Started);
            event(keyed_launch::LaunchEvent::Reaped);
            42
        });
        assert_eq!(result, 42);
        assert!(lease.launch.is_none());
    }

    #[test]
    fn paired_witness_failure_preserves_real_launch_and_admission() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        fs::write(temp.path().join(".grove/_BRIEF.md"), "root").unwrap();
        let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
        let directory = File::open(temp.path().join(".grove")).unwrap();
        assert_eq!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let channel = keyed_launch::Channel::allocate(&lease.control_dir).unwrap();
        lease
            .prepare_launch(root, &witness_selection(), channel.path())
            .unwrap();
        assert!(lease.launch.as_ref().unwrap().path().is_none());
        assert!(admit_session(temp.path(), "test", ambient(channel.path())).is_ok());
        assert!(matches!(
            crate::try_observe(temp.path(), &[None]).activity,
            crate::ActivityObservation::Unavailable(_)
        ));
        let config = temp.path().join("launch.kdl");
        fs::write(&config, "config { command \"run\" \"/bin/sh -c 'echo launched > proof'\"; bind \"run\" \"run\"; route \"test\" \"run\"; }\n").unwrap();
        let templates =
            keyed_launch::Templates::load(&config, None, keyed_launch::Vocabulary { slots: &[] })
                .unwrap();
        let argv = templates.expand("test", &[]).unwrap();
        lease
            .supervise_launch(|observer| {
                keyed_launch::run_observed(
                    keyed_launch::Launch {
                        argv: &argv,
                        channel: &channel,
                        channel_var: "GROVE_SIGNAL_FILE",
                        scrub: &[],
                        cwd: Some(temp.path()),
                        escalation: keyed_launch::Escalation {
                            grace: Duration::ZERO,
                            kill_grace: Duration::ZERO,
                        },
                    },
                    observer,
                )
            })
            .unwrap();
        assert_eq!(fs::read(temp.path().join("proof")).unwrap(), b"launched\n");
        assert!(lease.launch.is_none());
        lease.invalidate_session_epoch().unwrap();
        channel.discard().unwrap();
    }

    #[test]
    fn paired_witness_lease_drop_releases_both_before_replacement_cleanup() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        for unwind in [false, true] {
            let temp = TempDir::new().unwrap();
            fs::create_dir(temp.path().join(".jj")).unwrap();
            fs::create_dir(temp.path().join(".grove")).unwrap();
            let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
            let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
            let signal = lease.control_dir.join("signal-test");
            lease
                .prepare_launch(root, &witness_selection(), &signal)
                .unwrap();
            let path = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
            let directory = File::open(temp.path().join(".grove")).unwrap();
            let private = File::open(&path).unwrap();
            lease.supervise_launch(|event| event(keyed_launch::LaunchEvent::Started));
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                let _lease = lease;
                assert!(!unwind, "injected lease unwind");
            }));
            assert_eq!(result.is_err(), unwind);
            for file in [&directory, &private] {
                assert_eq!(
                    unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
                    0
                );
            }
            assert!(path.exists());
            let replacement = DriverLease::acquire_with(&workspace_at(temp.path()), || {
                assert!(
                    path.exists(),
                    "replacement cannot clean before epoch handoff"
                );
            })
            .unwrap();
            assert!(!path.exists());
            assert!(
                fs::read_to_string(replacement.control_dir.join(EPOCH_FILE_NAME))
                    .unwrap()
                    .starts_with("state=inactive\n")
            );
        }
    }

    #[test]
    fn paired_witnesses_are_owned_until_reap() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        fs::write(temp.path().join(".grove/_BRIEF.md"), "root").unwrap();
        let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let signal = lease.control_dir.join("signal-test");
        lease
            .prepare_launch(root, &witness_selection(), &signal)
            .unwrap();
        let directory = File::open(temp.path().join(".grove")).unwrap();
        assert_ne!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0,
            "prepared directory must be exclusively witnessed"
        );
        let path = fs::read_dir(&lease.control_dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("witness-")
            })
            .expect("a fresh private witness");
        let private = File::open(&path).unwrap();
        assert_ne!(
            unsafe { libc::flock(private.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        assert!(fs::read(&path).unwrap().is_empty());
        assert!(matches!(
            crate::try_observe(temp.path(), &[None]).activity,
            crate::ActivityObservation::Busy(_)
        ));
        // The directory witness must not take the containing-directory tree lock.
        assert!(matches!(
            crate::try_read(temp.path()).unwrap(),
            crate::TryReading::Ready(_)
        ));
        lease.supervise_launch(|event| event(keyed_launch::LaunchEvent::Started));
        lease.invalidate_session_epoch().unwrap();
        assert!(
            path.exists(),
            "unconfirmed reap retains witness even across invalidation"
        );
        assert_ne!(
            unsafe { libc::flock(private.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        lease.supervise_launch(|event| event(keyed_launch::LaunchEvent::Reaped));
        assert_eq!(
            unsafe { libc::flock(private.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        assert_eq!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        assert!(path.exists(), "reap must not clean before invalidation");
        lease.invalidate_session_epoch().unwrap();
        assert!(!path.exists());
    }

    #[test]
    fn lease_root_owner_real_spawn_and_preparation_failure_release() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        for program in ["/bin/sh", "/no-such-grove-test-program"] {
            let temp = TempDir::new().unwrap();
            fs::create_dir(temp.path().join(".jj")).unwrap();
            fs::create_dir(temp.path().join(".grove")).unwrap();
            let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
            let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
            let channel = keyed_launch::Channel::allocate(&lease.control_dir).unwrap();
            lease
                .prepare_launch(root, &witness_selection(), channel.path())
                .unwrap();
            let directory = File::open(temp.path().join(".grove")).unwrap();
            let private_path = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
            let private = File::open(&private_path).unwrap();
            let config = temp.path().join("launch.kdl");
            fs::write(&config, format!("config {{ command \"run\" \"{program} -c true\"; bind \"run\" \"run\"; route \"test\" \"run\"; }}\n")).unwrap();
            let templates = keyed_launch::Templates::load(
                &config,
                None,
                keyed_launch::Vocabulary { slots: &[] },
            )
            .unwrap();
            let argv = templates.expand("test", &[]).unwrap();
            let result = lease.supervise_launch(|observer| {
                keyed_launch::run_observed(
                    keyed_launch::Launch {
                        argv: &argv,
                        channel: &channel,
                        channel_var: "GROVE_SIGNAL_FILE",
                        scrub: &[],
                        cwd: Some(temp.path()),
                        escalation: keyed_launch::Escalation {
                            grace: Duration::ZERO,
                            kill_grace: Duration::ZERO,
                        },
                    },
                    &mut |event| {
                        observer(event);
                        assert_eq!(fs::read(&private_path).unwrap(), b"started\n");
                        if event == keyed_launch::LaunchEvent::Started {
                            keyed_launch::signal(channel.path(), "relaunch").unwrap();
                            assert!(channel.path().exists());
                        }
                        for file in [&directory, &private] {
                            let result = unsafe {
                                libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB)
                            };
                            if event == keyed_launch::LaunchEvent::Started {
                                assert_ne!(result, 0);
                            } else {
                                assert_eq!(result, 0, "Reaped releases before returning to runner");
                                assert_eq!(
                                    unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) },
                                    0
                                );
                            }
                        }
                    },
                )
            });
            assert_eq!(result.is_ok(), program == "/bin/sh", "{result:?}");
            assert!(lease.launch.is_none(), "{program}");
            for file in [&directory, &private] {
                assert_eq!(
                    unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
                    0
                );
                assert_eq!(unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) }, 0);
            }
            assert_eq!(
                fs::read(&private_path).unwrap(),
                if program == "/bin/sh" {
                    b"started\n".as_slice()
                } else {
                    b""
                }
            );

            let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
            let result =
                lease.prepare_launch_with(root, &witness_selection(), channel.path(), |_| {
                    Err(anyhow::anyhow!("injected epoch acquisition failure"))
                });
            assert!(result.is_err());
            assert!(lease.launch.is_none());
            let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
            let result =
                lease.prepare_launch_with(root, &witness_selection(), channel.path(), |path| {
                    // A read-only descriptor makes the mandatory epoch write fail.
                    Ok(File::open(path)?)
                });
            assert!(result.is_err());
            assert!(lease.launch.is_none());
            channel.discard().unwrap();
        }
    }

    #[test]
    fn lease_root_owner_unwind_retains_until_lease_drop() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let workspace = workspace_at(temp.path());
        let mut lease = DriverLease::acquire(&workspace).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let signal = lease.control_dir.join("signal-test");
        lease
            .prepare_launch(root, &witness_selection(), &signal)
            .unwrap();
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            lease.supervise_launch(|observer| {
                observer(keyed_launch::LaunchEvent::Started);
                panic!("injected runner unwind");
            });
        }));
        assert!(unwind.is_err());
        assert!(lease.launch.is_some());
        let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            lease.supervise_launch(|observer| {
                observer(keyed_launch::LaunchEvent::Reaped);
                panic!("failure after confirmed reap, before helper return");
            });
        }));
        assert!(unwind.is_err());
        assert!(lease.launch.is_none());
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        lease
            .prepare_launch(root, &witness_selection(), &signal)
            .unwrap();
        let original = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        fs::rename(temp.path().join(".grove"), temp.path().join("old")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let replacement_pin = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        assert!(lease
            .prepare_launch(replacement_pin, &witness_selection(), &signal)
            .is_err());
        assert!(
            lease.launch.as_ref().unwrap().root.same(&original).unwrap(),
            "a second attempt replaced the unreaped pin"
        );
        assert!(DriverLease::acquire(&workspace).is_err());
        drop(lease);
        assert!(DriverLease::acquire(&workspace).is_ok());
    }

    #[test]
    fn lease_root_owner_rechecks_after_epoch_wait() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let signal = lease.control_dir.join("signal-test");
        let epoch_path = lease.control_dir.join(EPOCH_FILE_NAME);
        let reader = acquire_epoch_file(&epoch_path, LockMode::Shared, "test reader").unwrap();
        let (waiting_tx, waiting_rx) = mpsc::channel();
        let worker = thread::spawn(move || {
            let result = lease.prepare_launch_with(root, &witness_selection(), &signal, |path| {
                acquire_epoch_file_with(
                    path,
                    LockMode::Exclusive,
                    "test preparation",
                    Duration::from_secs(5),
                    Instant::now,
                    thread::yield_now,
                    |_, _| Ok(()),
                    |_, _| Ok(()),
                    || waiting_tx.send(()).unwrap(),
                )
            });
            (lease, result)
        });
        waiting_rx.recv_timeout(Duration::from_secs(5)).unwrap();
        let directory = File::open(temp.path().join(".grove")).unwrap();
        assert_eq!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0,
            "preparation waiting on an old epoch must not acquire the directory witness"
        );
        assert_eq!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_UN) },
            0
        );
        assert!(fs::read_dir(temp.path().join(".jj/grove"))
            .unwrap()
            .all(|entry| !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with("witness-")));
        fs::rename(temp.path().join(".grove"), temp.path().join("old")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        drop(reader);
        let (lease, result) = worker.join().unwrap();
        assert!(format!("{:#}", result.unwrap_err()).contains("task tree changed"));
        assert!(lease.launch.is_none());
        assert!(fs::read_to_string(epoch_path)
            .unwrap()
            .starts_with("state=inactive\n"));
    }

    #[test]
    fn lease_root_owner_retains_unconfirmed_reap_and_releases_other_returns() {
        for events in [
            vec![],
            vec![keyed_launch::LaunchEvent::Started],
            vec![
                keyed_launch::LaunchEvent::Started,
                keyed_launch::LaunchEvent::Reaped,
            ],
        ] {
            let temp = TempDir::new().unwrap();
            fs::create_dir(temp.path().join(".jj")).unwrap();
            fs::create_dir(temp.path().join(".grove")).unwrap();
            let mut lease = DriverLease::acquire(&workspace_at(temp.path())).unwrap();
            let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
            let signal = lease.control_dir.join("signal-test");
            lease
                .prepare_launch(root, &witness_selection(), &signal)
                .unwrap();
            let result: Result<()> = lease.supervise_launch(|observer| {
                for event in &events {
                    observer(*event);
                }
                Err(anyhow::anyhow!("injected supervisor failure"))
            });
            assert!(result.is_err());
            let unconfirmed = events == [keyed_launch::LaunchEvent::Started];
            assert_eq!(lease.launch.is_some(), unconfirmed, "{events:?}");
            // Neither helper return nor epoch invalidation is evidence of reap.
            lease.invalidate_session_epoch().unwrap();
            assert_eq!(lease.launch.is_some(), unconfirmed);
        }
    }

````
<!-- /fragment -->

`workspace_at` resolves the fixture marker into the same Workspace value the
real lease acquisition accepts. It creates no launch configuration.

<!-- fragment «epoch-tests-workspace-fixture» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1518-1524" parent="lease-tests" -->
````rust
    /// The fixtures below build a `.jj` marker directly and then acquire against
    /// it. Resolving here rather than inside `acquire` is the shape of the
    /// change this leaf made: the lease takes a workspace it did not resolve.
    fn workspace_at(path: &Path) -> Workspace {
        Workspace::resolve(path).expect("a fixture worktree carries a `.jj` marker")
    }

````
<!-- /fragment -->

Its doc comment records a change rather than a rule: the lease *takes* a
workspace it did not resolve. That is the same claim chapter 16 read from the
production header and confirmed at `crates/grove/src/cli.rs` — one resolution,
handed to everything — and here it is what forces every fixture below to build a
`.jj` marker by hand before it can acquire anything. Thirteen of the original admission tests
build a `.jj` directory by hand before they can acquire anything — ten spell it
`root.join(".jj")`, one builds two of them because it needs two worktrees, and
two go straight to `.jj/grove` because they drive the lease-file helper without a
workspace at all. The five that do not are the three that touch no filesystem
fixture and the two that drive the epoch helper against a bare temporary file.

Then the imports, and one constant.

<!-- fragment «epoch-tests-imports» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1525-1534" parent="lease-tests" -->
````rust
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::process::Command;
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Duration, Instant};
    use tempfile::TempDir;

    const FORK_SENSITIVE_TEST: &str = "GROVE_DRIVER_LEASE_FORK_SENSITIVE_TEST";

````
<!-- /fragment -->

`Cell` and `RefCell` are here for the injected clocks and event logs two
scenarios use; `mpsc` and `thread` for the two that need a second thread to
contend with; `Command` for the one that needs a second *process*. The constant
names an environment variable, and it is the only variable this module ever sets
— it is a private handshake between a parent test and its own child, and it is
not `GROVE_SIGNAL_FILE`.

That distinction is the next item's whole subject.

<!-- fragment «epoch-tests-ambient-fixture» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1535-1544" parent="lease-tests" -->
````rust
    /// The ambient context an admission test would once have installed by
    /// writing `GROVE_SIGNAL_FILE`. Nothing here mutates the environment: these
    /// tests exercise [`admit_session`], whose ambient path is an argument, and
    /// the reading half above is covered separately — purely by
    /// [`signal_path_from`], and end to end by `tests/driver_lease.rs`, which
    /// sets the real variable on a real `grove-llm` subprocess.
    fn ambient(path: &Path) -> Option<PathBuf> {
        Some(path.to_path_buf())
    }

````
<!-- /fragment -->

**This is the fixture that makes the admission tests possible in process, and
the doc comment states why.** `admit_session` takes the resolved ambient path as
an *argument*; only its public wrapper `admit_ambient_session` reads the
environment. So a test can supply ambient context by constructing an `Option` —
no process-global is written, and a parallel sibling test cannot observe a
variable this one set.

The consequence is worth stating precisely, because it is easy to state
wrongly. `.cargo/config.toml` force-clears `GROVE_SIGNAL_FILE` to the empty
string for everything cargo runs, so under the suite `ambient_signal_path` can
only ever return `None`, and therefore `admit_ambient_session` — the crate's
public entry point, and the only one `crates/grove-llm/src/cli.rs` line 421 ever
calls — can only ever return `Ok(None)`. **No test in this workspace calls it.**
Every admission scenario here calls the private `admit_session` instead. What is
unobservable in process is the environment-reading wrapper, and that is a guard
rather than a gap: the split exists so the reading half can be pinned separately,
which the ninth test below does directly. The end-to-end path is covered where it
has to be, out of process, by the `tests/driver_lease.rs` this comment names —
the same file the structure brief mistook for part of this block.

The next item is the reason three of the original admission tests are not really run by
the test harness at all.

<!-- fragment «epoch-tests-fork-guard» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1545-1580" parent="lease-tests" -->
````rust
    pub(super) fn fork_sensitive_driver_lease_test_body_runs_here() -> bool {
        let current_thread = thread::current();
        let test_name = current_thread
            .name()
            .expect("the Rust test harness names every test thread");
        let arguments: Vec<_> = std::env::args_os().collect();
        let is_isolated_child = std::env::var_os(FORK_SENSITIVE_TEST)
            == Some(OsString::from(test_name))
            && arguments
                .windows(2)
                .any(|pair| pair[0] == "--exact" && pair[1] == test_name);
        if is_isolated_child {
            return true;
        }

        // flock locks survive fork until the child execs. A parallel unit test
        // that launches a subprocess can therefore extend this test's lease
        // briefly after its owner drops, even though every descriptor is
        // close-on-exec. Re-run only the fork-sensitive assertion in a child
        // test process with no parallel siblings; the production ordering and
        // lock assertions remain unchanged inside that process.
        let output = Command::new(std::env::current_exe().expect("locating the unit-test binary"))
            .args(["--exact", test_name, "--nocapture"])
            .env(FORK_SENSITIVE_TEST, test_name)
            .output()
            .expect("launching the isolated driver-lease test");
        assert!(
            output.status.success(),
            "isolated driver-lease test {test_name} failed with {}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        false
    }

````
<!-- /fragment -->

**The guard splits one test into two processes, and only one of them asserts
anything about the lease.** Called in the ordinary harness process it returns
`false`, and its caller returns immediately, having asserted nothing; before
returning it re-executes the test binary with `--exact <this test>` and the
private variable set, and asserts only that the child exited successfully.
Called inside that child both conditions hold, it returns `true`, and the real
body runs with no parallel siblings.

The reason is in the plain-`//` comment, and it is a fact about `flock` rather
than about grove: locks survive `fork` until the child `exec`s, so any sibling
test that spawns a subprocess briefly extends this test's lease past the drop
that was supposed to release it. Chapter 16's close-on-exec discipline does not
help, because the window is between the two calls.

What this costs is diagnostic, and it is worth knowing before reading a
failure. The parent's only assertion is `output.status.success()`, so a failure
inside the child arrives as an exit status with the child's captured streams
interpolated into the message. Mutated in a workspace copy — a bare panic
inserted at the top of one such body — the run reports the parent failing at line
875 with `exit status: 101`, and the child's own panic at its real line replayed
underneath. Both frames are present; neither is where a reader would first look.

The last item before the tests is a two-line hook shared by the first two.

<!-- fragment «epoch-tests-replace-locked» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1581-1586" parent="lease-tests" -->
````rust
    fn replace_locked_path(attempt: usize, path: &Path) -> Result<()> {
        fs::rename(path, path.with_extension(format!("attempt-{attempt}")))?;
        fs::write(path, format!("replacement {attempt}"))?;
        Ok(())
    }

````
<!-- /fragment -->

It renames the file out from under a descriptor that is already open and
writes a different file at the same path — the exact race
`acquire_lease_file_with_hook` retries against, performed deliberately.

<a id="one-retry-loop-two-tests"></a>
## One retry loop, two tests, and where the number 8 is actually pinned

The first pair drives chapter 16's lease-file acquisition through its identity
retry loop, once to success and once to exhaustion. The hook fires after the lock
is taken and before the identity comparison.

<!-- fragment «epoch-tests-retry-until-current» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1587-1611" parent="lease-tests" -->
````rust
    #[test]
    fn lease_path_replacement_retries_until_the_locked_descriptor_is_current() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        let control = root.join(".jj/grove");
        fs::create_dir_all(&control).unwrap();
        let path = control.join(LEASE_FILE_NAME);
        let mut observed_attempts = 0;

        let (_file, identity) = acquire_lease_file_with_hook(&path, &root, |attempt, path| {
            observed_attempts = attempt;
            if attempt < 3 {
                replace_locked_path(attempt, path)?;
            }
            Ok(())
        })
        .unwrap();

        assert_eq!(observed_attempts, 3);
        assert_eq!(
            identity,
            FileIdentity::from_metadata(&fs::metadata(&path).unwrap())
        );
    }

````
<!-- /fragment -->

**What it establishes.** The loop retries when the locked descriptor is no
longer the file at the path, and stops on the first attempt where they agree: the
hook replaces the file on attempts 1 and 2, does nothing on attempt 3, and
`observed_attempts` is 3.

**What it would still pass under.** The returned file is bound to `_file` and
discarded, so no property of the descriptor is asserted — not that it is locked,
not that it is the descriptor the identity was read from. The identity assertion
compares the returned `FileIdentity` against `fs::metadata` of the path *now*,
which is the same comparison the production loop just made in order to exit; an
implementation that computed both sides from the path rather than one from the
descriptor would satisfy it exactly. What the test pins is the loop's **control
flow** — that it iterates on disagreement and stops on agreement — and not that
the agreement means what the function's name says it means.

The second reaches the bound.

<!-- fragment «epoch-tests-fails-closed» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1612-1635" parent="lease-tests" -->
````rust
    #[test]
    fn lease_path_replacement_fails_closed_after_eight_attempts() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        let control = root.join(".jj/grove");
        fs::create_dir_all(&control).unwrap();
        let path = control.join(LEASE_FILE_NAME);
        let mut observed_attempts = 0;

        let error = acquire_lease_file_with_hook(&path, &root, |attempt, path| {
            observed_attempts = attempt;
            replace_locked_path(attempt, path)
        })
        .unwrap_err();

        assert_eq!(observed_attempts, IDENTITY_RETRY_LIMIT);
        assert!(
            error
                .to_string()
                .contains("was replaced during acquisition 8 times"),
            "unexpected error: {error:#}"
        );
    }

````
<!-- /fragment -->

**What it establishes.** The hook replaces the file on every attempt, so
agreement never happens; the loop stops after `IDENTITY_RETRY_LIMIT` attempts and
fails rather than looping or succeeding. The error is the identity-exhaustion
one, not a lock error.

**What it would still pass under, and the thing worth noticing.** The count
assertion is `assert_eq!(observed_attempts, IDENTITY_RETRY_LIMIT)` — both sides
move together, so it holds for any limit at all. The number 8 is pinned **only by
the string literal** `"was replaced during acquisition 8 times"`, which is
compared against a message that renders the constant. Change the constant to five
and the count assertion still passes while the message assertion fails. The test
is half-written against the symbol and half against its value, and the half that
guards the value is the one that looks like prose.

That pattern recurs below, in a test where it comes out the other way.

<a id="two-descriptors-and-the-two-it-does-not-check"></a>
## Two descriptors, and the two it does not check

One test, and it is the only one in the block that reads a descriptor flag
rather than a file's contents.

<!-- fragment «epoch-tests-close-on-exec» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1636-1657" parent="lease-tests" -->
````rust
    #[test]
    fn acquired_driver_descriptors_are_close_on_exec() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();

        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();

        for (label, descriptor) in [
            ("working-tree root", lease._worktree_directory.as_raw_fd()),
            ("driver lease", lease.lease_file.as_raw_fd()),
        ] {
            let flags = unsafe { libc::fcntl(descriptor, libc::F_GETFD) };
            assert_ne!(flags, -1, "reading {label} descriptor flags failed");
            assert_ne!(
                flags & libc::FD_CLOEXEC,
                0,
                "{label} descriptor can leak across exec"
            );
        }
    }

````
<!-- /fragment -->

**What it establishes.** Both descriptors a `DriverLease` holds — the working
tree root directory and the lease file — carry `FD_CLOEXEC`, read back through
`fcntl(F_GETFD)` rather than trusted from the setting call. The loop labels each
one, so a failure names which.

**What it would still pass under.** `ensure_close_on_exec` has four call sites
in this file: the working-tree directory at line 163, the epoch file at 391, the
lease file at 473, and the probe's second opening of the lease file at 654. This
test reaches the first and third, which is exactly what `DriverLease` keeps as
fields — and those two are the ones live across the driver's spawn, so the test
is complete for the name it carries. The other two are marked by the same helper
and asserted by nothing. One of them matters more than it looks: the epoch
descriptor at 391 is what a `SessionEpochGuard` holds for the whole of an
admitted operation, which is a real lifetime rather than a transient. Chapter 16
counted five opens and four marks, the unmarked fifth being `/dev/urandom`; the
symmetric statement here is that of the four that *are* marked, two are proved
and two are asserted nowhere in the corpus.

<a id="one-file-rewritten"></a>
## One file, rewritten and never replaced

The epoch record is the one file both sides of the handoff hold open at once,
and this test is about the file rather than about what it says.

<!-- fragment «epoch-tests-stable-record» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1658-1696" parent="lease-tests" -->
````rust
    #[test]
    fn activation_and_invalidation_replace_one_stable_epoch_record() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let epoch_path = root.join(".jj/grove").join(EPOCH_FILE_NAME);
        let epoch_identity = FileIdentity::from_metadata(&fs::metadata(&epoch_path).unwrap());
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");

        lease.activate_session_epoch(&signal_path).unwrap();

        let active = fs::read_to_string(&epoch_path).unwrap();
        assert!(active.starts_with("state=active\n"), "{active:?}");
        assert!(
            active.contains(&format!(
                "signal-path-hex={}\n",
                encode_path(&signal_path).unwrap()
            )),
            "{active:?}"
        );
        assert_eq!(
            FileIdentity::from_metadata(&fs::metadata(&epoch_path).unwrap()),
            epoch_identity,
            "activation must rewrite the stable epoch file rather than replace it"
        );

        lease.invalidate_session_epoch().unwrap();

        let inactive = fs::read_to_string(&epoch_path).unwrap();
        assert!(inactive.starts_with("state=inactive\n"), "{inactive:?}");
        assert!(!inactive.contains("signal-path-hex="), "{inactive:?}");
        assert_eq!(
            FileIdentity::from_metadata(&fs::metadata(&epoch_path).unwrap()),
            epoch_identity,
            "invalidation must keep the same stable epoch file"
        );
    }

````
<!-- /fragment -->

**What it establishes.** Activation writes `state=active` and a
`signal-path-hex=` line; invalidation writes `state=inactive` and no such line;
and across both the epoch file's device and inode are unchanged. That last
assertion is the point, and it is made twice: the record is *rewritten in place*,
never replaced, so a reader holding the file open by descriptor is looking at the
same object the writer just changed. Every shared-lock admission below depends on
it — a replaced file would leave a lock held on an orphan.

**What it would still pass under.** The four fields the record carries besides
`state` and the signal path — device, inode, path-hex, nonce — are never read
back, so a rewrite that dropped or corrupted any of them satisfies this test.
Truncation is pinned, but only in one direction and only by an accident of field
order: the active record is the inactive one plus a trailing `signal-path-hex=`
line, so invalidating without `set_len(0)` would leave that line behind, and
`assert!(!inactive.contains("signal-path-hex="))` catches it. Going the other way
the new record is longer, so nothing would be left over, and a missing truncation
on activation would pass unnoticed.

<a id="how-the-epoch-file-is-taken"></a>
## How the epoch file is taken: an order, a bound, and a sentence

Three tests drive `acquire_epoch_file_with`, which is the seam chapter 16
described as the one the decision record reserves: a clock, a wait, two barriers
and a contention reporter, all injected. This chapter says what each is used for.

<!-- fragment «epoch-tests-event-order» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1697-1737" parent="lease-tests" -->
````rust
    #[test]
    fn epoch_acquisition_retries_open_lock_path_replacement_in_event_order() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join(EPOCH_FILE_NAME);
        fs::write(&path, "old epoch\n").unwrap();
        let events = RefCell::new(Vec::new());
        let start = Instant::now();

        let guard = acquire_epoch_file_with(
            &path,
            LockMode::Exclusive,
            "test replacement race",
            Duration::from_secs(30),
            || start,
            || {},
            |attempt, path| {
                events.borrow_mut().push(format!("open-{attempt}"));
                if attempt == 1 {
                    fs::rename(path, path.with_extension("old"))?;
                    fs::write(path, "replacement epoch\n")?;
                }
                Ok(())
            },
            |attempt, _| {
                events.borrow_mut().push(format!("lock-{attempt}"));
                Ok(())
            },
            || events.borrow_mut().push("contended".to_string()),
        )
        .unwrap();

        assert_eq!(
            events.into_inner(),
            ["open-1", "lock-1", "open-2", "lock-2"]
        );
        assert_eq!(
            FileIdentity::from_metadata(&guard.metadata().unwrap()),
            FileIdentity::from_metadata(&fs::metadata(&path).unwrap())
        );
    }

````
<!-- /fragment -->

**What it establishes.** The two barriers interleave in a fixed order across a
retry — open, lock, open, lock — so the file is re-opened, not merely re-locked,
when the identity check fails. The `open-1` hook renames the file away and writes
a replacement, which is what forces the second pass. The final assertion is that
the guard's own metadata matches the path's, so the descriptor returned is the
one that won.

**What it would still pass under.** The clock is `|| start`, a constant, so the
deadline is never reached and the timeout branch is never entered; `wait` does
nothing. The contention reporter pushes `"contended"`, and the expected vector
does not contain it — so the test also pins, quietly, that an uncontended
acquisition reports nothing, and that is the only assertion in the block about
the reporter's *call site* rather than its text.

<!-- fragment «epoch-tests-orphaned-timeout» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1738-1776" parent="lease-tests" -->
````rust
    #[test]
    fn an_orphaned_epoch_guard_times_out_post_reap_once_at_the_fixed_bound() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join(EPOCH_FILE_NAME);
        fs::write(&path, "active epoch\n").unwrap();
        let owner = File::open(&path).unwrap();
        assert_eq!(unsafe { libc::flock(owner.as_raw_fd(), libc::LOCK_EX) }, 0);
        let elapsed = Cell::new(Duration::ZERO);
        let contention_reports = Cell::new(0);
        let start = Instant::now();

        let error = acquire_epoch_file_with(
            &path,
            LockMode::Exclusive,
            "post-reap session epoch invalidation",
            Duration::from_secs(30),
            || start + elapsed.get(),
            || elapsed.set(elapsed.get() + Duration::from_secs(10)),
            |_, _| Ok(()),
            |_, _| Ok(()),
            || contention_reports.set(contention_reports.get() + 1),
        )
        .unwrap_err();

        assert_eq!(contention_reports.get(), 1);
        assert_eq!(elapsed.get(), Duration::from_secs(30));
        assert!(
            error.to_string().contains(
                "timed out after 30s waiting for exclusive session epoch lock for post-reap session epoch invalidation"
            ),
            "unexpected error: {error:#}"
        );
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "active epoch\n",
            "a timed-out post-reap acquisition rewrote the epoch"
        );
    }

````
<!-- /fragment -->

**What it establishes.** Against a lock a live process holds, the wait is
bounded and the contention diagnostic is emitted **once** rather than once per
poll; the injected clock advances ten seconds per wait, so the loop gives up at
thirty; and the epoch file is byte-identical afterwards, so a timed-out
acquisition writes nothing.

**What it would still pass under — and this is where the earlier pattern
inverts.** The bound under test is the test's own argument,
`Duration::from_secs(30)`, and the error message renders that same argument back.
So neither the `elapsed` assertion nor the string says anything about
`EPOCH_HANDOFF_TIMEOUT`. Enumerating the constant's uses settles it: it appears
twice in the workspace, at its declaration on line 35 and at the one wrapper that
passes it, line 341 — and in no test. `EPOCH_WAIT_INTERVAL` is the same, at lines
36 and 343. **The thirty-second handoff bound and the ten-millisecond poll are
pinned by nothing in this corpus.** Even the `elapsed` assertion is weaker than it
looks: with a ten-second step, any bound in the range 21 to 30 seconds produces
the same final reading of thirty.

<!-- fragment «epoch-tests-contention-text» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1777-1787" parent="lease-tests" -->
````rust
    #[test]
    fn the_epoch_contention_diagnostic_names_the_lock_mode_and_operation() {
        let diagnostic = epoch_contention_diagnostic(LockMode::Exclusive, "post-reap invalidation");

        assert!(diagnostic.contains("exclusive"), "{diagnostic}");
        assert!(
            diagnostic.contains("post-reap invalidation"),
            "{diagnostic}"
        );
    }

````
<!-- /fragment -->

**What it establishes.** The diagnostic names the lock mode and the operation
it is waiting for — the two things an operator staring at a stalled `grove` needs
in order to tell a slow handoff from a wedged one.

**What it would still pass under.** Two `contains` assertions over a formatted
string. Word order, punctuation, the phrase *waiting for*, and whether the
sentence is a diagnostic at all rather than a fragment, are all unconstrained;
so is where it goes, since the test calls the formatter directly and never the
`eprintln!` that uses it. It is the block's only pure unit test, and it pins a
vocabulary rather than a behaviour.

<a id="what-no-ambient-context-means"></a>
## What *no ambient context* means, twice

Two tests split the absent case between the two halves the production code
separated: the decision an absent context leads to, and the reading that decides
a context is absent.

<!-- fragment «epoch-tests-manual-operations» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1788-1795" parent="lease-tests" -->
````rust
    #[test]
    fn manual_agent_operations_need_no_driver_epoch() {
        let admission = admit_session(Path::new("/not-a-working-tree"), "manual pick", None)
            .expect("no ambient context is a manual command, not a failure");

        assert!(admission.is_none());
    }

````
<!-- /fragment -->

**What it establishes.** A `grove-llm` invocation with no loop-control context
is a manual command, and admission returns `Ok(None)` rather than an error. This
is the case that keeps grove usable by hand.

**What it would still pass under.** The fixture path is `/not-a-working-tree`,
which reads as though the test were also checking that a bogus path is tolerated.
It is not: `admit_session` destructures the ambient option in its first
statement and returns before `Workspace::resolve` is ever reached, so the path is
never examined. The test would pass with a real worktree, a temporary directory,
or an empty string. Its expressive fixture describes a branch it does not take.

<!-- fragment «epoch-tests-nonempty-ambient» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1796-1813" parent="lease-tests" -->
````rust
    /// The reading half, pinned without touching the environment. The empty case
    /// is not a curiosity: `.cargo/config.toml` force-clears `GROVE_SIGNAL_FILE`
    /// to the empty string, so treating empty as a *path* would make every
    /// cargo-launched `grove-llm` stale-fail before reaching its test seam.
    #[test]
    fn only_a_nonempty_loop_control_value_is_ambient_context() {
        assert_eq!(signal_path_from(None), None, "unset is no ambient context");
        assert_eq!(
            signal_path_from(Some(OsString::new())),
            None,
            "the cargo-cleared empty value is no ambient context either"
        );
        assert_eq!(
            signal_path_from(Some(OsString::from("/w/.jj/grove/signal-0"))),
            Some(PathBuf::from("/w/.jj/grove/signal-0"))
        );
    }

````
<!-- /fragment -->

**What it establishes.** `signal_path_from` treats an unset value and an empty
value alike as *no ambient context*, and a non-empty value as a path. Its doc
comment gives the reason, and the reason is a fact about this repository's own
build configuration rather than about Unix: `.cargo/config.toml` clears the
variable to the empty string instead of unsetting it, so *empty* is what every
cargo-launched `grove-llm` actually sees. Treating it as a degenerate path would
make every one of them fail admission before reaching its test seam.

**What it would still pass under.** The function is total and pure over three
inputs, and all three are here, so as a test of `signal_path_from` it is
complete. What it does not reach is the wrapper: nothing asserts that
`ambient_signal_path` reads `GROVE_SIGNAL_FILE` rather than some other name, and
nothing in process can, for the reason the previous section gave. Chapter 15 met
the same shape in `verbs::signal_channel`, which carries its own separate copy of
this filter, and established by mutation that deleting the environment fallback
there changed no test at all. These are two filters with one rationale; this one
has a direct test because the value arrives as an argument.

<a id="the-handoff-twice"></a>
## The handoff, twice, and the tests that have to leave the process

These two are the carried example, and they are the reason the fork guard
exists. Both drive a real replacement against a real lease.

<!-- fragment «epoch-tests-old-finishes» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1814-1873" parent="lease-tests" -->
````rust
    #[test]
    fn an_admitted_old_operation_finishes_before_replacement_invalidates_new_calls() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();

        let admission = admit_session(&root, "test pick", ambient(&signal_path))
            .unwrap()
            .expect("ambient loop context must return a held admission guard");

        let epoch_path = root.join(".jj/grove").join(EPOCH_FILE_NAME);
        let probe = File::open(&epoch_path).unwrap();
        assert_ne!(
            unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
            0,
            "exclusive invalidation overlapped the admitted ambient operation"
        );

        drop(probe);
        drop(lease);
        let (started_tx, started_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let replacement_root = root.clone();
        let replacement = thread::spawn(move || {
            let _ = started_tx.send(());
            let result = DriverLease::acquire(&workspace_at(&replacement_root));
            if result_tx.send(result).is_err() {
                panic!("replacement result receiver disappeared");
            }
        });
        started_rx.recv().unwrap();
        assert!(
            matches!(
                result_rx.recv_timeout(Duration::from_millis(50)),
                Err(mpsc::RecvTimeoutError::Timeout)
            ),
            "replacement invalidated the epoch before the admitted operation returned"
        );

        drop(admission);
        let replacement_lease = result_rx
            .recv_timeout(Duration::from_secs(1))
            .expect("replacement did not acquire after the ambient guard dropped")
            .unwrap();
        replacement.join().unwrap();

        let error = admit_session(&root, "test pick", ambient(&signal_path)).unwrap_err();
        assert!(
            format!("{error:#}").contains("session epoch is inactive"),
            "a new call from the old session was not refused: {error:#}"
        );
        drop(replacement_lease);
    }

````
<!-- /fragment -->

**What it establishes.** The ordering claim in full, in four moves. An
admitted operation holds the epoch file's shared lock — proved by a
`LOCK_EX | LOCK_NB` probe from the same process *failing*. A replacement driver
started on another thread does not complete while that guard is alive — proved by
a `recv_timeout` of fifty milliseconds expiring. It completes promptly once the
guard drops — proved by a second receive with a one-second bound succeeding. And
the old session's next call is then refused as inactive. Admitted calls finish;
new ones are refused; nothing is timed out of the way.

**What it would still pass under.** The fifty-millisecond negative is a
liveness assertion made by waiting, so it would also pass against a replacement
that was merely slow for other reasons — the test cannot distinguish *blocked on
the lock* from *not yet started*, and the `started_tx` handshake it takes first
only proves the thread began, not that it reached the lock. The final refusal is
matched on `"session epoch is inactive"`.

<!-- fragment «epoch-tests-record-until-handoff» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1874-1916" parent="lease-tests" -->
````rust
    #[test]
    fn replacement_keeps_the_old_lease_record_until_it_owns_epoch_handoff() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let old_lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        old_lease.activate_session_epoch(&signal_path).unwrap();
        let lease_path = root.join(".jj/grove").join(LEASE_FILE_NAME);
        let old_record = fs::read_to_string(&lease_path).unwrap();
        let epoch_guard = File::open(root.join(".jj/grove").join(EPOCH_FILE_NAME)).unwrap();
        assert_eq!(
            unsafe { libc::flock(epoch_guard.as_raw_fd(), libc::LOCK_SH) },
            0
        );
        drop(old_lease);

        let (reached_tx, reached_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let replacement_root = root.clone();
        let replacement = thread::spawn(move || {
            DriverLease::acquire_with(&workspace_at(&replacement_root), || {
                reached_tx.send(()).unwrap();
                release_rx.recv().unwrap();
            })
        });
        reached_rx.recv().unwrap();

        assert_eq!(
            fs::read_to_string(&lease_path).unwrap(),
            old_record,
            "replacement published its nonce before acquiring exclusive epoch handoff"
        );
        release_tx.send(()).unwrap();
        drop(epoch_guard);
        let replacement_lease = replacement.join().unwrap().unwrap();
        assert_ne!(fs::read_to_string(&lease_path).unwrap(), old_record);
        drop(replacement_lease);
    }

````
<!-- /fragment -->

**What it establishes.** The ordering chapter 16 argued from
`initialize_epoch_record`'s comment, observed. The test holds the epoch file's
shared lock itself, then starts a replacement and stops it at
`before_initial_epoch_handoff`; at that moment the lease record on disk is still
byte-for-byte the predecessor's. Only after the barrier is released and the
shared lock dropped does the record change. A replacement that published its
nonce before winning exclusive epoch handoff would reject an operation the old
epoch had already admitted, and this is the test that says it does not.

**What it would still pass under.** The comparison is `read_to_string` of the
lease path against the earlier string, so it pins the record's *bytes* but not
the descriptor's identity — a replacement that rewrote the same content into a
new file would pass. The closing `assert_ne!` establishes only that the record
changed by the end, not that it changed at the right moment; the moment is
carried entirely by the barrier.

<a id="the-admission-ladder"></a>
## The admission ladder, and which rungs are pinned

The last seven tests are all about `admit_session` — six call it directly, and
the seventh drives the liveness probe that sits underneath its last rung — and
they are best read against its shape rather than one at a time. It is a
**ladder**: a fixed sequence of checks, each with its own refusal, each reached
only if every check above it passed. Resolve the workspace; find the control
directory; take the epoch file's shared lock; parse the record; compare the
working-tree root; compare its identity; require the epoch to be active; require
the channel to match; probe that the driver is still alive.

That order is what makes these tests stronger than they look. A test asserting
on a refusal from far down the ladder has thereby established that every check
above it *passed* for its fixture — it is an ordering proof, not merely a refusal
proof. It also makes one of them much weaker than it looks, because six separate
rungs and the whole probe — seven sites in all — are wrapped in the same `stale
Grove session for {operation}` context, so that substring does not identify a
rung. The only refusal on the path that is *not* so wrapped is the working-tree
mismatch.

Which rungs are actually held was measured rather than read, in a copy of the
workspace, by replacing each refusal's **whole macro call** with `panic!("MUTANT")`
— a message-preserving panic is invisible to the out-of-process `grove-llm` suite,
which asserts on stderr substrings — and diffing the per-test results against a
control. The control is 560 tests, 549 passing, with eleven failures in
`crates/grove-loop/tests/prompt.rs` that are environmental and have two causes
rather than one: ten because the copy is not a jj repository, and
`the_namespace_is_the_shipped_plugin_entrys_declared_name` because the copy
carries no `.claude-plugin/marketplace.json`.

Three things about that run are worth stating, because each of them is a way the
study could have read cleanly while measuring nothing.

**Each mutant is relinked into the `grove` binary.** `cargo build -p grove
--bins` runs *after* the edit, not before it: `cargo test -p grove-loop -p
grove-llm` never rebuilds that package, and `workspace_binary` reuses
`target/debug/grove` if the file merely exists. Without the step every
out-of-process observer runs a `grove` built from unmutated source and stays
green — which would have emptied the channel row below of the two tests that make
it interesting.

**Failure names are qualified by the binary that ran them.** The 560 test lines
carry 559 distinct bare names; the one duplicate,
`finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf`, passes in both
`grove-llm/tests/finish_commit.rs` and `grove-loop/tests/verbs.rs`, and this is
the first study in the book whose rows credit out-of-process `grove-llm` tests by
design.

**A mutant is checked against the control's whole roster, not its total.** Every
mutant below reported 560, so none failed to compile — but the total is a scalar
and cannot say *what* went wrong when it disagrees. Two of these eight mutations
make a fork-sensitive lease test's re-exec'd child panic, and cargo's stdout then
gains a **nested** block — `running 1 test` … `245 filtered out` — inside the
parent binary's. Counting all the per-test lines gives 561 and 562, which does
flag those two; what it does not say is that pairing stdout's blocks against
stderr's `Running` lines by index has shifted every later binary's label by one,
so that the newly-failing set names tests in binaries that never ran them.
Requiring the set of `<binary>@<test>` pairs to be *identical* to the control's,
with only the verdicts free to move, names that fault instead of merely
signalling one. All eight rows below were read that way.

| Rung | Refusal | Observed by |
|---|---|---|
| working tree differs | `wrong working tree for {op}` | `ambient_context_from_another_worktree_names_both_roots` |
| identity changed | `working-tree identity changed` | **nothing** |
| epoch inactive | `session epoch is inactive` | `an_inactive_epoch_is_reported_without_claiming_a_session_is_active`, `an_admitted_old_operation_finishes_before_replacement_invalidates_new_calls` |
| channel differs | `loop-control path does not match the active epoch` | `a_rotated_epoch_refuses_the_old_signal_path`, and two out-of-process tests in `crates/grove-loop/tests/driver_lease.rs`: `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session` and `grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` |
| lease record differs | `driver lease record does not match…` | **nothing** |
| lease unlocked | `driver lease is unlocked` | `a_successful_liveness_probe_releases_the_lease_before_validation`, `an_active_epoch_without_a_live_lease_is_stale` |
| probe replaced 8× | `replaced during liveness probe 8 times` | **nothing**, and reachable |
| probe retries exhausted | `liveness probe exhausted its bounded retries` | **nothing**, and unreachable |

One name is missing from that column on purpose, and the reason is worth more
than the row it would have joined.
`task_grow::tests::leaf_insert_lints_cross_references_under_a_shared_opening_of_its_own`
came back newly failing under four of the eleven mutant runs behind this table —
three distinct mutations, two of which had left it green on their own first run.
A single run would have credited it to whichever rung it happened to land on.

Three things say it observes nothing. It fails on its own last assertion —
`exclusive_lock_is_free(worktree.path())` when this table was taken, and a
descriptor count since, for the reason the end of this section gives — which is
about its own worktree and not about a refusal. It cannot have executed a
mutated line at all:
`admit_session` has exactly one production caller, and that caller takes the
ambient path `cargo` force-clears to empty, so no in-process test outside this
module's own block reaches any rung. And run alone under the mutant it passes,
while the two tests credited on that rung fail alone under the same mutant and
pass alone under the control — which is the same experiment answering in both
directions. What it is sensitive to is its neighbourhood rather than the
mutation: the assertion that breaks is a lock probe over its own worktree, and
every mutation that has broken it also turned a test in the same binary red. The
obvious candidate is a sibling's re-exec'd subprocess outliving the probe — the
fork sensitivity two of the original admission tests are already run in a subprocess to
contain — but one of the three broke it in a run that emitted no such block at
all, so the mechanism was not settled here. What is settled here is that it
observes no rung.

**The mechanism has been settled since, and the run that emitted no block is
what names it.** `flock` attaches to the open file description and `fork`
duplicates every one of them, so it is not a re-exec'd grove that matters but
*any* spawn — the `jj` these fixtures run constantly — copying the guard into a
child that holds the lock until `exec` closes it under `O_CLOEXEC`. Against a
private directory locked and released in a loop, an exclusive probe came back
`EWOULDBLOCK` 0 times in 20,000 with nothing else spawning, 189 with four
spawning threads and 445 with eight. Both assertions of this shape ask
`descriptors_held_on` how many descriptors *this process* holds instead, which a
forked child's copy cannot perturb and which is the stronger question anyway,
since a lock needs a descriptor to live on.

The general form is worth keeping. A cross-test flake reads exactly like a newly
attributed observer, and a second full run is the expensive way to tell them
apart and an unreliable one: of the three mutations that ever reddened it, two
changed their answer on a second run and the third repeated the false
attribution. Running the candidate alone under the mutant is the cheap way, and
it answers.

Two of the four zeros are the interesting ones, and they are asymmetries
rather than absences. *Identity changed* sits directly beneath *working tree
differs*, which has a test; both are reached by the same fixture shape — two
directories, one epoch — and the difference between them is only whether the path
compares equal before the inode does. Nothing explains why one is pinned and the
other is not. *Lease record differs* sits inside the probe between two arms that
are pinned, and it is the arm that catches a driver which was replaced between
the epoch read and the probe. The third zero is reachable and the fourth is not, which
is the distinction a bare count cannot make. The probe's eight-attempt identity
arm needs only a fixture that replaces the lease path on every pass, and the
existing `after_successful_probe` hook runs immediately before the comparison, so
the seam for it is already there. The trailing `bail!` after that loop is
different: the last iteration always returns or bails, so control never reaches
the line at all. Its zero is deadness, not absence — and the file has three of
them, one after each bounded-retry loop, all in chapter 16's half.

The channel-mismatch row is the one to read twice. Its three observers include
`a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session` and
`grove_llm_admits_only_the_live_epoch_while_version_remains_exempt`, neither of
which is in this crate's unit tests: they shell out to a real `grove-llm`. A
message-preserving mutation would have shown this rung held by one test instead
of three, because the two integration tests match on stderr and cannot tell a
panic that keeps the format string from the refusal it replaced.

With the ladder in view, the seven read quickly.

<!-- fragment «epoch-tests-foreign-worktree» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1917-1941" parent="lease-tests" -->
````rust
    #[test]
    fn ambient_context_from_another_worktree_names_both_roots() {
        let tmp = TempDir::new().unwrap();
        let owner_root = tmp.path().join("owner");
        let foreign_root = tmp.path().join("foreign");
        fs::create_dir_all(owner_root.join(".jj")).unwrap();
        fs::create_dir_all(foreign_root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&owner_root)).unwrap();
        let signal_path = owner_root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();

        let error = admit_session(&foreign_root, "test pick", ambient(&signal_path)).unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("wrong working tree"), "{message}");
        assert!(
            message.contains(owner_root.canonicalize().unwrap().to_str().unwrap()),
            "{message}"
        );
        assert!(
            message.contains(foreign_root.canonicalize().unwrap().to_str().unwrap()),
            "{message}"
        );
    }

````
<!-- /fragment -->

**What it establishes.** A call made from a different working tree than the
one the epoch belongs to is refused, and the message names **both** roots —
canonicalised, which the test checks by canonicalising them itself. An operator
who has two worktrees open needs to know which one the session belongs to, not
merely that this is the wrong one. The mutation attributes this rung to this test
alone.

**What it would still pass under.** Three `contains` assertions and no
constraint on order or framing. It also does not establish that the *identity*
comparison one rung below would have caught the same case, which is the arm the
mutation found unheld.

<!-- fragment «epoch-tests-inactive-reported» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1942-1956" parent="lease-tests" -->
````rust
    #[test]
    fn an_inactive_epoch_is_reported_without_claiming_a_session_is_active() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let _lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let stale_signal = root.join(".jj/grove/signal-11111111111111111111111111111111");

        let error = admit_session(&root, "test pick", ambient(&stale_signal)).unwrap_err();

        let message = format!("{error:#}");
        assert!(message.contains("session epoch is inactive"), "{message}");
        assert!(!message.contains("active epoch"), "{message}");
    }

````
<!-- /fragment -->

**What it establishes.** An inactive epoch is reported as inactive, and — the
second assertion, which is the real one — the message does **not** contain the
phrase *active epoch*. A refusal that says the session is stale must not read as
though some other session were live; that is a diagnostic obligation, and it is
pinned negatively because there is no other way to pin it.

**What it would still pass under.** The negative assertion is over a literal
substring, so any rewording that avoided those two words while still implying a
live session would satisfy it.

<!-- fragment «epoch-tests-rotated-signal» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1957-1980" parent="lease-tests" -->
````rust
    #[test]
    fn a_rotated_epoch_refuses_the_old_signal_path() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let old_signal = root.join(".jj/grove/signal-11111111111111111111111111111111");
        let new_signal = root.join(".jj/grove/signal-22222222222222222222222222222222");
        lease.activate_session_epoch(&old_signal).unwrap();

        drop(
            admit_session(&root, "test pick", ambient(&old_signal))
                .unwrap()
                .expect("the old signal must be admitted while its epoch is live"),
        );
        lease.activate_session_epoch(&new_signal).unwrap();

        let error = admit_session(&root, "test pick", ambient(&old_signal)).unwrap_err();
        assert!(
            format!("{error:#}").contains("loop-control path does not match the active epoch"),
            "unexpected error: {error:#}"
        );
    }

````
<!-- /fragment -->

**What it establishes.** The strongest of the ladder tests, because it asserts
a *transition*. The old signal path is admitted while its epoch is live — the
`.expect` on that first call is a real assertion — then the driver activates a new
channel, and the same old path is now refused. So the record is authoritative
about which channel is current, and admission is not a property of the path's
shape.

**What it would still pass under.** The refusal is matched on `"loop-control
path does not match the active epoch"`, and the mutation confirms this rung. What
is not asserted is that the *new* path would now be admitted; the test rotates
and checks only the losing side.

<!-- fragment «epoch-tests-separator-bytes» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="1981-2000" parent="lease-tests" -->
````rust
    #[test]
    fn an_epoch_signal_path_round_trips_record_separator_bytes() {
        let tmp = TempDir::new().unwrap();
        let root = tmp
            .path()
            .join(OsString::from_vec(b"worktree-\n-name".to_vec()));
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();

        drop(
            admit_session(&root, "test pick", ambient(&signal_path))
                .unwrap()
                .expect("the exact signal path must survive epoch serialization"),
        );
        let record = fs::read_to_string(root.join(".jj/grove/session.epoch")).unwrap();
        assert!(record.contains("signal-path-hex="), "{record:?}");
    }

````
<!-- /fragment -->

**What it establishes.** A working-tree path containing a newline survives the
epoch record's line-oriented `key=value` format, because the path is hex-encoded
rather than written raw. The fixture builds the directory name from raw bytes
through `OsString::from_vec` to get a newline into a path at all.

**What it would still pass under.** Very nearly anything, if you read only the
assertion. `assert!(record.contains("signal-path-hex="))` holds for any record
with that field, whatever it encodes. The actual round-trip is pinned by the
`.expect` on the line above: admission succeeded, and admission compares the
*decoded* path from the record against the ambient one at the channel rung, so
the encode-decode pair must have been faithful for that call to be admitted. The
weight is on the `expect`, and the assertion that follows it is close to
decorative.

<!-- fragment «epoch-tests-probe-releases» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="2001-2042" parent="lease-tests" -->
````rust
    #[test]
    fn a_successful_liveness_probe_releases_the_lease_before_validation() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        let control_dir = root.join(".jj/grove");
        let mut epoch_file = File::open(control_dir.join(EPOCH_FILE_NAME)).unwrap();
        let epoch = read_epoch_record(&mut epoch_file).unwrap();
        drop(lease);
        let mut observed_unlocked_probe = false;

        let error = probe_live_lease_with_post_unlock_hook(
            &control_dir,
            &epoch,
            "test pick",
            |lease_path| {
                let contender = File::open(lease_path)?;
                let result =
                    unsafe { libc::flock(contender.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
                if result != 0 {
                    return Err(std::io::Error::last_os_error())
                        .context("successful liveness probe still held the driver lease");
                }
                observed_unlocked_probe = true;
                Ok(())
            },
        )
        .unwrap_err();

        assert!(observed_unlocked_probe);
        assert!(
            format!("{error:#}").contains("driver lease is unlocked"),
            "unexpected error: {error:#}"
        );
    }

````
<!-- /fragment -->

**What it establishes.** The liveness probe releases the lock it took before it
judges what it found. The hook runs after a successful probe and takes
`LOCK_EX | LOCK_NB` on the lease path from the same process; that succeeding is
what proves the probe unlocked. The test then requires the overall result to be
the *unlocked* refusal — the probe found no live driver — so the release happens
on the success path and not only on the way out.

**What it would still pass under.** `observed_unlocked_probe` is set inside the
hook and asserted afterwards, so a probe that never called the hook would fail;
that much is guarded. But the test drives
`probe_live_lease_with_post_unlock_hook` directly rather than through
`admit_session`, so it says nothing about the probe being reached in admission —
which is the previous section's ladder claim, held by the next test instead.

<!-- fragment «epoch-tests-active-no-lease» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="2043-2062" parent="lease-tests" -->
````rust
    #[test]
    fn an_active_epoch_without_a_live_lease_is_stale() {
        if !fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        drop(lease);

        let error = admit_session(&root, "test pick", ambient(&signal_path)).unwrap_err();
        assert!(
            format!("{error:#}").contains("driver lease is unlocked"),
            "unexpected error: {error:#}"
        );
    }

````
<!-- /fragment -->

**What it establishes.** The whole ladder, end to end, on a real fixture. The
lease is acquired, an epoch activated, and the lease then dropped, leaving an
`active` record whose driver is gone. Admission is refused with *driver lease is
unlocked* — and because that refusal is the **last** rung, this single assertion
establishes that the workspace resolved, the control directory was found, the
shared lock was taken, the record parsed, the root matched, the identity matched,
the epoch was active and the channel matched. It is the most informative
assertion in the block, and it is one line.

**What it would still pass under.** The message is matched with `contains`, and
`format!("{error:#}")` renders the whole context chain, so the assertion cannot
tell which layer produced the phrase. It is nonetheless attributed: the mutation
puts this test and the probe test on that arm and no other.

<!-- fragment «epoch-tests-malformed» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease.rs" lines="2063-2079" parent="lease-tests" -->
````rust
    #[test]
    fn a_malformed_epoch_is_stale() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("worktree");
        fs::create_dir_all(root.join(".jj")).unwrap();
        let lease = DriverLease::acquire(&workspace_at(&root)).unwrap();
        let signal_path = root.join(".jj/grove/signal-11111111111111111111111111111111");
        lease.activate_session_epoch(&signal_path).unwrap();
        fs::write(root.join(".jj/grove/session.epoch"), "state=active\n").unwrap();

        let error = admit_session(&root, "test pick", ambient(&signal_path)).unwrap_err();
        assert!(
            format!("{error:#}").contains("stale Grove session"),
            "unexpected error: {error:#}"
        );
    }
}
````
<!-- /fragment -->

**What it establishes.** A record that is syntactically an epoch but missing
its required fields is treated as stale rather than trusted or crashed. The
fixture overwrites a good record with the single line `state=active`, so parsing
fails on the absent process fields.

**What it would still pass under, and this is the weakest assertion in the
block.** It matches on `"stale Grove session"` alone. That phrase is produced at
**seven** distinct places in `admit_session`'s path — the missing control-directory
parent, the shared-lock acquisition, the record parse, the identity comparison,
the inactive epoch, the channel mismatch, and every failure the liveness probe
can return. So the test establishes *refused, and classified as stale*, and not
*refused because the record was malformed*. Had the fixture instead produced an
inactive epoch, a wrong channel or a dead driver, the assertion would have held
just the same. Its sibling one rung up shows the alternative: the inactive test
adds a negative assertion and thereby pins which refusal it got.

The module closes on the same line as the last test.

<a id="what-could-not-move-here"></a>
## What could not move

The book's question, asked of a block that is entirely evidence.

**On the way in — the names.** This block owns no grammar and parses nothing,
but it is where one naming decision is visible that no library beneath grove
could have made: the ambient context is a **path**, and the empty string is not a
degenerate one. That is not a Unix fact. It is a fact about how this repository
builds itself, discovered in `.cargo/config.toml`, and it had to be written down
in a filter and pinned by a test because nothing else in the system would have
noticed.

**On the way through — the preconditions.** This is the block's whole subject,
and the outcome's second cost is legible here in a way it is nowhere else: *the
check must run against the same snapshot the operation then plans from.* The
admitted guard holds its shared lock for the operation's whole life rather than
checking and releasing, and two of the original admission tests exist only to observe that
— at the price of running in a subprocess, because a sibling test's `fork` would
otherwise extend a lease past its drop.

**On the way out — the policy.** What this block chooses is where its own
evidence stops, and the enumeration above is the honest account of it. Eighteen
scenarios hold four of the ladder's seven refusal arms; the identity rung, the
probe's record comparison and its identity exhaustion are held by nothing though
all three are reachable, the thirty-second handoff bound and the ten-millisecond
poll are pinned by nothing, and two of the four close-on-exec marks are asserted
nowhere. None of that makes the code wrong.
Chapter 16 argued each mechanism from the source and found it tighter than its
own record's slogan; what this chapter adds is the shape of the evidence
underneath that argument, which is thinner in specific, nameable places.

**And the thing this chapter is really for.** Chapter 13 said the crate can be
re-derived from the tree and chapter 16 said this file cannot. The evidence for
*cannot* is here. A test for a lease has to hold real descriptors, take real
locks, start real threads and — twice — start a real process, because there is
nothing on disk to assert against afterwards. Every other chapter's tests can end
by listing a directory. These end by racing something and watching what happened,
and that is the same fact about untracked state, seen from the side that has to
prove it.

<a id="runtime-observation"></a>
## Observing without admission

A viewer needs evidence without acquiring authority. Tree capture has already
released its guard when this private child of driver_lease runs. It returns
Idle, Running, Busy or Unavailable independently of tree readability. A released
private witness establishes Idle even with an active epoch. A held exact Started
marker establishes Running only after checking the captured directory relation;
old active records cannot identify a mandate.

<!-- fragment «runtime-observer» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1-2172" parent="source-runtime-observation" -->
<!-- insert «runtime-entry» -->
<!-- insert «runtime-read» -->
<!-- insert «runtime-extension» -->
<!-- insert «runtime-private-probe» -->
<!-- insert «runtime-files» -->
<!-- insert «runtime-test-fixture» -->
<!-- insert «runtime-test-release» -->
<!-- insert «runtime-test-legacy» -->
<!-- insert «runtime-test-extension» -->
<!-- insert «runtime-test-substitution» -->
<!-- insert «runtime-test-controls» -->
<!-- insert «runtime-test-guards» -->
<!-- insert «runtime-test-races» -->
<!-- /fragment -->

### The observation boundary

The private reader converts errors into Unavailable while leaving the captured tree intact. It shares the lease module’s mandatory grammar and identity type; it never enters session admission.

<!-- fragment «runtime-entry» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1-19" parent="runtime-observer" -->
````rust
//! Advisory runtime reads share admission's grammar, never its authority path.

use super::*;
use crate::observation::RuntimeIdentity;
use crate::{ActivityObservation, LaunchTreeIdentity, RunningMandate, TreeLifetime, TreeRelation};

const RECORD_LIMIT: u64 = 64 * 1024;

pub(crate) fn observe(
    worktree: &Path,
    tree: Option<&TreeLifetime>,
    in_epoch: impl FnMut(),
) -> ActivityObservation {
    match read_runtime(worktree, tree, in_epoch, &mut NativeWitnessIo) {
        Ok(activity) => activity,
        Err(error) => ActivityObservation::Unavailable(format!("{error:#}")),
    }
}

````
<!-- /fragment -->

### One bounded runtime sample

Discovery names an exact existing namespace. The root is pinned before namespace discovery so a retargeted alias cannot mix workspaces. Each attempt opens the namespace, lease and epoch, then tries a shared epoch lock without waiting. Path identities are checked after acquisition and again after copying records. Matching inactive records or a validated released private witness establish Idle; an active legacy epoch has no witness and remains Unavailable. A vanished lease is Idle only after checking the pinned directories. The eight-attempt limit bounds replacement races; descriptor drop releases every successful lock before a retry or return.

The active-record diagnostic names only the epoch record: without a witness it
cannot establish that a session is alive. The replacement checks cover alias
retargeting during discovery, replacement between open and lock, and replacement
after copying. Those three windows are justified by the following source order;
the deterministic replacement test below injects only inside the epoch guard,
before copying. It does not exercise those other windows.

<!-- fragment «runtime-read» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="20-106" parent="runtime-observer" -->
````rust
fn read_runtime(
    worktree: &Path,
    tree: Option<&TreeLifetime>,
    mut in_epoch: impl FnMut(),
    io: &mut impl WitnessIo,
) -> Result<ActivityObservation> {
    // Pin before discovery so a retargeted workspace alias cannot mix namespaces.
    let root = match open(worktree, true) {
        Ok(root) => root,
        Err(error) if missing(&error) => return Ok(ActivityObservation::Idle),
        Err(error) => return Err(error),
    };
    let namespace = Workspace::discover_control_dir(worktree, CONTROL_NAMESPACE)?;
    anyhow::ensure!(
        current(&root, worktree)?,
        "workspace changed during runtime discovery"
    );
    let Some(namespace) = namespace else {
        return Ok(ActivityObservation::Idle);
    };
    for _ in 0..IDENTITY_RETRY_LIMIT {
        let directory = open(&namespace, true)?;
        let lease_path = namespace.join(LEASE_FILE_NAME);
        let epoch_path = namespace.join(EPOCH_FILE_NAME);
        let mut lease = match open(&lease_path, false) {
            Ok(file) => file,
            Err(error) if missing(&error) => {
                if current(&root, worktree)? && current(&directory, &namespace)? {
                    return Ok(ActivityObservation::Idle);
                }
                continue;
            }
            Err(error) => return Err(error),
        };
        let mut epoch = open(&epoch_path, false)?;
        let locked = unsafe { libc::flock(epoch.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) };
        let lock_error = (locked != 0).then(std::io::Error::last_os_error);
        // Check after the probe: a replaced descriptor cannot authorize even Busy.
        if !current(&root, worktree)?
            || !current(&directory, &namespace)?
            || !current(&lease, &lease_path)?
            || !current(&epoch, &epoch_path)?
        {
            continue;
        }
        if let Some(error) = lock_error {
            return if matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN)
            {
                Ok(ActivityObservation::Busy(
                    "session epoch publication is in progress".into(),
                ))
            } else {
                Err(error).context("probing shared session epoch")
            };
        }
        in_epoch();
        // The only guarded work is bounded record copying and identity checking.
        // File drop releases the shared flock on every return/retry/error path.
        let result = (|| {
            let lease_record = parse_process_record(&bounded_record(&mut lease)?)?;
            let epoch_bytes = bounded_record(&mut epoch)?;
            let epoch_record = parse_epoch_record(&epoch_bytes)?;
            anyhow::ensure!(
                lease_record == epoch_record.process,
                "lease and epoch records do not match"
            );
            anyhow::ensure!(
                lease_record.worktree_identity == FileIdentity::from_metadata(&root.metadata()?),
                "runtime records belong to a different working tree"
            );
            Ok(if epoch_record.signal_path.is_none() {
                ActivityObservation::Idle
            } else {
                observe_private(&namespace, &epoch_bytes, &epoch_record, tree, io)?
            })
        })();
        if current(&root, worktree)?
            && current(&directory, &namespace)?
            && current(&lease, &lease_path)?
            && current(&epoch, &epoch_path)?
        {
            return result;
        }
    }
    bail!("runtime paths changed during all {IDENTITY_RETRY_LIMIT} observation attempts")
}

````
<!-- /fragment -->

### Validate the launch binding

An active epoch must carry a complete version-1 extension before its witness can establish even Idle. The observer reuses the canonical Handle and Kind grammar, checks the explicit key, retains the launch-tree identity, and permits only a plain witness basename. The enclosing mandatory record already binds this extension to the lease nonce and signal path; admission still ignores these optional fields. The parsed mandate retains those binding values for runtime comparison, and the probe supplies its verified tree relation before Running is returned.

<!-- fragment «runtime-extension» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="107-168" parent="runtime-observer" -->
````rust
/// Validate the entire extension before trusting even evidence of release.
fn observation_witness(record: &str, epoch: &EpochRecord) -> Result<RunningMandate> {
    anyhow::ensure!(
        record_field(record, "observation-version")
            .context("active epoch record has no supported observation witness")?
            == "1",
        "active epoch record has no supported observation witness"
    );
    let text = |name| -> Result<String> {
        decode_path(record_field(record, name)?)?
            .into_os_string()
            .into_string()
            .map_err(|_| anyhow::anyhow!("{name} is not UTF-8"))
    };
    let handle = crate::Handle::parse(&text("observation-handle-hex")?)?;
    anyhow::ensure!(
        record_field(record, "observation-key")? == handle.key().to_string(),
        "observation key does not match its handle"
    );
    let kind = crate::Kind::new(&text("observation-kind-hex")?)?;
    let number = |name| -> Result<u64> {
        record_field(record, name)?
            .parse::<u64>()
            .with_context(|| format!("parsing {name}"))
    };
    let tree_identity = LaunchTreeIdentity((
        number("observation-tree-device")?,
        number("observation-tree-inode")?,
    ));
    let name = text("observation-witness-name-hex")?;
    anyhow::ensure!(
        !name.is_empty()
            && name != "."
            && name != ".."
            && !name.contains('/')
            && !name.contains('\0'),
        "observation witness name is not a plain basename"
    );
    Ok(RunningMandate {
        handle,
        kind,
        tree_identity,
        relation: TreeRelation::NoReadableTree,
        runtime: RuntimeIdentity {
            worktree: (
                epoch.process.worktree_identity.device,
                epoch.process.worktree_identity.inode,
            ),
            nonce: epoch.process.nonce.clone(),
            signal: epoch
                .signal_path
                .clone()
                .context("observation epoch is inactive")?,
            witness_name: PathBuf::from(name),
            witness: (
                number("observation-witness-device")?,
                number("observation-witness-inode")?,
            ),
        },
    })
}

````
<!-- /fragment -->

### Verify the captured tree and launch

The directory-before-private order implements `docs/specs/item-status.md`, One bounded observation. The directory proves which captured tree can receive activity; the private file proves that a launch started and has not released its witness. Neither fact substitutes for the other. The final private probe decides Idle even when earlier evidence failed, so teardown cannot preserve Running through leftover bytes or a directory error.

<!-- fragment «runtime-private-probe» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="169-287" parent="runtime-observer" -->
<!-- insert «runtime-witness-io» -->
<!-- insert «runtime-tree-relation» -->
<!-- insert «runtime-witness-verdict» -->
<!-- /fragment -->

The private `WitnessIo` seam supplies only filesystem and shared-probe operations. Production opens read-only, reads at most nine bytes to detect extra marker data, and releases every successful probe before further work. Tests can replace an acquisition or probe operation while retaining the production parser and status algorithm. No viewer-specific status provider exists.

<!-- fragment «runtime-witness-io» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="169-211" parent="runtime-private-probe" -->
````rust
/// Native operations are replaceable at the filesystem/lock boundary, not at
/// the status-provider boundary. A probe returns true only after unlocking.
trait WitnessIo {
    fn open(&mut self, path: &Path) -> Result<File> {
        open(path, false)
    }

    fn marker(&mut self, file: &mut File) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        file.take(9).read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    fn current(&mut self, file: &File, path: &Path) -> Result<bool> {
        current(file, path)
    }

    fn probe(&mut self, file: &File) -> Result<bool> {
        // Independent shared probes are compatible; only exclusive ownership
        // can contend. Release success immediately, before validation or I/O.
        // https://man7.org/linux/man-pages/man2/flock.2.html
        // https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/flock.2.html
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) } == 0 {
            if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) } != 0 {
                return Err(std::io::Error::last_os_error())
                    .context("releasing shared witness probe");
            }
            Ok(true)
        } else {
            let error = std::io::Error::last_os_error();
            if matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN)
            {
                Ok(false)
            } else {
                Err(error).context("probing shared witness")
            }
        }
    }
}

struct NativeWitnessIo;
impl WitnessIo for NativeWitnessIo {}

````
<!-- /fragment -->

The directory check consumes the accepted pin, never its current path. Missing capture evidence permits only NoReadableTree; unequal identities permit PreviousTree. For a numeric match, contention on this descriptor establishes SameTree, while an unlocked directory records an unverified binding. A directory error is retained until the final private probe determines whether any session remains.

<!-- fragment «runtime-tree-relation» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="212-234" parent="runtime-private-probe" -->
````rust
fn tree_relation(
    tree: Option<&TreeLifetime>,
    expected: LaunchTreeIdentity,
    io: &mut impl WitnessIo,
) -> Result<Option<TreeRelation>> {
    let Some(tree) = tree else {
        return Ok(Some(TreeRelation::NoReadableTree));
    };
    let metadata = tree.directory().metadata()?;
    if (metadata.dev(), metadata.ino()) != expected.0 {
        return Ok(Some(TreeRelation::PreviousTree));
    }
    // Probe the accepted pin itself: reopening its path could bind another tree.
    if io
        .probe(tree.directory())
        .context("probing task-root witness")?
    {
        Ok(None)
    } else {
        Ok(Some(TreeRelation::SameTree))
    }
}

````
<!-- /fragment -->

The observer validates the complete extension before either witness can authorize a result. It saves the directory verdict, then opens the private file, checks its published and path identities, reads the bounded marker and probes. A successful probe establishes Idle before marker or directory errors are considered. Otherwise exact Started bytes need a verified relation; prefixes are Busy and other bytes are Unavailable. Replacement after open or probe retries at most eight times, and a stable but unpublished replacement identity is rejected.

<!-- fragment «runtime-witness-verdict» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="235-287" parent="runtime-private-probe" -->
````rust
fn observe_private(
    namespace: &Path,
    record: &str,
    epoch: &EpochRecord,
    tree: Option<&TreeLifetime>,
    io: &mut impl WitnessIo,
) -> Result<ActivityObservation> {
    let mut mandate = observation_witness(record, epoch)?;
    // Save errors and unverified binding until the final private probe: release
    // establishes Idle even if the directory probe or marker read failed.
    let relation = tree_relation(tree, mandate.tree_identity, io);
    let path = namespace.join(&mandate.runtime.witness_name);
    for _ in 0..IDENTITY_RETRY_LIMIT {
        let mut file = io.open(&path)?;
        if !io.current(&file, &path)? {
            continue;
        }
        anyhow::ensure!(
            {
                let metadata = file.metadata()?;
                (metadata.dev(), metadata.ino()) == mandate.runtime.witness
            },
            "private witness identity does not match the epoch"
        );
        let marker = io.marker(&mut file);
        let released = io.probe(&file);
        if !io.current(&file, &path)? {
            continue;
        }
        if released.context("probing private witness")? {
            return Ok(ActivityObservation::Idle);
        }
        let marker = marker.context("reading private witness marker")?;
        if marker != b"started\n" {
            anyhow::ensure!(
                b"started\n".starts_with(&marker),
                "invalid private witness marker"
            );
            return Ok(ActivityObservation::Busy(
                "launch publication is in progress".into(),
            ));
        }
        let Some(relation) = relation? else {
            return Ok(ActivityObservation::Busy(
                "task-root witness is not held".into(),
            ));
        };
        mandate.relation = relation;
        return Ok(ActivityObservation::Running(mandate));
    }
    bail!("private witness changed during all {IDENTITY_RETRY_LIMIT} observation attempts")
}

````
<!-- /fragment -->


### Opening and copying controls

The filesystem helpers distinguish absence from inspection errors, use read-only nonblocking close-on-exec descriptors, reject wrong file types, and compare descriptor and path identities. The extra byte in the bounded read distinguishes an exactly 64 KiB record from an oversized one. These helpers perform no lease-lock probe, write or cleanup.

<!-- fragment «runtime-files» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="288-335" parent="runtime-observer" -->
````rust
fn missing(error: &anyhow::Error) -> bool {
    error
        .downcast_ref::<std::io::Error>()
        .is_some_and(|error| error.kind() == std::io::ErrorKind::NotFound)
}

fn open(path: &Path, directory: bool) -> Result<File> {
    // O_NONBLOCK prevents a substituted FIFO from waiting before fstat rejects it.
    // https://man7.org/linux/man-pages/man2/open.2.html
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(
            libc::O_NONBLOCK | libc::O_CLOEXEC | if directory { libc::O_DIRECTORY } else { 0 },
        )
        .open(path)
        .with_context(|| format!("opening runtime path {}", path.display()))?;
    let metadata = file.metadata()?;
    anyhow::ensure!(
        if directory {
            metadata.is_dir()
        } else {
            metadata.is_file()
        },
        "runtime path has the wrong file type: {}",
        path.display()
    );
    Ok(file)
}

fn current(file: &File, path: &Path) -> Result<bool> {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error).context("validating runtime path identity"),
    };
    Ok(FileIdentity::from_metadata(&file.metadata()?) == FileIdentity::from_metadata(&metadata))
}

fn bounded_record(file: &mut File) -> Result<String> {
    let mut record = String::new();
    file.take(RECORD_LIMIT + 1).read_to_string(&mut record)?;
    anyhow::ensure!(
        record.len() as u64 <= RECORD_LIMIT,
        "runtime record exceeds 64 KiB"
    );
    Ok(record)
}

````
<!-- /fragment -->

### Real records and the public capture

The fixture creates an exact temporary workspace and acquires a real driver lease. Its sample helper uses the public try_observe operation and requires a readable tree even when activity is unavailable. Fixture writes belong to tests, not observation.

<!-- fragment «runtime-test-fixture» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="336-360" parent="runtime-observer" -->
````rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use tempfile::TempDir;

    fn fixture() -> (TempDir, DriverLease) {
        let work = TempDir::new().unwrap();
        fs::create_dir(work.path().join(".jj")).unwrap();
        fs::create_dir(work.path().join(".grove")).unwrap();
        fs::write(work.path().join(".grove/_BRIEF.md"), "root").unwrap();
        let workspace = Workspace::resolve(work.path()).unwrap();
        let lease = DriverLease::acquire(&workspace).unwrap();
        (work, lease)
    }

    fn sample(work: &Path) -> ActivityObservation {
        let capture = crate::try_observe(work, &[]);
        assert!(matches!(
            capture.tree.unwrap(),
            crate::TreeObservation::Ready(_)
        ));
        capture.activity
    }

````
<!-- /fragment -->

### Released evidence overrides leftover bytes

The first public try_observe control identifies the prepared launch's handle/key, kind, nonce and SameTree relation, and compares a second sample for equality. The release control likewise starts with Running. Releasing the lease-owned pair leaves the active epoch intact. Empty, partial, exact and extra marker bytes must then all yield Idle without changing tree or administration bytes. An independent exclusive probe after each capture checks that no observer lock escaped. Controls that require an uncontended epoch after fixture creation or an exclusive epoch operation run in the existing isolated test process. A sibling fork could otherwise retain that exclusive descriptor until exec and produce incidental Busy activity. Each wrapper runs before fixture creation and preserves the original assertions.

<!-- fragment «runtime-test-release» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="361-428" parent="runtime-observer" -->
````rust
    #[test]
    fn witness_started_public_observation_identifies_the_prepared_launch() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = fixture();
        let root = crate::TreeLifetime::open(work.path()).unwrap().unwrap();
        lease
            .prepare_launch(
                root,
                &super::super::tests::witness_selection(),
                &lease.control_dir.join("signal-test"),
            )
            .unwrap();
        lease.launch.as_mut().unwrap().started().unwrap();
        let observed = sample(work.path());
        let ActivityObservation::Running(mandate) = observed else {
            panic!("{observed:?}");
        };
        assert_eq!(mandate.handle.to_string(), "work-k1");
        assert_eq!(mandate.handle.key().get(), 1);
        assert_eq!(mandate.kind.label(), "impl");
        assert_eq!(mandate.relation, TreeRelation::SameTree);
        assert_eq!(mandate.runtime.nonce, lease.nonce);
        assert_eq!(sample(work.path()), ActivityObservation::Running(mandate));
    }

    #[test]
    fn witness_released_active_epoch_is_idle_despite_leftover_bytes() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = fixture();
        let root = crate::TreeLifetime::open(work.path()).unwrap().unwrap();
        lease
            .prepare_launch(
                root,
                &super::super::tests::witness_selection(),
                &lease.control_dir.join("signal-test"),
            )
            .unwrap();
        let witness = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        lease.launch.as_mut().unwrap().started().unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Running(_)
        ));
        lease.launch.take();
        for marker in [
            b"started\n".as_slice(),
            b"",
            b"sta",
            b"invalid",
            b"started\nextra",
        ] {
            fs::write(&witness, marker).unwrap();
            let before = contents(work.path());
            assert_eq!(sample(work.path()), ActivityObservation::Idle);
            assert_eq!(contents(work.path()), before);
            // No successful observer probe may escape into the returned value.
            let file = File::open(&witness).unwrap();
            assert_eq!(
                unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
        }
    }

````
<!-- /fragment -->

### Legacy records retain their refusal

A real lease moves from inactive to active, admits a shared session operation, contends with an exclusive epoch writer and rotates its signal. Legacy active records remain unavailable, and old admission fails after rotation.

<!-- fragment «runtime-test-legacy» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="429-463" parent="runtime-observer" -->
````rust
    #[test]
    fn idle_legacy_active_contention_and_recovery_preserve_tree_and_admission() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease) = fixture();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        let signal = lease.control_dir.join("signal-first");
        lease.activate_session_epoch(&signal).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        let admitted = admit_session(work.path(), "test", Some(signal.clone())).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        drop(admitted);
        let lock = acquire_epoch_file(
            &lease.control_dir.join(EPOCH_FILE_NAME),
            LockMode::Exclusive,
            "test",
        )
        .unwrap();
        assert!(matches!(sample(work.path()), ActivityObservation::Busy(_)));
        drop(lock);
        lease.invalidate_session_epoch().unwrap();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        lease
            .activate_session_epoch(&lease.control_dir.join("signal-second"))
            .unwrap();
        assert!(admit_session(work.path(), "old session", Some(signal)).is_err());
    }

````
<!-- /fragment -->

### Invalid extensions cannot masquerade as release

Preparation supplies an authentic version-1 record and an unlocked witness. The tests enumerate every published observation field, remove it, duplicate it and corrupt it, then restore the valid Idle control between cases. Admission must continue accepting the same mandatory record. Additional cases challenge versions, canonical keys, typed handle/kind text, integer bounds, descriptor identity and basename traversal. Blanket Unavailable fails the positive controls.

<!-- fragment «runtime-test-extension» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="464-565" parent="runtime-observer" -->
````rust
    fn released_fixture() -> (TempDir, DriverLease, PathBuf, PathBuf, String) {
        let (work, mut lease) = fixture();
        let signal = lease.control_dir.join("signal-test");
        let root = crate::TreeLifetime::open(work.path()).unwrap().unwrap();
        lease
            .prepare_launch(root, &super::super::tests::witness_selection(), &signal)
            .unwrap();
        let witness = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        lease.launch.take();
        let epoch = lease.control_dir.join(EPOCH_FILE_NAME);
        let record = fs::read_to_string(&epoch).unwrap();
        (work, lease, witness, epoch, record)
    }

    #[test]
    fn witness_extension_validation_cannot_be_replaced_by_blanket_unavailability() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease, _, epoch, original) = released_fixture();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        // Enumerate the writer's extension: every published field is mandatory
        // and unique for observation, while admission ignores the extension.
        for line in original
            .lines()
            .filter(|line| line.starts_with("observation-"))
        {
            let (name, _) = line.split_once('=').unwrap();
            for invalid in [
                original.replace(&format!("{line}\n"), ""),
                format!("{original}{line}\n"),
                original.replace(line, &format!("{name}=invalid")),
            ] {
                fs::write(&epoch, invalid).unwrap();
                assert!(
                    matches!(sample(work.path()), ActivityObservation::Unavailable(_)),
                    "{name}"
                );
                assert!(admit_session(
                    work.path(),
                    "test",
                    Some(lease.control_dir.join("signal-test"))
                )
                .is_ok());
                fs::write(&epoch, &original).unwrap();
                assert_eq!(sample(work.path()), ActivityObservation::Idle);
            }
        }
        for (name, value) in [
            ("observation-version", "0"),
            ("observation-version", "2"),
            ("observation-key", "2"),
            ("observation-key", "01"),
            ("observation-key", "0"),
            ("observation-handle-hex", "776f726b2d6b3031"), // work-k01
            ("observation-handle-hex", "ff"),
            ("observation-kind-hex", "49"), // I
            ("observation-tree-device", "18446744073709551616"),
            ("observation-witness-device", "18446744073709551615"),
            ("observation-witness-inode", "0"),
        ] {
            let old = record_field(&original, name).unwrap();
            fs::write(
                &epoch,
                original.replace(&format!("{name}={old}"), &format!("{name}={value}")),
            )
            .unwrap();
            assert!(
                matches!(sample(work.path()), ActivityObservation::Unavailable(_)),
                "{name}={value}"
            );
        }
        let name_field = "observation-witness-name-hex";
        let old = record_field(&original, name_field).unwrap();
        for name in [
            "",
            ".",
            "..",
            "../witness",
            "/tmp/witness",
            "sub/witness",
            "witness/",
            "witness\0",
        ] {
            let encoded = encode_path(Path::new(name)).unwrap();
            fs::write(
                &epoch,
                original.replace(
                    &format!("{name_field}={old}"),
                    &format!("{name_field}={encoded}"),
                ),
            )
            .unwrap();
            assert!(
                matches!(sample(work.path()), ActivityObservation::Unavailable(_)),
                "{name:?}"
            );
        }
        fs::write(&epoch, &original).unwrap();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
    }

````
<!-- /fragment -->

### A name is insufficient evidence

An existing compatible shared holder still permits Idle. The same fixture then makes the witness unreadable, removes it, replaces it with a different regular file, and substitutes a directory and FIFO. Each unavailable result comes through try_observe. Retaining the original descriptor prevents the replacement control from depending on inode reuse; restoring its name restores Idle. Access refusal is checked only for a non-root test process.

<!-- fragment «runtime-test-substitution» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="566-616" parent="runtime-observer" -->
````rust
    #[test]
    fn witness_missing_replaced_and_nonregular_evidence_is_unavailable() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        use std::os::unix::fs::PermissionsExt;
        let (work, _lease, witness, _, _) = released_fixture();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        let retained = File::open(&witness).unwrap();
        assert_eq!(
            unsafe { libc::flock(retained.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        fs::set_permissions(&witness, fs::Permissions::from_mode(0o0)).unwrap();
        if unsafe { libc::geteuid() } != 0 {
            assert!(matches!(
                sample(work.path()),
                ActivityObservation::Unavailable(_)
            ));
        }
        fs::set_permissions(&witness, fs::Permissions::from_mode(0o600)).unwrap();
        let saved = witness.with_extension("saved");
        fs::rename(&witness, &saved).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::write(&witness, "started\n").unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_file(&witness).unwrap();
        fs::create_dir(&witness).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_dir(&witness).unwrap();
        let path = std::ffi::CString::new(witness.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_file(&witness).unwrap();
        fs::rename(saved, &witness).unwrap();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
    }

````
<!-- /fragment -->

### Record bounds and exact-workspace discovery

These retained controls reject malformed, oversized, invalid UTF-8, missing, directory and FIFO records. They also check aliases and exact-workspace isolation. Descriptor flag checks and bounded reads exercise both sides of the 64 KiB record limit.

<!-- fragment «runtime-test-controls» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="617-736" parent="runtime-observer" -->
````rust
    #[test]
    fn absent_controls_are_idle_but_missing_or_bad_records_are_unavailable() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease) = fixture();
        let epoch = lease.control_dir.join(EPOCH_FILE_NAME);
        let original = fs::read(&epoch).unwrap();
        for bytes in [
            b"bad".to_vec(),
            vec![b'x'; 65537],
            b"\xff".to_vec(),
            String::from_utf8(original.clone())
                .unwrap()
                .replace(&lease.nonce, "00000000000000000000000000000000")
                .into_bytes(),
            String::from_utf8(original.clone())
                .unwrap()
                .replace("state=inactive", "state=other")
                .into_bytes(),
            String::from_utf8(original.clone())
                .unwrap()
                .replace("state=inactive", "state=inactive\nsignal-path-hex=61")
                .into_bytes(),
        ] {
            fs::write(&epoch, bytes).unwrap();
            assert!(matches!(
                sample(work.path()),
                ActivityObservation::Unavailable(_)
            ));
        }
        fs::remove_file(&epoch).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::create_dir(&epoch).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_dir(&epoch).unwrap();
        let path = std::ffi::CString::new(epoch.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_file(&epoch).unwrap();
        fs::write(&epoch, original).unwrap();
        fs::write(&lease.lease_path, vec![b'x'; 65537]).unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        fs::remove_file(&lease.lease_path).unwrap();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
    }

    #[test]
    fn aliases_match_by_identity_and_subdirectories_do_not_borrow_runtime() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease) = fixture();
        let alias_parent = TempDir::new().unwrap();
        let alias = alias_parent.path().join("alias");
        std::os::unix::fs::symlink(work.path(), &alias).unwrap();
        assert_eq!(sample(&alias), ActivityObservation::Idle);
        lease
            .activate_session_epoch(&lease.control_dir.join("signal"))
            .unwrap();
        assert!(matches!(
            sample(&alias),
            ActivityObservation::Unavailable(_)
        ));
        let sub = work.path().join("sub");
        fs::create_dir(&sub).unwrap();
        assert_eq!(
            crate::try_observe(&sub, &[]).activity,
            ActivityObservation::Idle
        );
        let other = TempDir::new().unwrap();
        fs::create_dir(other.path().join(".jj")).unwrap();
        std::os::unix::fs::symlink(&lease.control_dir, other.path().join(".jj/grove")).unwrap();
        assert!(matches!(
            crate::try_observe(other.path(), &[]).activity,
            ActivityObservation::Unavailable(_)
        ));
    }

    #[test]
    fn descriptor_flags_and_bounded_reads_are_enforced() {
        let (work, lease) = fixture();
        for (path, directory) in [
            (work.path(), true),
            (lease.control_dir.as_path(), true),
            (lease.lease_path.as_path(), false),
        ] {
            let file = open(path, directory).unwrap();
            let flags = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETFL) };
            assert_eq!(flags & libc::O_ACCMODE, libc::O_RDONLY);
            assert_ne!(flags & libc::O_NONBLOCK, 0);
            assert_ne!(
                unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETFD) } & libc::FD_CLOEXEC,
                0
            );
            assert!(current(&file, path).unwrap());
        }
        fs::write(&lease.lease_path, vec![b'a'; 65536]).unwrap();
        assert_eq!(
            bounded_record(&mut open(&lease.lease_path, false).unwrap())
                .unwrap()
                .len(),
            65536
        );
        fs::write(&lease.lease_path, vec![b'a'; 65537]).unwrap();
        assert!(bounded_record(&mut open(&lease.lease_path, false).unwrap()).is_err());
    }

````
<!-- /fragment -->



### Pauses at the two guard boundaries

Readiness and release channels suspend the same typed operation after capture and inside the epoch guard. A driver and tree writer proceed during the first pause. During the second, other shared observers and tree writers still proceed, while exclusive handoff reaches the existing 30-second bound through the driver loop’s deterministic clock seam and reports contention once. Releasing the observer permits handoff while captured values remain alive. Ten-second channel timeouts bound broken barriers; the test does not wait thirty wall-clock seconds.

<!-- fragment «runtime-test-guards» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="737-834" parent="runtime-observer" -->
````rust
    #[test]
    fn capture_pause_and_returned_values_hold_no_epoch_or_tree_lock() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease) = fixture();
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        std::thread::scope(|scope| {
            let path = work.path();
            let observer = scope.spawn(move || {
                crate::observation::observe_with(
                    path,
                    &[],
                    || {
                        ready_tx.send(()).unwrap();
                        release_rx.recv_timeout(Duration::from_secs(10)).unwrap();
                    },
                    || {},
                )
            });
            ready_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            lease
                .activate_session_epoch(&lease.control_dir.join("signal"))
                .unwrap();
            let tree_writer = File::open(work.path()).unwrap();
            assert_eq!(
                unsafe { libc::flock(tree_writer.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
            drop(tree_writer);
            release_tx.send(()).unwrap();
            let capture = observer.join().unwrap();
            lease.invalidate_session_epoch().unwrap();
            assert!(matches!(
                capture.activity,
                ActivityObservation::Unavailable(_)
            ));
            assert!(matches!(
                capture.tree.unwrap(),
                crate::TreeObservation::Ready(_)
            ));
        });
    }

    #[test]
    fn paused_runtime_reader_allows_shared_observers_and_bounds_driver_handoff() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease) = fixture();
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        std::thread::scope(|scope| {
            let path = work.path();
            let observer = scope.spawn(move || {
                crate::observation::observe_with(
                    path,
                    &[],
                    || {},
                    || {
                        ready_tx.send(()).unwrap();
                        release_rx.recv_timeout(Duration::from_secs(10)).unwrap();
                    },
                )
            });
            ready_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            assert_eq!(sample(work.path()), ActivityObservation::Idle);
            let tree_writer = File::open(work.path()).unwrap();
            assert_eq!(
                unsafe { libc::flock(tree_writer.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
            drop(tree_writer);
            let elapsed = std::cell::Cell::new(Duration::ZERO);
            let reports = std::cell::Cell::new(0);
            let start = Instant::now();
            let error = acquire_epoch_file_with(
                &lease.control_dir.join(EPOCH_FILE_NAME),
                LockMode::Exclusive,
                "post-reap invalidation",
                EPOCH_HANDOFF_TIMEOUT,
                || start + elapsed.get(),
                || elapsed.set(elapsed.get() + Duration::from_secs(10)),
                |_, _| Ok(()),
                |_, _| Ok(()),
                || reports.set(reports.get() + 1),
            )
            .unwrap_err();
            assert_eq!(elapsed.get(), Duration::from_secs(30));
            assert_eq!(reports.get(), 1);
            assert!(error.to_string().contains("timed out after 30s"));
            release_tx.send(()).unwrap();
            let retained = observer.join().unwrap();
            lease.invalidate_session_epoch().unwrap();
            assert_eq!(retained.activity, ActivityObservation::Idle);
        });
    }
````
<!-- /fragment -->

### Replacement and read-only controls

Replacing the epoch once forces a second attempt; replacing it on every guarded read exhausts exactly eight. Recursive snapshots compare file bytes and directory entries, including the administration area, across idle and legacy-active samples. The final control checks unreadable epochs where permissions apply and rejects a namespace replaced by a regular file. These controls exercise the observer’s own acquisition path rather than a parallel test implementation.

<!-- fragment «runtime-test-races» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="835-2172" parent="runtime-observer" -->
<!-- insert «runtime-test-epoch-replacements» -->
<!-- insert «runtime-test-started-fixture» -->
<!-- insert «runtime-test-real-launch» -->
<!-- insert «runtime-test-io-errors» -->
<!-- insert «runtime-test-tree-binding» -->
<!-- insert «runtime-test-markers» -->
<!-- insert «runtime-test-probe-precedence» -->
<!-- insert «runtime-test-directory-release» -->
<!-- insert «runtime-test-private-replacements» -->
<!-- insert «runtime-test-release-orders» -->
<!-- insert «runtime-test-viewer-workers» -->
<!-- insert «runtime-test-shared-overlap» -->
<!-- insert «runtime-test-repeated-launches» -->
<!-- insert «runtime-test-concurrent-snapshots» -->
<!-- insert «runtime-test-delayed-replacement» -->
<!-- insert «runtime-test-preparation-order» -->
<!-- insert «runtime-test-filesystem-preservation» -->
<!-- insert «runtime-native-exec» -->
<!-- insert «runtime-native-holder» -->
<!-- insert «runtime-native-reap» -->
<!-- insert «runtime-native-scenarios» -->
<!-- /fragment -->

This control replaces the epoch under the runtime pause seam. One replacement recovers on the next attempt; eight replacements exhaust the bound. It exercises the production outer retry loop independently of private-file replacement.

<!-- fragment «runtime-test-epoch-replacements» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="835-868" parent="runtime-test-races" -->
````rust
    #[test]
    fn identity_replacements_retry_and_stop_after_eight_attempts() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease) = fixture();
        let epoch = lease.control_dir.join(EPOCH_FILE_NAME);
        let bytes = fs::read(&epoch).unwrap();
        for replacements in [1, 8] {
            let mut attempts = 0;
            let result = crate::observation::observe_with(
                work.path(),
                &[],
                || {},
                || {
                    attempts += 1;
                    if attempts <= replacements {
                        fs::rename(&epoch, epoch.with_extension("old")).unwrap();
                        fs::write(&epoch, &bytes).unwrap();
                    }
                },
            );
            if replacements == 1 {
                assert_eq!(result.activity, ActivityObservation::Idle);
                assert_eq!(attempts, 2);
            } else {
                assert!(
                    matches!(result.activity, ActivityObservation::Unavailable(ref reason) if reason.contains("8 observation attempts"))
                );
                assert_eq!(attempts, 8);
            }
        }
    }

````
<!-- /fragment -->

The shared fixture prepares a real lease-owned pair and publishes Started. The assertion helper requires a typed Running result; conservative blanket unavailability cannot satisfy the following controls.

<!-- fragment «runtime-test-started-fixture» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="869-889" parent="runtime-test-races" -->
````rust
    fn started_fixture() -> (TempDir, DriverLease) {
        let (work, mut lease) = fixture();
        let root = TreeLifetime::open(work.path()).unwrap().unwrap();
        lease
            .prepare_launch(
                root,
                &super::super::tests::witness_selection(),
                &lease.control_dir.join("signal-test"),
            )
            .unwrap();
        lease.launch.as_mut().unwrap().started().unwrap();
        (work, lease)
    }

    fn running(activity: ActivityObservation) -> RunningMandate {
        let ActivityObservation::Running(mandate) = activity else {
            panic!("expected Running, got {activity:?}")
        };
        mandate
    }

````
<!-- /fragment -->

The generic runner actually spawns and reaps a shell. Its parent callbacks first notify the lease and then sample the public observer: Started must report the prepared handle and SameTree, while Reaped must report Idle. Filesystem snapshots around each sample show that observing changes no bytes. This host control is not the cross-platform killed-holder and exec-survivor evidence owned by the later native-process leaf.

<!-- fragment «runtime-test-real-launch» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="890-956" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_real_launch_started_and_reaped_reach_public_observation() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = fixture();
        let channel = keyed_launch::Channel::allocate(&lease.control_dir).unwrap();
        let root = TreeLifetime::open(work.path()).unwrap().unwrap();
        lease
            .prepare_launch(
                root,
                &super::super::tests::witness_selection(),
                channel.path(),
            )
            .unwrap();
        assert!(matches!(sample(work.path()), ActivityObservation::Busy(_)));
        let config = work.path().join("launch.kdl");
        fs::write(&config, "config { command \"run\" \"/bin/sh -c true\"; bind \"run\" \"run\"; route \"test\" \"run\"; }\n").unwrap();
        let templates =
            keyed_launch::Templates::load(&config, None, keyed_launch::Vocabulary { slots: &[] })
                .unwrap();
        let argv = templates.expand("test", &[]).unwrap();
        let mut events = Vec::new();
        lease
            .supervise_launch(|notify| {
                keyed_launch::run_observed(
                    keyed_launch::Launch {
                        argv: &argv,
                        channel: &channel,
                        channel_var: "GROVE_SIGNAL_FILE",
                        scrub: &[],
                        cwd: Some(work.path()),
                        escalation: keyed_launch::Escalation {
                            grace: Duration::ZERO,
                            kill_grace: Duration::ZERO,
                        },
                    },
                    &mut |event| {
                        notify(event);
                        events.push(event);
                        let before = contents(work.path());
                        let activity = sample(work.path());
                        assert_eq!(contents(work.path()), before);
                        match event {
                            keyed_launch::LaunchEvent::Started => {
                                let mandate = running(activity);
                                assert_eq!(mandate.handle.to_string(), "work-k1");
                                assert_eq!(mandate.relation, TreeRelation::SameTree);
                                assert_eq!(mandate.runtime.signal, channel.path());
                            }
                            keyed_launch::LaunchEvent::Reaped => {
                                assert_eq!(activity, ActivityObservation::Idle)
                            }
                        }
                    },
                )
            })
            .unwrap();
        assert_eq!(
            events,
            [
                keyed_launch::LaunchEvent::Started,
                keyed_launch::LaunchEvent::Reaped
            ]
        );
    }

````
<!-- /fragment -->

Injected errors at private open and each path-identity read must return through the production runtime error path and release the epoch guard. A subsequent native positive sample must still identify the live launch. The injection changes filesystem operations, not the returned activity provider.

<!-- fragment «runtime-test-io-errors» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="957-1007" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_private_open_and_identity_io_errors_release_the_epoch_guard() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        struct FailingIo {
            fail_at: usize,
            calls: usize,
        }
        impl FailingIo {
            fn check(&mut self) -> Result<()> {
                self.calls += 1;
                anyhow::ensure!(
                    self.calls != self.fail_at,
                    "injected witness filesystem error"
                );
                Ok(())
            }
        }
        impl WitnessIo for FailingIo {
            fn open(&mut self, path: &Path) -> Result<File> {
                self.check()?;
                NativeWitnessIo.open(path)
            }
            fn current(&mut self, file: &File, path: &Path) -> Result<bool> {
                self.check()?;
                NativeWitnessIo.current(file, path)
            }
        }
        let (work, _lease) = started_fixture();
        let pin = TreeLifetime::open(work.path()).unwrap().unwrap();
        for fail_at in 1..=3 {
            let mut io = FailingIo { fail_at, calls: 0 };
            let error = read_runtime(work.path(), Some(&pin), || {}, &mut io).unwrap_err();
            assert!(error
                .to_string()
                .contains("injected witness filesystem error"));
            assert_eq!(io.calls, fail_at);
            assert!(acquire_epoch_file(
                &work.path().join(".jj/grove/session.epoch"),
                LockMode::Exclusive,
                "test"
            )
            .is_ok());
            assert_eq!(
                running(sample(work.path())).relation,
                TreeRelation::SameTree
            );
        }
    }

````
<!-- /fragment -->

Replacing the path after tree capture must preserve SameTree for the already-captured old directory; reopening the path here would return the wrong relation. A new observer instead reports PreviousTree with the original runtime identity. Separate busy, duplicate-key and removed-tree cases require NoReadableTree while preserving the launch identity for a summary.

<!-- fragment «runtime-test-tree-binding» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1008-1064" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_capture_uses_accepted_pin_after_root_path_replacement() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, _lease) = started_fixture();
        let original = running(sample(work.path()));
        let captured = crate::observation::observe_with(
            work.path(),
            &[],
            || {
                fs::rename(work.path().join(".grove"), work.path().join("old")).unwrap();
                fs::create_dir(work.path().join(".grove")).unwrap();
                fs::write(work.path().join(".grove/_BRIEF.md"), "replacement").unwrap();
            },
            || {},
        );
        assert_eq!(running(captured.activity), original);
        let crate::TreeObservation::Ready(tree) = captured.tree.unwrap() else {
            panic!()
        };
        assert!(!tree.lifetime.at(work.path()).unwrap());
        assert_eq!(tree.content.unwrap(), b"root");
        let replacement = running(sample(work.path()));
        assert_eq!(replacement.relation, TreeRelation::PreviousTree);
        assert_eq!(replacement.tree_identity, original.tree_identity);
        assert_eq!(replacement.runtime, original.runtime);
    }

    #[test]
    fn witness_tree_absence_failure_and_contention_preserve_runtime_identity() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, _lease) = started_fixture();
        let original = running(sample(work.path()));
        let check = || {
            let mandate = running(crate::try_observe(work.path(), &[]).activity);
            assert_eq!(mandate.relation, TreeRelation::NoReadableTree);
            assert_eq!(mandate.runtime, original.runtime);
            assert_eq!(mandate.tree_identity, original.tree_identity);
        };
        let writer = File::open(work.path()).unwrap();
        assert_eq!(
            unsafe { libc::flock(writer.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
            0
        );
        check();
        drop(writer);
        fs::write(work.path().join(".grove/01-impl--work-k1.md"), "").unwrap();
        fs::write(work.path().join(".grove/02-impl--duplicate-k1.md"), "").unwrap();
        assert!(crate::try_observe(work.path(), &[]).tree.is_err());
        check();
        fs::remove_dir_all(work.path().join(".grove")).unwrap();
        check();
    }

````
<!-- /fragment -->

Every proper prefix of the eight-byte marker is Busy under the real private lock. Invalid bytes, a trailing byte and an oversized file are Unavailable; the bounded read consumes only nine bytes. Releasing the owner makes the same oversized leftovers Idle. The isolated test process prevents unrelated forks from extending its descriptors.

<!-- fragment «runtime-test-markers» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1065-1112" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_marker_prefixes_invalid_bytes_and_release_have_exact_precedence() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = started_fixture();
        let path = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        for length in 0..8 {
            fs::write(&path, &b"started\n"[..length]).unwrap();
            assert!(
                matches!(sample(work.path()), ActivityObservation::Busy(_)),
                "{length}"
            );
        }
        for marker in [
            b"started\n!".as_slice(),
            b"started",
            b"stax",
            b"\xff",
            b"started\0",
        ] {
            fs::write(&path, marker).unwrap();
            let activity = sample(work.path());
            if marker == b"started" {
                assert!(matches!(activity, ActivityObservation::Busy(_)));
            } else {
                assert!(
                    matches!(activity, ActivityObservation::Unavailable(_)),
                    "{marker:?}: {activity:?}"
                );
            }
        }
        fs::write(&path, vec![b'x'; 100_000]).unwrap();
        assert_eq!(
            NativeWitnessIo
                .marker(&mut File::open(&path).unwrap())
                .unwrap()
                .len(),
            9
        );
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
        lease.launch.take();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
    }

````
<!-- /fragment -->

The operation-level matrix varies both directory and private probe outcomes and marker-read failure. It checks the event order directory, marker, private, then the actual runtime verdict. Private release must override every earlier outcome. Each case reacquires the epoch exclusively after return, demonstrating that even failure paths release its guard.

The matrix runs in the existing isolated test process: its repeated exclusive
epoch probes must not be inherited by a concurrently forking test. Otherwise a
brief inherited lock can make runtime observation return Busy before the matrix
reaches any witness probe. Isolation retains every precedence assertion.

<!-- fragment «runtime-test-probe-precedence» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1113-1194" parent="runtime-test-races" -->
````rust
    #[derive(Clone, Copy, Debug)]
    enum ProbeResult {
        Released,
        Held,
        Error,
    }

    struct ProbeCases {
        directory: ProbeResult,
        private: ProbeResult,
        marker_error: bool,
        events: Vec<&'static str>,
    }

    impl WitnessIo for ProbeCases {
        fn marker(&mut self, file: &mut File) -> Result<Vec<u8>> {
            self.events.push("marker");
            if self.marker_error {
                bail!("injected marker read error");
            }
            NativeWitnessIo.marker(file)
        }
        fn probe(&mut self, file: &File) -> Result<bool> {
            let result = if file.metadata()?.is_dir() {
                self.events.push("directory");
                self.directory
            } else {
                self.events.push("private");
                self.private
            };
            match result {
                ProbeResult::Released => Ok(true),
                ProbeResult::Held => Ok(false),
                ProbeResult::Error => Err(std::io::Error::from_raw_os_error(libc::ENOLCK).into()),
            }
        }
    }

    #[test]
    fn witness_directory_private_and_marker_error_matrix_obeys_final_probe() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, _lease) = started_fixture();
        let pin = TreeLifetime::open(work.path()).unwrap().unwrap();
        for directory in [ProbeResult::Released, ProbeResult::Held, ProbeResult::Error] {
            for private in [ProbeResult::Released, ProbeResult::Held, ProbeResult::Error] {
                for marker_error in [false, true] {
                    let mut io = ProbeCases {
                        directory,
                        private,
                        marker_error,
                        events: vec![],
                    };
                    let result = read_runtime(work.path(), Some(&pin), || {}, &mut io);
                    assert_eq!(io.events, ["directory", "marker", "private"]);
                    match (private, marker_error, directory) {
                        (ProbeResult::Released, _, _) => {
                            assert_eq!(result.unwrap(), ActivityObservation::Idle)
                        }
                        (ProbeResult::Error, _, _) | (_, true, _) | (_, _, ProbeResult::Error) => {
                            assert!(result.is_err())
                        }
                        (_, _, ProbeResult::Released) => {
                            assert!(matches!(result.unwrap(), ActivityObservation::Busy(_)))
                        }
                        (_, _, ProbeResult::Held) => {
                            assert_eq!(running(result.unwrap()).relation, TreeRelation::SameTree)
                        }
                    }
                    // Every result drops the shared epoch guard.
                    assert!(acquire_epoch_file(
                        &work.path().join(".jj/grove/session.epoch"),
                        LockMode::Exclusive,
                        "test"
                    )
                    .is_ok());
                }
            }
        }
    }

````
<!-- /fragment -->

A real matching directory is explicitly unlocked while its private witness remains held. The public capture must be Busy. Independent exclusive probes succeed while returned captures remain alive, proving that successful directory, private and epoch probes did not escape as advisory guards. The captures still retain their directory pins.

<!-- fragment «runtime-test-directory-release» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1195-1230" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_unlocked_matching_directory_is_busy_and_probes_escape_no_locks() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = started_fixture();
        let path = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        let directory = lease.launch.as_ref().unwrap().root.directory();
        assert_eq!(
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_UN) },
            0
        );
        let capture = crate::try_observe(work.path(), &[]);
        assert!(matches!(capture.activity, ActivityObservation::Busy(_)));
        let probe = File::open(work.path().join(".grove")).unwrap();
        assert_eq!(
            unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
            0
        );
        drop(probe);
        lease.launch.take();
        let idle = crate::try_observe(work.path(), &[]);
        assert_eq!(idle.activity, ActivityObservation::Idle);
        for path in [
            path,
            work.path().join(".grove"),
            work.path().join(".jj/grove/session.epoch"),
        ] {
            let probe = File::open(path).unwrap();
            assert_eq!(
                unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
        }
    }

````
<!-- /fragment -->

The replacement backend moves actual private-file paths after open or after the native shared probe. A stable wrong object is rejected; restoring the published object allows recovery; continued replacement stops after eight attempts. Both objects remain alive throughout, so the control depends on descriptor identity rather than inode reuse. It also checks that observation did not rewrite the epoch.

<!-- fragment «runtime-test-private-replacements» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1231-1330" parent="runtime-test-races" -->
````rust
    #[derive(Clone, Copy, Debug)]
    enum ReplacementPoint {
        Open,
        Probe,
    }

    struct ReplacingIo {
        point: ReplacementPoint,
        path: PathBuf,
        original: PathBuf,
        substitute: PathBuf,
        remaining: usize,
        opens: usize,
        recover: bool,
    }

    impl ReplacingIo {
        fn replace(&mut self) {
            if self.remaining == 0 {
                return;
            }
            self.remaining -= 1;
            // Alternate paths while keeping both objects alive. Restoring the
            // published object on the second attempt proves retry can recover.
            if self.original.exists() {
                fs::rename(&self.path, &self.substitute).unwrap();
                fs::rename(&self.original, &self.path).unwrap();
            } else {
                fs::rename(&self.path, &self.original).unwrap();
                fs::rename(&self.substitute, &self.path).unwrap();
            }
        }
    }

    impl WitnessIo for ReplacingIo {
        fn open(&mut self, path: &Path) -> Result<File> {
            self.opens += 1;
            if matches!(self.point, ReplacementPoint::Probe)
                && self.recover
                && self.original.exists()
            {
                fs::rename(&self.path, &self.substitute)?;
                fs::rename(&self.original, &self.path)?;
            }
            let file = NativeWitnessIo.open(path)?;
            if matches!(self.point, ReplacementPoint::Open) {
                self.replace();
            }
            Ok(file)
        }
        fn probe(&mut self, file: &File) -> Result<bool> {
            let result = NativeWitnessIo.probe(file);
            if !file.metadata()?.is_dir() && matches!(self.point, ReplacementPoint::Probe) {
                self.replace();
            }
            result
        }
    }

    #[test]
    fn witness_open_and_probe_replacements_reject_stale_descriptors_and_bound_retries() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        for point in [ReplacementPoint::Open, ReplacementPoint::Probe] {
            for remaining in [1, 2, 8] {
                let (work, _lease, path, epoch_path, record) = released_fixture();
                let original = path.with_extension("original");
                let substitute = path.with_extension("substitute");
                fs::write(&substitute, "started\n").unwrap();
                let mut io = ReplacingIo {
                    point,
                    path,
                    original,
                    substitute,
                    remaining,
                    opens: 0,
                    recover: remaining > 1,
                };
                let result = read_runtime(work.path(), None, || {}, &mut io);
                if remaining == 2 {
                    assert_eq!(result.unwrap(), ActivityObservation::Idle);
                    assert_eq!(io.opens, 3);
                } else {
                    let message = format!("{:#}", result.unwrap_err());
                    assert!(
                        message.contains(if remaining == 8 {
                            "8 observation attempts"
                        } else {
                            "identity does not match"
                        }),
                        "{message}"
                    );
                    assert_eq!(io.opens, if remaining == 8 { 8 } else { 2 });
                }
                assert_eq!(fs::read_to_string(epoch_path).unwrap(), record);
            }
        }
    }

````
<!-- /fragment -->

### Independent witness release orders

The release fixture starts with DriverLease's actual epoch and Started marker,
then transfers exclusive ownership to independently droppable native lock holders.
A Running sample proves that these holders satisfy the production observer before
any release. Directory-first release removes the old tree and creates another
leaf with key 1. Rewriting only the fixture's recorded tree numbers forces the
identity-reuse premise: the unlocked replacement must produce Busy despite the
old private witness. This models numeric reuse; it does not measure host inode
allocation or pause kernel teardown. Removing directory verification makes this
control falsely return SameTree Running while the real-launch positive still
passes.

Private-first and both-released samples must be Idle. The filesystem/lock seam
also releases both holders immediately after a contended directory probe, before
the final private probe; Idle must override that earlier SameTree evidence.
An ordered event trace proves this schedule without sleeps. Retained captures
permit independent exclusive probes of every relevant lock, and recursive byte
snapshots show that these observations write nothing. Replacement epoch ordering
and concurrent observers have their own controls; these release cases do not
establish those properties.

<!-- fragment «runtime-test-release-orders» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1331-1491" parent="runtime-test-races" -->
````rust
    /// Reproduce independent descriptor teardown with real native locks. The
    /// epoch/marker come from DriverLease; only the holders are test-controlled.
    fn release_fixture() -> (TempDir, DriverLease, Option<File>, Option<File>) {
        let (work, mut lease) = started_fixture();
        fs::write(work.path().join(".grove/01-impl--work-k1.md"), "old").unwrap();
        let private = File::open(lease.launch.as_ref().unwrap().path().unwrap()).unwrap();
        let directory = File::open(work.path().join(".grove")).unwrap();
        lease.launch.take();
        for file in [&directory, &private] {
            assert_eq!(
                unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
        }
        assert_eq!(
            running(sample(work.path())).relation,
            TreeRelation::SameTree
        );
        (work, lease, Some(directory), Some(private))
    }

    fn assert_observation_releases_guards(work: &Path, private: &Path) {
        // Keep the returned capture (and therefore its tree pin) alive while
        // independently acquiring every advisory lock it could have retained.
        let capture = crate::try_observe(work, &[]);
        assert_eq!(capture.activity, ActivityObservation::Idle);
        for path in [
            work.to_path_buf(),
            work.join(".grove"),
            work.join(".jj/grove/session.epoch"),
            private.to_path_buf(),
        ] {
            let probe = File::open(path).unwrap();
            assert_eq!(
                unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
        }
        drop(capture);
    }

    #[test]
    fn witness_release_directory_first_reused_identity_and_key_cannot_attach() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease, mut directory, mut private) = release_fixture();
        let original = running(sample(work.path()));
        directory.take();
        fs::remove_dir_all(work.path().join(".grove")).unwrap();
        fs::create_dir(work.path().join(".grove")).unwrap();
        fs::write(work.path().join(".grove/_BRIEF.md"), "replacement").unwrap();
        fs::write(work.path().join(".grove/01-impl--replacement-k1.md"), "new").unwrap();
        let pin = TreeLifetime::open(work.path()).unwrap().unwrap();
        let metadata = pin.directory().metadata().unwrap();
        let epoch_path = lease.control_dir.join(EPOCH_FILE_NAME);
        let record = fs::read_to_string(&epoch_path).unwrap();
        // Force the ABA premise: old launch numbers equal the replacement's.
        // This is a fixture-only identity model, not observed host inode reuse.
        let record = record
            .replace(
                &format!("observation-tree-device={}", original.tree_identity.0 .0),
                &format!("observation-tree-device={}", metadata.dev()),
            )
            .replace(
                &format!("observation-tree-inode={}", original.tree_identity.0 .1),
                &format!("observation-tree-inode={}", metadata.ino()),
            );
        fs::write(&epoch_path, record).unwrap();
        let before = contents(work.path());
        let capture = crate::try_observe(work.path(), &[]);
        let crate::TreeObservation::Ready(tree) = capture.tree.unwrap() else {
            panic!("replacement tree was not captured")
        };
        assert_eq!(tree.content.unwrap(), b"replacement");
        assert!(
            matches!(capture.activity, ActivityObservation::Busy(_)),
            "released directory falsely attached old work-k1 to replacement-k1: {:?}",
            capture.activity
        );
        private.take();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        assert_observation_releases_guards(
            work.path(),
            &lease.control_dir.join(original.runtime.witness_name),
        );
        assert_eq!(contents(work.path()), before);
    }

    #[test]
    fn witness_release_private_first_and_both_released_are_idle() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, lease, mut directory, mut private) = release_fixture();
        let original = running(sample(work.path()));
        let before = contents(work.path());
        private.take();
        assert!(!NativeWitnessIo
            .probe(&File::open(work.path().join(".grove")).unwrap())
            .unwrap());
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        directory.take();
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        assert_observation_releases_guards(
            work.path(),
            &lease.control_dir.join(original.runtime.witness_name),
        );
        assert_eq!(contents(work.path()), before);
    }

    #[test]
    fn witness_release_after_directory_verification_obeys_final_private_probe() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        struct ReleaseAfterDirectory {
            directory: Option<File>,
            private: Option<File>,
            events: Vec<&'static str>,
        }
        impl WitnessIo for ReleaseAfterDirectory {
            fn probe(&mut self, file: &File) -> Result<bool> {
                let released = NativeWitnessIo.probe(file)?;
                if file.metadata()?.is_dir() {
                    assert!(!released, "positive directory evidence is required");
                    self.events.push("directory verified");
                    self.directory.take();
                    self.private.take();
                    self.events.push("both released");
                } else {
                    assert!(released, "private release must be observed");
                    self.events.push("private released");
                }
                Ok(released)
            }
        }
        let (work, lease, directory, private) = release_fixture();
        let original = running(sample(work.path()));
        let pin = TreeLifetime::open(work.path()).unwrap().unwrap();
        let before = contents(work.path());
        let mut io = ReleaseAfterDirectory {
            directory,
            private,
            events: vec![],
        };
        assert_eq!(
            read_runtime(work.path(), Some(&pin), || {}, &mut io).unwrap(),
            ActivityObservation::Idle
        );
        assert_eq!(
            io.events,
            ["directory verified", "both released", "private released"]
        );
        assert_observation_releases_guards(
            work.path(),
            &lease.control_dir.join(original.runtime.witness_name),
        );
        assert_eq!(contents(work.path()), before);
    }

````
<!-- /fragment -->

Continuous workers exercise the public observer between requested checkpoints. Each request receives a fresh observation, and channel disconnection stops the cohort even when an assertion unwinds. Bounded receives detect missing progress without using elapsed time to infer ordering.

<!-- fragment «runtime-test-viewer-workers» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1492-1530" parent="runtime-test-races" -->
````rust
    // Workers observe continuously, including between requested checkpoints.
    // A request is acknowledged only by a fresh public observation after it.
    fn with_continuous_viewers(
        paths: &[PathBuf],
        run: impl FnOnce(&dyn Fn() -> Vec<ActivityObservation>),
    ) {
        std::thread::scope(|scope| {
            let mut viewers = Vec::new();
            for path in paths {
                let (request_tx, request_rx) = mpsc::channel();
                let (result_tx, result_rx) = mpsc::channel();
                scope.spawn(move || loop {
                    let requested = match request_rx.try_recv() {
                        Ok(()) => true,
                        Err(mpsc::TryRecvError::Empty) => false,
                        Err(mpsc::TryRecvError::Disconnected) => break,
                    };
                    let capture = crate::try_observe(path, &[]);
                    if requested && result_tx.send(capture.activity).is_err() {
                        break;
                    }
                    std::thread::yield_now();
                });
                viewers.push((request_tx, result_rx));
            }
            run(&|| {
                for (request, _) in &viewers {
                    request.send(()).unwrap();
                }
                viewers
                    .iter()
                    .map(|(_, result)| result.recv_timeout(Duration::from_secs(10)).unwrap())
                    .collect()
            });
            // Disconnection stops every worker, also on assertion unwind.
            drop(viewers);
        });
    }

````
<!-- /fragment -->

Independent shared descriptors hold both released witness probe windows open while three public observers read leftover Started bytes. Native exclusive probes first confirm the holders are effective. Every observation must be Idle; after the cohort and holders stop, independent exclusive probes check that returned captures retain no advisory guards.

<!-- fragment «runtime-test-shared-overlap» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1531-1572" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_concurrent_shared_probes_of_released_started_bytes_stay_idle() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = started_fixture();
        let private_path = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        lease.launch.take();
        assert_eq!(fs::read(&private_path).unwrap(), b"started\n");
        let before = contents(work.path());
        let mut overlaps = Vec::new();
        for path in [work.path().join(".grove"), private_path.clone()] {
            // Hold the tiny successful-probe window open on independent native
            // descriptors. These are test holders, never returned viewer locks.
            for _ in 0..3 {
                let file = File::open(&path).unwrap();
                assert_eq!(
                    unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
                    0
                );
                overlaps.push(file);
            }
            let exclusive = File::open(path).unwrap();
            assert_ne!(
                unsafe { libc::flock(exclusive.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                0
            );
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::EWOULDBLOCK)
            );
        }
        with_continuous_viewers(&vec![work.path().to_path_buf(); 3], |sample_all| {
            for _ in 0..16 {
                assert_eq!(sample_all(), vec![ActivityObservation::Idle; 3]);
            }
        });
        drop(overlaps);
        assert_observation_releases_guards(work.path(), &private_path);
        assert_eq!(contents(work.path()), before);
    }

````
<!-- /fragment -->

Three continuous observers span four real launches. Each preparation must retain both exclusive witnesses; at the synchronous Started callback every viewer identifies the exact signal and SameTree mandate, and at Reaped every viewer reports Idle. Compatible probes check release while viewers continue; invalidation and the next preparation must still succeed.

<!-- fragment «runtime-test-repeated-launches» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1573-1655" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_continuous_viewers_allow_repeated_real_launch_preparation() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = fixture();
        let config = work.path().join("launch.kdl");
        fs::write(&config, "config { command \"run\" \"/bin/sh -c true\"; bind \"run\" \"run\"; route \"test\" \"run\"; }\n").unwrap();
        let templates =
            keyed_launch::Templates::load(&config, None, keyed_launch::Vocabulary { slots: &[] })
                .unwrap();
        let argv = templates.expand("test", &[]).unwrap();
        with_continuous_viewers(&vec![work.path().to_path_buf(); 3], |sample_all| {
            assert_eq!(sample_all(), vec![ActivityObservation::Idle; 3]);
            for _ in 0..4 {
                let channel = keyed_launch::Channel::allocate(&lease.control_dir).unwrap();
                let root = TreeLifetime::open(work.path()).unwrap().unwrap();
                lease
                    .prepare_launch(
                        root,
                        &super::super::tests::witness_selection(),
                        channel.path(),
                    )
                    .unwrap();
                let private = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
                for path in [work.path().join(".grove"), private.clone()] {
                    assert!(!NativeWitnessIo.probe(&File::open(path).unwrap()).unwrap());
                }
                let mut events = Vec::new();
                lease
                    .supervise_launch(|notify| {
                        keyed_launch::run_observed(
                            keyed_launch::Launch {
                                argv: &argv,
                                channel: &channel,
                                channel_var: "GROVE_SIGNAL_FILE",
                                scrub: &[],
                                cwd: Some(work.path()),
                                escalation: keyed_launch::Escalation {
                                    grace: Duration::ZERO,
                                    kill_grace: Duration::ZERO,
                                },
                            },
                            &mut |event| {
                                notify(event);
                                events.push(event);
                                let before = contents(work.path());
                                for activity in sample_all() {
                                    match event {
                                        keyed_launch::LaunchEvent::Started => {
                                            let mandate = running(activity);
                                            assert_eq!(mandate.handle.to_string(), "work-k1");
                                            assert_eq!(mandate.relation, TreeRelation::SameTree);
                                            assert_eq!(mandate.runtime.signal, channel.path());
                                        }
                                        keyed_launch::LaunchEvent::Reaped => {
                                            assert_eq!(activity, ActivityObservation::Idle);
                                        }
                                    }
                                }
                                assert_eq!(contents(work.path()), before);
                            },
                        )
                    })
                    .unwrap();
                assert_eq!(
                    events,
                    [
                        keyed_launch::LaunchEvent::Started,
                        keyed_launch::LaunchEvent::Reaped
                    ]
                );
                assert!(lease.launch.is_none());
                // Other viewers may be inside their short shared probe here;
                // use compatible probes until the worker cohort has stopped.
                for path in [work.path().join(".grove"), private] {
                    assert!(NativeWitnessIo.probe(&File::open(path).unwrap()).unwrap());
                }
                lease.invalidate_session_epoch().unwrap();
            }
        });
    }

````
<!-- /fragment -->

Concurrent readers use both the exact worktree path and a symlink alias through absent-tree, non-jj-tree, missing-namespace and jj-without-tree cases. Recursive before/after snapshots and the unchanged alias target show that observation creates neither tree nor administration entries.

<!-- fragment «runtime-test-concurrent-snapshots» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1656-1685" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_concurrent_read_only_captures_cover_aliases_and_absent_controls() {
        let parent = TempDir::new().unwrap();
        let work = parent.path().join("work");
        fs::create_dir(&work).unwrap();
        let alias = parent.path().join("alias");
        std::os::unix::fs::symlink(&work, &alias).unwrap();
        // Each stage adds only the named prerequisite; observers add nothing.
        for stage in 0..4 {
            match stage {
                1 => {
                    fs::create_dir(work.join(".grove")).unwrap();
                    fs::write(work.join(".grove/_BRIEF.md"), "root").unwrap();
                }
                2 => fs::create_dir(work.join(".jj")).unwrap(),
                3 => fs::remove_dir_all(work.join(".grove")).unwrap(),
                _ => {}
            }
            let before = contents(&work);
            let paths = [work.clone(), alias.clone(), work.clone()];
            with_continuous_viewers(&paths, |sample_all| {
                for _ in 0..16 {
                    assert_eq!(sample_all(), vec![ActivityObservation::Idle; 3]);
                }
            });
            assert_eq!(contents(&work), before);
            assert_eq!(fs::read_link(&alias).unwrap(), work);
        }
    }

````
<!-- /fragment -->

<a id="epoch-before-witness-preparation"></a>
### Epoch ownership precedes witness preparation

A replacement may own the lease while an old reader still holds the shared epoch guard. The first control pauses real `DriverLease::acquire_with` at its handoff boundary. Independent nonblocking probes establish lease ownership and epoch exclusion; three continuous public observers read released Started bytes as Idle. Both witness locations remain exclusively acquirable, and the filesystem snapshot retains the old lease and epoch bytes. Releasing the reader permits invalidation and cleanup; no elapsed delay is used to infer ordering.

<!-- fragment «runtime-test-delayed-replacement» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1686-1755" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_epoch_replacement_waits_with_old_bytes_and_idle_viewers() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease) = started_fixture();
        let private = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        lease.launch.take();
        let epoch_path = lease.control_dir.join(EPOCH_FILE_NAME);
        let lease_path = lease.lease_path.clone();
        let epoch_reader = File::open(&epoch_path).unwrap();
        assert_eq!(
            unsafe { libc::flock(epoch_reader.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        let before = contents(work.path());
        let old_lease = fs::read(&lease_path).unwrap();
        drop(lease);
        let (ready_tx, ready_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        std::thread::scope(|scope| {
            let root = work.path();
            scope.spawn(move || {
                let result = DriverLease::acquire_with(&Workspace::resolve(root).unwrap(), || {
                    ready_tx.send(()).unwrap();
                    release_rx.recv_timeout(Duration::from_secs(10)).unwrap();
                });
                done_tx.send(result).unwrap();
            });
            ready_rx.recv_timeout(Duration::from_secs(10)).unwrap();
            // The replacement owns the lease, but the old reader still
            // excludes invalidation. Neither observation witness is held.
            for path in [&lease_path, &epoch_path] {
                let file = File::open(path).unwrap();
                assert_ne!(
                    unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                    0
                );
                assert_eq!(
                    std::io::Error::last_os_error().raw_os_error(),
                    Some(libc::EWOULDBLOCK)
                );
            }
            with_continuous_viewers(&vec![root.to_path_buf(); 3], |sample_all| {
                for _ in 0..8 {
                    assert_eq!(sample_all(), vec![ActivityObservation::Idle; 3]);
                }
            });
            for path in [root.join(".grove"), private.clone()] {
                let file = File::open(path).unwrap();
                assert_eq!(
                    unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                    0
                );
            }
            assert_eq!(contents(root), before);
            release_tx.send(()).unwrap();
            drop(epoch_reader);
            let replacement = done_rx
                .recv_timeout(Duration::from_secs(10))
                .unwrap()
                .unwrap();
            assert_ne!(fs::read(&lease_path).unwrap(), old_lease);
            assert!(!private.exists(), "cleanup follows exclusive invalidation");
            assert_eq!(sample(root), ActivityObservation::Idle);
            drop(replacement);
        });
    }

````
<!-- /fragment -->

The preparation control supplies the separate old-record hazard. It releases the old directory witness, retains the old private witness, replaces the tree with a reused key, and rewrites only the fixture epoch's numeric tree identity. This models reuse; it does not claim host inode reuse. The retained lease reaches `prepare_launch_with` with an old active record, while the preceding control exercises actual replacement acquisition.

At the acquisition callback, a shared epoch reader still excludes invalidation. Concurrent public observers must report Busy, the new directory must be unlocked, and no private witness may have been created. Moving the production `launch.prepare` block before acquisition makes those observers attach the old mandate as SameTree Running to the new tree. The independent real-launch positive still passes under that mutation. With production restored, the callback releases the reader, preparation acquires both witnesses, and returned observations retain no locks or filesystem changes.

<!-- fragment «runtime-test-preparation-order» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1756-1840" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_epoch_preparation_cannot_attach_old_mandate_to_reused_tree() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (work, mut lease, mut directory, private) = release_fixture();
        let original = running(sample(work.path()));
        directory.take();
        fs::remove_dir_all(work.path().join(".grove")).unwrap();
        fs::create_dir(work.path().join(".grove")).unwrap();
        fs::write(work.path().join(".grove/_BRIEF.md"), "replacement").unwrap();
        fs::write(work.path().join(".grove/01-impl--replacement-k1.md"), "new").unwrap();
        let pin = TreeLifetime::open(work.path()).unwrap().unwrap();
        let metadata = pin.directory().metadata().unwrap();
        let epoch_path = lease.control_dir.join(EPOCH_FILE_NAME);
        // Model numeric reuse, never assert that this host reused an inode.
        // The retained lease lets the preparation seam see an old active
        // record; actual replacement acquisition is covered separately above.
        let record = fs::read_to_string(&epoch_path)
            .unwrap()
            .replace(
                &format!("observation-tree-device={}", original.tree_identity.0 .0),
                &format!("observation-tree-device={}", metadata.dev()),
            )
            .replace(
                &format!("observation-tree-inode={}", original.tree_identity.0 .1),
                &format!("observation-tree-inode={}", metadata.ino()),
            );
        fs::write(&epoch_path, &record).unwrap();
        let reader = File::open(&epoch_path).unwrap();
        assert_eq!(
            unsafe { libc::flock(reader.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        let before = contents(work.path());
        let signal = lease.control_dir.join("signal-replacement");
        lease
            .prepare_launch_with(
                pin,
                &super::super::tests::witness_selection(),
                &signal,
                |path| {
                    // This callback runs exactly at acquisition, so an early prepare
                    // mutant has already exposed its new directory witness here.
                    with_continuous_viewers(&vec![work.path().to_path_buf(); 3], |sample_all| {
                        for activity in sample_all() {
                            assert!(matches!(activity, ActivityObservation::Busy(_)),
                        "early preparation attached old work-k1 to replacement-k1: {activity:?}");
                        }
                    });
                    assert!(NativeWitnessIo.probe(&File::open(work.path().join(".grove"))?)?);
                    assert_eq!(
                        contents(work.path()),
                        before,
                        "no new private witness before invalidation"
                    );
                    let probe = File::open(path)?;
                    assert_ne!(
                        unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                        0
                    );
                    assert_eq!(
                        std::io::Error::last_os_error().raw_os_error(),
                        Some(libc::EWOULDBLOCK)
                    );
                    drop(reader);
                    super::super::acquire_epoch_file(path, LockMode::Exclusive, "test preparation")
                },
            )
            .unwrap();
        let new_private = lease.launch.as_ref().unwrap().path().unwrap().to_path_buf();
        assert!(!NativeWitnessIo
            .probe(&File::open(work.path().join(".grove")).unwrap())
            .unwrap());
        assert!(!NativeWitnessIo
            .probe(&File::open(&new_private).unwrap())
            .unwrap());
        assert_ne!(fs::read_to_string(&epoch_path).unwrap(), record);
        drop(private);
        lease.launch.take();
        let after = contents(work.path());
        assert_observation_releases_guards(work.path(), &new_private);
        assert_eq!(contents(work.path()), after);
    }

````
<!-- /fragment -->

Recursive snapshots include tree and administration bytes for idle and legacy-active samples, absent controls and non-jj locations. Permission and namespace-type failures remain Unavailable. These existing controls complement the started-launch snapshots without granting observation cleanup or repair authority.

<!-- fragment «runtime-test-filesystem-preservation» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1841-1907" parent="runtime-test-races" -->
````rust
    fn contents(path: &Path) -> Vec<(PathBuf, Vec<u8>)> {
        let mut files = Vec::new();
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                files.push((path.clone(), Vec::new()));
                files.extend(contents(&path));
            } else {
                files.push((path.clone(), fs::read(path).unwrap()));
            }
        }
        files.sort();
        files
    }

    #[test]
    fn observations_preserve_all_tree_and_administration_bytes() {
        let (work, lease) = fixture();
        for active in [false, true] {
            if active {
                lease
                    .activate_session_epoch(&lease.control_dir.join("signal"))
                    .unwrap();
            }
            let before = contents(work.path());
            let _ = sample(work.path());
            let _ = sample(work.path());
            assert_eq!(contents(work.path()), before);
        }
        drop(lease);
        let work = TempDir::new().unwrap();
        for namespace in [false, true] {
            if namespace {
                fs::create_dir(work.path().join(".jj")).unwrap();
            }
            let before = contents(work.path());
            assert_eq!(
                crate::try_observe(work.path(), &[]).activity,
                ActivityObservation::Idle
            );
            assert_eq!(contents(work.path()), before);
        }
    }

    #[test]
    fn unreadable_epoch_and_nondirectory_namespace_are_unavailable() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        use std::os::unix::fs::PermissionsExt;
        let (work, lease) = fixture();
        let epoch = lease.control_dir.join(EPOCH_FILE_NAME);
        fs::set_permissions(&epoch, fs::Permissions::from_mode(0o0)).unwrap();
        if unsafe { libc::geteuid() } != 0 {
            assert!(matches!(
                sample(work.path()),
                ActivityObservation::Unavailable(_)
            ));
        }
        fs::set_permissions(&epoch, fs::Permissions::from_mode(0o600)).unwrap();
        fs::rename(&lease.control_dir, lease.control_dir.with_extension("old")).unwrap();
        fs::write(&lease.control_dir, "not a namespace").unwrap();
        assert!(matches!(
            sample(work.path()),
            ActivityObservation::Unavailable(_)
        ));
    }
````
<!-- /fragment -->



<a id="paired-witness-controls"></a>
## Paired witness controls

The tests create real directories and private files and probe them through independently opened descriptors. They exercise close-on-exec flags, the private-close operation while the directory remains locked, bounded occupied draws without truncation, random and open failures, foreign shared holders of either witness, reported lock failures, and cleanup grammar with an explicitly retained file. Fork-sensitive release checks run in the existing isolated harness. The lease tests earlier in this chapter additionally exercise real successful and failed launches, admission under foreign directory contention, Reaped callbacks, unwind/drop, and the old-reader acquisition barrier. These are native macOS lock controls, not the two-platform observer or forced-reuse evidence required by the later observation increment.

The marker controls check duplicate Started calls and a failed write followed
by a duplicate: neither can rewrite the file. A read-only private descriptor
injects a real publication failure during a configured child launch. Independent
shared probes still find both locks held at Started and released at Reaped;
exclusive epoch and containing-directory probes succeed inside both callbacks.
Admission remains usable and the successful runner result survives the warning.

<!-- fragment «witness-tests» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/witnesses.rs" lines="176-722" parent="source-witnesses" -->
<!-- insert «witness-process-holder» -->
<!-- insert «witness-foreign-launch» -->
<!-- insert «witness-local-controls» -->
<!-- /fragment -->

<a id="foreign-shared-processes"></a>
### Foreign shared processes

The self-exec holder opens its own descriptor and acknowledges only after
acquiring a native shared lock. `ForeignHolder` drains stdout, bounds readiness,
and owns kill/reap cleanup on failure. A stdin byte releases the normal path;
`try_wait` confirms exit before the parent credits lock release. The helper
returns immediately when invoked without its private test environment. The two
scenario tests below exercise it with that environment, so merely listing or
running the helper alone is no process evidence.

<!-- fragment «witness-process-holder» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/witnesses.rs" lines="176-260" parent="witness-tests" -->
````rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Read};
    use std::os::unix::fs::PermissionsExt;
    use std::process::{Child, Command, Stdio};
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Duration, Instant};
    use tempfile::TempDir;

    const FOREIGN_PATH: &str = "GROVE_TEST_FOREIGN_WITNESS_PATH";
    const WAIT: Duration = Duration::from_secs(10);

    #[test]
    fn witness_foreign_shared_holder_process() {
        let Some(path) = std::env::var_os(FOREIGN_PATH) else {
            return;
        };
        let file = File::open(path).unwrap();
        assert_eq!(
            unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
            0
        );
        println!("locked");
        std::io::stdout().flush().unwrap();
        let mut release = [0];
        std::io::stdin().read_exact(&mut release).unwrap();
        assert_eq!(release, [b'x']);
        drop(file);
    }

    struct ForeignHolder(Child);

    impl ForeignHolder {
        fn start(path: &Path) -> Self {
            let mut holder = Self(
                Command::new(std::env::current_exe().unwrap())
                    .args([
                        "--exact",
                        "driver_lease::witnesses::tests::witness_foreign_shared_holder_process",
                        "--nocapture",
                    ])
                    .env(FOREIGN_PATH, path)
                    .env_remove("GROVE_SIGNAL_FILE")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap(),
            );
            let stdout = holder.0.stdout.take().unwrap();
            let (ready_tx, ready_rx) = mpsc::channel();
            thread::spawn(move || {
                for line in BufReader::new(stdout).lines() {
                    if line.unwrap() == "locked" {
                        ready_tx.send(()).unwrap();
                    }
                }
            });
            ready_rx
                .recv_timeout(WAIT)
                .expect("foreign shared lock readiness");
            holder
        }

        fn release_and_reap(&mut self) {
            self.0.stdin.take().unwrap().write_all(b"x").unwrap();
            let deadline = Instant::now() + WAIT;
            loop {
                if let Some(status) = self.0.try_wait().unwrap() {
                    assert!(status.success(), "foreign holder failed: {status}");
                    return;
                }
                assert!(Instant::now() < deadline, "foreign holder did not exit");
                thread::yield_now();
            }
        }
    }

    impl Drop for ForeignHolder {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
````
<!-- /fragment -->

### Preparation failure preserves launch authority

The directory scenario acquires the foreign lock before preparation. The private
scenario pauses at the existing after-create hook and asks the parent to start
the holder on that exact path. An independent shared probe succeeds while an
exclusive probe fails. The preparation result must arrive before holder release;
the ten-second timeout detects blocking and supplies no ordering evidence.

Both scenarios use real epoch activation and `/bin/sh` supervision. Started and
Reaped callbacks verify admission, public Unavailable, and the still-held foreign
lock. The private case also checks that the directory witness excludes shared
probes during preparation while the containing-directory mutation lock remains
usable. Failure rolls back the directory lock; after holder reap, exclusive
probes succeed, invalidation removes the abandoned private file, and a fresh
preparation succeeds. The internal preparation callback defaults to the normal
`LaunchWitnesses::prepare`; tests replace only its synchronization hook.

<!-- fragment «witness-foreign-launch» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/witnesses.rs" lines="261-441" parent="witness-tests" -->
````rust

    #[test]
    fn witness_foreign_directory_holder_preserves_launch_and_admission() {
        foreign_holder_preserves_launch(false);
    }

    #[test]
    fn witness_foreign_private_holder_preserves_launch_and_admission() {
        foreign_holder_preserves_launch(true);
    }

    fn foreign_holder_preserves_launch(private: bool) {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        use super::super::{
            acquire_epoch_file, admit_session, tests::witness_selection, DriverLease, LockMode,
        };
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        fs::write(temp.path().join(".grove/_BRIEF.md"), "root").unwrap();
        let workspace = jj_workspace::Workspace::resolve(temp.path()).unwrap();
        let mut lease = DriverLease::acquire(&workspace).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        let channel = keyed_launch::Channel::allocate(lease.control_dir()).unwrap();
        let signal = channel.path().to_path_buf();
        let directory = temp.path().join(".grove");
        let mut holder = (!private).then(|| ForeignHolder::start(&directory));
        let (path_tx, path_rx) = mpsc::channel();
        let (ready_tx, ready_rx) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let worker = thread::spawn(move || {
            let mut preparation_error = None;
            let result = lease.prepare_launch_using(
                root,
                &signal,
                |path| acquire_epoch_file(path, LockMode::Exclusive, "foreign holder test"),
                |launch, control| {
                    let result = launch.prepare_with(
                        control,
                        random_nonce,
                        |path, _| {
                            assert!(private, "directory contention must prevent allocation");
                            path_tx.send(path.to_path_buf()).unwrap();
                            ready_rx.recv_timeout(WAIT).unwrap();
                            Ok(())
                        },
                        lock,
                    );
                    preparation_error = result.as_ref().err().map(|error| format!("{error:#}"));
                    result
                },
                |launch, epoch| launch.publish(epoch, &witness_selection()),
            );
            done_tx.send((lease, result, preparation_error)).unwrap();
        });
        let target = if private {
            let path = path_rx
                .recv_timeout(WAIT)
                .expect("private witness allocation");
            holder = Some(ForeignHolder::start(&path));
            path
        } else {
            directory.clone()
        };
        let probe = File::open(&target).unwrap();
        assert!(
            shared(&probe),
            "foreign shared locks permit shared observation"
        );
        // Keep the independent probe nonblocking even if production locking regresses.
        let assert_foreign_lock = || {
            assert_eq!(
                unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
                -1
            );
            assert!(matches!(std::io::Error::last_os_error().raw_os_error(),
                Some(code) if code == libc::EAGAIN || code == libc::EWOULDBLOCK));
        };
        assert_foreign_lock();
        if private {
            // Preparation already holds the directory, without blocking mutations
            // guarded by its containing directory.
            assert!(!shared(&File::open(&directory).unwrap()));
            lock(&File::open(temp.path()).unwrap()).unwrap();
            ready_tx.send(()).unwrap();
        }
        let (mut lease, result, error) = done_rx
            .recv_timeout(WAIT)
            .expect("preparation must finish before the foreign holder releases");
        worker.join().unwrap();
        result.unwrap();
        let error = error.expect("foreign holder must defeat preparation");
        assert!(
            error.contains(if private {
                "locking private witness"
            } else {
                "locking task-root witness"
            }),
            "{error}"
        );
        assert!(lease.launch.as_ref().unwrap().private.is_none());
        assert!(lease.launch.as_ref().unwrap().path().is_none());
        assert!(
            shared(&File::open(&directory).unwrap()),
            "partial directory lock rolled back"
        );
        lock(&File::open(temp.path()).unwrap()).unwrap();

        let config = temp.path().join("launch.kdl");
        fs::write(&config, "config { command \"run\" \"/bin/sh -c 'echo launched > proof'\"; bind \"run\" \"run\"; route \"test\" \"run\"; }\n").unwrap();
        let templates =
            keyed_launch::Templates::load(&config, None, keyed_launch::Vocabulary { slots: &[] })
                .unwrap();
        let argv = templates.expand("test", &[]).unwrap();
        let mut events = Vec::new();
        lease
            .supervise_launch(|observer| {
                keyed_launch::run_observed(
                    keyed_launch::Launch {
                        argv: &argv,
                        channel: &channel,
                        channel_var: "GROVE_SIGNAL_FILE",
                        scrub: &[],
                        cwd: Some(temp.path()),
                        escalation: keyed_launch::Escalation {
                            grace: Duration::ZERO,
                            kill_grace: Duration::ZERO,
                        },
                    },
                    &mut |event| {
                        observer(event);
                        events.push(event);
                        assert!(admit_session(
                            temp.path(),
                            "test",
                            Some(channel.path().to_path_buf())
                        )
                        .is_ok());
                        assert!(matches!(
                            crate::try_observe(temp.path(), &[None]).activity,
                            crate::ActivityObservation::Unavailable(_)
                        ));
                        assert!(shared(&probe));
                        assert_foreign_lock();
                    },
                )
            })
            .unwrap();
        assert_eq!(
            events,
            [
                keyed_launch::LaunchEvent::Started,
                keyed_launch::LaunchEvent::Reaped
            ]
        );
        assert_eq!(fs::read(temp.path().join("proof")).unwrap(), b"launched\n");
        assert!(lease.launch.is_none());
        holder.as_mut().unwrap().release_and_reap();
        lock(&probe).unwrap();
        drop(probe);
        lock(&File::open(&directory).unwrap()).unwrap();
        lease.invalidate_session_epoch().unwrap();
        if private {
            assert!(
                !target.exists(),
                "invalidation cleans the abandoned private file"
            );
        }
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        lease
            .prepare_launch(root, &witness_selection(), channel.path())
            .unwrap();
        assert!(
            lease.launch.as_ref().unwrap().private.is_some(),
            "preparation recovers after reap"
        );
        drop(lease);
        channel.discard().unwrap();
    }
````
<!-- /fragment -->

### Local rollback and marker controls

These remaining controls isolate marker writes, descriptor ownership, collision
retries, injected lock errors and cleanup grammar. They complement the foreign
process scenarios by naming each local failure boundary and checking which
locks remain held. The real-launch marker failure keeps both locks until Reaped.

<!-- fragment «witness-local-controls» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/witnesses.rs" lines="442-722" parent="witness-tests" -->
````rust

    #[test]
    fn witnessed_started_is_exact_and_never_retried_after_success_or_failure() {
        let (_temp, mut owner, control) = fixture();
        owner.prepare(&control).unwrap();
        owner.started().unwrap();
        owner.started().unwrap();
        assert_eq!(fs::read(owner.path().unwrap()).unwrap(), b"started\n");

        let (_temp, mut owner, control) = fixture();
        owner.prepare(&control).unwrap();
        let path = owner.path().unwrap().to_path_buf();
        // Read-only descriptor injects a real write failure without unsafe fd reuse.
        owner.private = Some(File::open(&path).unwrap());
        assert!(owner.started().is_err());
        owner.private = Some(OpenOptions::new().write(true).open(&path).unwrap());
        owner.started().unwrap();
        assert!(
            fs::read(path).unwrap().is_empty(),
            "failed publication was retried"
        );
    }

    #[test]
    fn witnessed_marker_failure_preserves_real_launch_and_both_locks_until_reap() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        use super::super::{tests::witness_selection, DriverLease, EPOCH_FILE_NAME};
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".jj")).unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let workspace = jj_workspace::Workspace::resolve(temp.path()).unwrap();
        let mut lease = DriverLease::acquire(&workspace).unwrap();
        let channel = keyed_launch::Channel::allocate(lease.control_dir()).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        lease
            .prepare_launch(root, &witness_selection(), channel.path())
            .unwrap();
        let owner = lease.launch.as_mut().unwrap();
        let path = owner.path().unwrap().to_path_buf();
        owner.private = Some(File::open(&path).unwrap());
        lock(owner.private.as_ref().unwrap()).unwrap();
        let private = File::open(&path).unwrap();
        let directory = File::open(temp.path().join(".grove")).unwrap();
        let epoch_path = lease.control_dir().join(EPOCH_FILE_NAME);
        let config = temp.path().join("launch.kdl");
        fs::write(&config, "config { command \"run\" \"/bin/sh -c true\"; bind \"run\" \"run\"; route \"test\" \"run\"; }\n").unwrap();
        let templates =
            keyed_launch::Templates::load(&config, None, keyed_launch::Vocabulary { slots: &[] })
                .unwrap();
        let argv = templates.expand("test", &[]).unwrap();
        let mut events = Vec::new();
        let result = lease.supervise_launch(|observer| {
            keyed_launch::run_observed(
                keyed_launch::Launch {
                    argv: &argv,
                    channel: &channel,
                    channel_var: "GROVE_SIGNAL_FILE",
                    scrub: &[],
                    cwd: Some(temp.path()),
                    escalation: keyed_launch::Escalation {
                        grace: std::time::Duration::ZERO,
                        kill_grace: std::time::Duration::ZERO,
                    },
                },
                &mut |event| {
                    observer(event);
                    events.push(event);
                    assert!(fs::read(&path).unwrap().is_empty());
                    let reaped = event == keyed_launch::LaunchEvent::Reaped;
                    assert_eq!(shared(&private), reaped);
                    assert_eq!(shared(&directory), reaped);
                    // Callbacks must have neither epoch nor containing-tree guard.
                    lock(&File::open(&epoch_path).unwrap()).unwrap();
                    lock(&File::open(temp.path()).unwrap()).unwrap();
                    assert!(super::super::admit_session(
                        temp.path(),
                        "test",
                        Some(channel.path().to_path_buf())
                    )
                    .is_ok());
                },
            )
        });
        assert!(result.is_ok(), "{result:?}");
        assert_eq!(
            events,
            [
                keyed_launch::LaunchEvent::Started,
                keyed_launch::LaunchEvent::Reaped
            ]
        );
        assert!(lease.launch.is_none());
    }

    fn fixture() -> (TempDir, LaunchWitnesses, PathBuf) {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join(".grove")).unwrap();
        let control = temp.path().join("control");
        fs::create_dir(&control).unwrap();
        let root = crate::TreeLifetime::open(temp.path()).unwrap().unwrap();
        (temp, LaunchWitnesses::new(root), control)
    }

    fn shared(file: &File) -> bool {
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) } == 0 {
            assert_eq!(unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) }, 0);
            true
        } else {
            assert!(matches!(std::io::Error::last_os_error().raw_os_error(),
                Some(code) if code == libc::EAGAIN || code == libc::EWOULDBLOCK));
            false
        }
    }

    #[test]
    fn paired_owner_closes_private_before_root_and_sets_exec_flags() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let (temp, mut owner, control) = fixture();
        owner.prepare(&control).unwrap();
        let directory = File::open(temp.path().join(".grove")).unwrap();
        let private = File::open(owner.path().unwrap()).unwrap();
        assert!(!shared(&directory));
        assert!(!shared(&private));
        for descriptor in [owner.root.directory(), owner.private.as_ref().unwrap()] {
            let flags = unsafe { libc::fcntl(descriptor.as_raw_fd(), libc::F_GETFD) };
            assert_ne!(flags, -1);
            assert_ne!(flags & libc::FD_CLOEXEC, 0);
        }
        assert_eq!(
            private.metadata().unwrap().permissions().mode() & 0o777,
            0o600
        );
        // This is the operation Drop invokes before Rust drops the root field.
        owner.close_private();
        assert!(shared(&private));
        assert!(!shared(&directory));
        drop(owner);
        assert!(shared(&directory));
    }

    #[test]
    fn paired_owner_retries_only_occupied_draws_and_never_truncates() {
        let (_temp, mut owner, control) = fixture();
        let occupied = control.join(format!("{PREFIX}{}", "00".repeat(16)));
        fs::write(&occupied, "occupied").unwrap();
        let mut draws = 0;
        owner
            .prepare_with(
                &control,
                || {
                    draws += 1;
                    Ok([u8::from(draws == 3); 16])
                },
                |_, _| Ok(()),
                lock,
            )
            .unwrap();
        assert_eq!(draws, 3);
        assert_eq!(fs::read(&occupied).unwrap(), b"occupied");
        assert!(fs::read(owner.path().unwrap()).unwrap().is_empty());
    }

    #[test]
    fn paired_owner_exhaustion_and_allocation_errors_release_directory() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        for failure in ["collision", "random", "open"] {
            let (temp, mut owner, control) = fixture();
            let occupied = control.join(format!("{PREFIX}{}", "00".repeat(16)));
            fs::write(&occupied, "preserve").unwrap();
            let target = if failure == "open" {
                occupied.clone()
            } else {
                control
            };
            let mut draws = 0;
            let result = owner.prepare_with(
                &target,
                || {
                    draws += 1;
                    if failure == "random" {
                        bail!("injected random source failure");
                    }
                    Ok([0; 16])
                },
                |_, _| Ok(()),
                lock,
            );
            assert!(result.is_err(), "{failure}");
            assert_eq!(draws, if failure == "collision" { 8 } else { 1 });
            assert!(owner.private.is_none());
            assert!(shared(&File::open(temp.path().join(".grove")).unwrap()));
            assert_eq!(fs::read(&occupied).unwrap(), b"preserve");
        }
    }

    #[test]
    fn paired_owner_foreign_holders_and_lock_errors_roll_back() {
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        for failure in [
            "directory-holder",
            "private-holder",
            "directory-error",
            "private-error",
        ] {
            let (temp, mut owner, control) = fixture();
            let directory = File::open(temp.path().join(".grove")).unwrap();
            if failure == "directory-holder" {
                assert_eq!(
                    unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
                    0
                );
            }
            let mut foreign = None;
            let mut locks = 0;
            let result = owner.prepare_with(
                &control,
                random_nonce,
                |path, _| {
                    if failure == "private-holder" {
                        let file = File::open(path)?;
                        assert_eq!(
                            unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) },
                            0
                        );
                        foreign = Some(file);
                    }
                    Ok(())
                },
                |file| {
                    locks += 1;
                    if (failure == "directory-error" && locks == 1)
                        || (failure == "private-error" && locks == 2)
                    {
                        bail!("injected reported lock failure");
                    }
                    lock(file)
                },
            );
            assert!(result.is_err(), "{failure}");
            assert!(owner.private.is_none());
            drop(foreign);
            assert!(shared(&directory));
            lock(&directory).unwrap();
            for entry in fs::read_dir(&control).unwrap() {
                assert!(shared(&File::open(entry.unwrap().path()).unwrap()));
            }
        }
    }

    #[test]
    fn paired_owner_cleanup_recognizes_only_its_names_and_preserves_retained() {
        let (_temp, mut owner, control) = fixture();
        owner.prepare(&control).unwrap();
        let retained = owner.path().unwrap().to_path_buf();
        let abandoned = control.join(format!("{PREFIX}{}", "ff".repeat(16)));
        fs::write(&abandoned, "old").unwrap();
        for name in [
            "witness-short",
            "witness-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "driver.lease",
        ] {
            fs::write(control.join(name), "untouched").unwrap();
        }
        discard_abandoned(&control, Some(&retained)).unwrap();
        assert!(retained.exists());
        assert!(!abandoned.exists());
        assert_eq!(fs::read_dir(&control).unwrap().count(), 4);
        drop(owner);
        discard_abandoned(&control, None).unwrap();
        assert!(!retained.exists());
        assert_eq!(fs::read_dir(&control).unwrap().count(), 3);
    }
}
````
<!-- /fragment -->




<a id="native-witness-processes"></a>
### Native process death and exec survivors

The exec child acknowledges startup over a Unix socket and echoes a later ping. That response proves the launched executable remains alive after the supervising holder is killed; leftover process identifiers alone cannot establish this.

<!-- fragment «runtime-native-exec» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1908-1927" parent="runtime-test-races" -->
````rust

    // These helpers only act when explicitly launched by the native scenarios.
    const NATIVE_WORK: &str = "GROVE_TEST_NATIVE_WITNESS_WORK";
    const NATIVE_WAIT: Duration = Duration::from_secs(15);

    #[test]
    fn witness_native_exec_child() {
        use std::os::unix::net::UnixStream;
        let Some(work) = std::env::var_os(NATIVE_WORK) else {
            return;
        };
        let mut stream = UnixStream::connect(Path::new(&work).join("child.sock")).unwrap();
        stream.set_read_timeout(Some(NATIVE_WAIT)).unwrap();
        stream.write_all(b"ready").unwrap();
        let mut byte = [0];
        while stream.read(&mut byte).unwrap() != 0 {
            stream.write_all(&byte).unwrap();
        }
    }

````
<!-- /fragment -->

The separate holder uses the real lease, preparation and runner callbacks. Started publishes the witness marker before the holder acknowledges readiness. The configured executable then waits for the observer to finish, so holder death and child exit can be controlled independently.

<!-- fragment «runtime-native-holder» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1928-1996" parent="runtime-test-races" -->
````rust
    #[test]
    fn witness_native_holder_process() {
        let Some(work) = std::env::var_os(NATIVE_WORK) else {
            return;
        };
        let work = Path::new(&work);
        let workspace = Workspace::resolve(work).unwrap();
        let mut lease = DriverLease::acquire(&workspace).unwrap();
        let channel = keyed_launch::Channel::allocate(&lease.control_dir).unwrap();
        lease
            .prepare_launch(
                TreeLifetime::open(work).unwrap().unwrap(),
                &super::super::tests::witness_selection(),
                channel.path(),
            )
            .unwrap();
        // The executable is a template slot, so spaces in host paths remain one argv.
        let config = work.join("launch.kdl");
        fs::write(
            &config,
            "config { command \"run\" \"/usr/bin/env ${exe} --exact driver_lease::observation::tests::witness_native_exec_child --nocapture\"; bind \"run\" \"run\"; route \"test\" \"run\"; }\n",
        )
        .unwrap();
        let templates = keyed_launch::Templates::load(
            &config,
            None,
            keyed_launch::Vocabulary {
                slots: &[keyed_launch::SlotRule {
                    name: "exe",
                    requirement: keyed_launch::Requirement::ExactlyOnce,
                }],
            },
        )
        .unwrap();
        let exe = std::env::current_exe().unwrap();
        let argv = templates
            .expand(
                "test",
                &[keyed_launch::Slot {
                    name: "exe",
                    value: exe.as_os_str(),
                }],
            )
            .unwrap();
        lease
            .supervise_launch(|notify| {
                keyed_launch::run_observed(
                    keyed_launch::Launch {
                        argv: &argv,
                        channel: &channel,
                        channel_var: "GROVE_SIGNAL_FILE",
                        scrub: &[],
                        cwd: Some(work),
                        escalation: keyed_launch::Escalation {
                            grace: Duration::ZERO,
                            kill_grace: Duration::ZERO,
                        },
                    },
                    &mut |event| {
                        notify(event);
                        if event == keyed_launch::LaunchEvent::Started {
                            println!("holder-ready");
                            std::io::stdout().flush().unwrap();
                        }
                    },
                )
            })
            .unwrap();
    }
````
<!-- /fragment -->

The scenario owns the holder as a child process. SIGKILL is followed by a bounded try_wait loop and a checked signal status. Drop also kills and reaps on assertion failure, so the test never treats signal delivery as completed teardown.

<!-- fragment «runtime-native-reap» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="1997-2021" parent="runtime-test-races" -->
````rust

    struct NativeHolder(std::process::Child);

    impl NativeHolder {
        fn kill_and_reap(&mut self) {
            use std::os::unix::process::ExitStatusExt;
            self.0.kill().unwrap();
            let deadline = std::time::Instant::now() + NATIVE_WAIT;
            loop {
                if let Some(status) = self.0.try_wait().unwrap() {
                    assert_eq!(status.signal(), Some(libc::SIGKILL));
                    return;
                }
                assert!(std::time::Instant::now() < deadline, "holder reap timeout");
                std::thread::yield_now();
            }
        }
    }

    impl Drop for NativeHolder {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
````
<!-- /fragment -->

The native scenarios open independent descriptors and prove both shared probes contend after readiness. After actual holder reap, both succeed while the exec child still answers and Started bytes remain; the production observer must report Idle. The replacement scenario first removes the root path, then creates a new tree with reused key k1 and unlinks the old root. A fresh observation must report PreviousTree. This tests real open-object binding, not forced numeric identity reuse. Socket and reap waits use failure deadlines; no elapsed sleep supplies evidence.

<!-- fragment «runtime-native-scenarios» owner="which-calls-are-admitted" source="crates/grove-loop/src/driver_lease/observation.rs" lines="2022-2172" parent="runtime-test-races" -->
````rust

    fn native_process_evidence(replace: bool) {
        use std::io::{BufRead, BufReader};
        use std::os::unix::net::UnixListener;
        use std::process::{Command, Stdio};
        if !super::super::tests::fork_sensitive_driver_lease_test_body_runs_here() {
            return;
        }
        let work = TempDir::new().unwrap();
        fs::create_dir(work.path().join(".jj")).unwrap();
        let root = work.path().join(".grove");
        fs::create_dir(&root).unwrap();
        fs::write(root.join("_BRIEF.md"), "original").unwrap();
        fs::write(root.join("01-impl--work-k1.md"), "original task").unwrap();
        let listener = UnixListener::bind(work.path().join("child.sock")).unwrap();
        listener.set_nonblocking(true).unwrap();
        let mut holder = NativeHolder(
            Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "driver_lease::observation::tests::witness_native_holder_process",
                    "--nocapture",
                ])
                .env(NATIVE_WORK, work.path())
                .env_remove("GROVE_SIGNAL_FILE")
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap(),
        );
        let stdout = holder.0.stdout.take().unwrap();
        let (ready_tx, ready_rx) = mpsc::channel();
        let reader = std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                if line.unwrap() == "holder-ready" {
                    let _ = ready_tx.send(());
                }
            }
        });
        ready_rx
            .recv_timeout(NATIVE_WAIT)
            .expect("Started readiness");
        let deadline = std::time::Instant::now() + NATIVE_WAIT;
        let mut child = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(
                        std::time::Instant::now() < deadline,
                        "exec readiness timeout"
                    );
                    std::thread::yield_now();
                }
                Err(error) => panic!("accepting exec child: {error}"),
            }
        };
        // Accepted sockets inherit nonblocking mode on macOS; use bounded
        // blocking protocol reads consistently on both native platforms.
        // https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/accept.2.html
        child.set_nonblocking(false).unwrap();
        child.set_read_timeout(Some(NATIVE_WAIT)).unwrap();
        child.set_write_timeout(Some(NATIVE_WAIT)).unwrap();
        let mut ready = [0; 5];
        child.read_exact(&mut ready).unwrap();
        assert_eq!(&ready, b"ready");
        let mandate = running(sample(work.path()));
        assert_eq!(mandate.handle.to_string(), "work-k1");
        assert_eq!(mandate.relation, TreeRelation::SameTree);
        let control = Workspace::discover_control_dir(work.path(), CONTROL_NAMESPACE)
            .unwrap()
            .unwrap();
        let witness = fs::read_dir(&control)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("witness-")
            })
            .unwrap();
        let directory_probe = File::open(&root).unwrap();
        let private_probe = File::open(&witness).unwrap();
        let shared = |file: &File| {
            let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) };
            if result == 0 {
                assert_eq!(unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) }, 0);
                true
            } else {
                let error = std::io::Error::last_os_error();
                assert!(
                    matches!(error.raw_os_error(), Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN),
                    "{error}"
                );
                false
            }
        };
        assert!(
            !shared(&directory_probe),
            "directory witness must be exclusive"
        );
        assert!(!shared(&private_probe), "private witness must be exclusive");
        assert_eq!(fs::read(&witness).unwrap(), b"started\n");
        if replace {
            let old = work.path().join("old-root");
            fs::rename(&root, &old).unwrap();
            assert_eq!(
                running(crate::try_observe(work.path(), &[]).activity).relation,
                TreeRelation::NoReadableTree
            );
            fs::create_dir(&root).unwrap();
            fs::write(root.join("_BRIEF.md"), "replacement").unwrap();
            fs::write(root.join("01-impl--replacement-k1.md"), "reused key").unwrap();
            fs::remove_dir_all(&old).unwrap();
            let fresh = running(crate::try_observe(work.path(), &[]).activity);
            assert_eq!(fresh.handle.to_string(), "work-k1");
            assert_eq!(fresh.relation, TreeRelation::PreviousTree);
            assert!(shared(&File::open(&root).unwrap()));
            assert!(!shared(&directory_probe));
            assert!(!shared(&private_probe));
        }
        holder.kill_and_reap();
        // The exec'd child is alive after confirmed holder death. If either
        // descriptor leaked through exec, these independent probes would fail.
        child.write_all(b"p").unwrap();
        let mut pong = [0];
        child.read_exact(&mut pong).unwrap();
        assert_eq!(&pong, b"p");
        assert!(shared(&directory_probe));
        assert!(shared(&private_probe));
        assert_eq!(fs::read(&witness).unwrap(), b"started\n");
        assert_eq!(sample(work.path()), ActivityObservation::Idle);
        child.shutdown(std::net::Shutdown::Write).unwrap();
        assert_eq!(
            child.read(&mut pong).unwrap(),
            0,
            "child exited its protocol"
        );
        reader.join().unwrap();
    }

    #[test]
    fn witness_native_killed_holder_releases_both_locks_with_exec_survivor() {
        native_process_evidence(false);
    }

    #[test]
    fn witness_native_replaced_root_reports_previous_tree_to_new_observer() {
        native_process_evidence(true);
    }
}
````
<!-- /fragment -->

[Previous: One live driver per working tree](16-the-lease.md) | [Contents](README.md) | [Next: Which files take part](18-which-files.md)
