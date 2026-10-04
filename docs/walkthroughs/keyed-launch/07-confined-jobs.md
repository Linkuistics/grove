# Confined noninteractive jobs
<!-- book-page id="confined-jobs" slice="confined-jobs" order="7" -->
[Previous: What ends a launch](06-what-ends-a-launch.md) | [Contents](README.md)

<a id="confined-job"></a>
## A separate process mode and filesystem policy

`run_noninteractive` accepts a `NoninteractiveLaunch` without a completion
channel, captures stdout and stderr in a caller-owned regular file, creates a
new POSIX session and keeps stdin at EOF. Grove uses it to supervise dispatch.
It adds no filesystem confinement. The interactive `run` and `run_observed`
path retains its foreground-job behavior.

`run_confined_observed` uses the detached process mode with inherited stdout
and stderr. `FilesystemGrants` names writable directories and literal runtime
files. It clears the environment before applying explicit grants and setting
the channel last. Dispatch uses it to confine cwd, scratch and exit storage.
Both detached forms close inherited descriptors above stderr on exec and
report cancellation only after group cleanup.

<a id="worked-confined-job"></a>
## Worked example: run a confined job and read its result

A caller creates `/tmp/invocation/work`, `/tmp/invocation/tmp` and
`/tmp/invocation/control`, opens the work directory and stages `input.txt` containing `alpha` plus a
newline. Suppose channel allocation chooses
`/tmp/invocation/control/signal-0123456789abcdef0123456789abcdef`.
`Launch.cwd` is `/tmp/invocation/work`, `channel_var` is `TASK_DONE`, and the
explicit grants contain only what the confined harness needs.
The escalation uses a two-second completion grace and five-second kill grace.
`FilesystemGrants.writable` contains `/tmp/invocation`; its runtime-read list contains the
existing file `/home/reader/.config/harness/token`.

The caller's `Argv` holds the program `/bin/sh` and two arguments: `-c`
and the following command body. It copies the staged input and acknowledges
completion through the exact channel granted by this launch.

```sh
cat input.txt > result.md
: > "$TASK_DONE"
```

The caller passes that `Launch`, the confinement grants and a spawn/reap observer to
`run_confined_observed`. On macOS the runner invokes `sandbox-exec` with a default-deny
profile; on Linux it invokes bubblewrap with a restricted filesystem view.
Neither path retries through ordinary execution if confinement fails. The child
can read `input.txt` and write `result.md`; the parent's unrelated project is
outside the granted filesystem set.

Assume the shell writes `done` plus a newline to the channel and exits with
status 0 before the completion grace expires. The supervisor observes leader
exit with `waitid` and `WNOWAIT`, retaining the leader's identity while killing
remaining members of its process group twice. It reaps the leader, then polls
for the group's disappearance using signal zero only. A reused PID therefore
cannot redirect a later destructive cleanup signal at an unrelated process
group. This is the same end every launch has, interactive or not, and chapter 4
reads it. The returned `Ended` has `end: End::Exited`, a successful exit status,
`signalled: true` and `group: Group::Gone`.

The caller then opens `result.md` with `regular_file_at` through the held work
directory and reads `alpha` plus a newline, even if the child renamed that
directory's pathname. A symlink, FIFO or non-regular result is refused. This is
the example's observable end: a supervised outcome and bytes available through
a stable file handle. Deciding whether to export those bytes remains the
caller's responsibility. Cancellation kills the confined job immediately and
returns an interrupted outcome, while a noninteractive job is forwarded the
signal and given the kill-grace first. A group that survives is reported as
`Group::Present`, which `grove run` treats as a reason to publish nothing.
Processes that deliberately leave their process group are outside this cleanup
guarantee.

The following comparison states which boundary each entry point supplies.

| Entry point | Streams and process control | Filesystem policy |
|---|---|---|
| `run`, `run_observed` | Interactive process group and terminal handover | Caller environment |
| `run_noninteractive` | New session, EOF stdin, captured output | No added filesystem confinement |
| `run_confined_observed` | New session, EOF stdin, inherited output, explicit environment, spawn/reap callbacks | Required native backend and multiple writable grants |

The policy implementation below supplies command construction and stable artifact
reads; the job and watch chapters own spawning, cancellation and reaping.

<!-- fragment «confinement-policy» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="1-326" parent="source-confinement" -->
<!-- insert «held-directory-read» -->
<!-- insert «confinement-contract» -->
<!-- insert «confinement-resource-resolution» -->
<!-- insert «confinement-macos» -->
<!-- insert «confinement-linux» -->
<!-- insert «confinement-unavailable» -->
<!-- /fragment -->

<a id="held-directory-read"></a>
## Read through a held directory

`regular_file_at` accepts one basename and an already-open directory. It uses
`openat` with no-follow, nonblocking and close-on-exec flags, then verifies the
result is a regular file. The held descriptor fixes the parent directory's
identity; the final-component check prevents a substituted symlink or FIFO from
redirecting or blocking the example's result read.

<!-- fragment «held-directory-read» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="1-42" parent="confinement-policy" -->
````rust
//! Mandatory outer filesystem policy for standalone invocations.
use std::ffi::{CString, OsStr};
use std::fs::File;
use std::os::fd::{AsRawFd, FromRawFd};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{Argv, LaunchError};

/// Open one regular file in a directory held before launching untrusted work.
/// Neither a replaced directory path nor a symlink can redirect this read.
pub fn regular_file_at(directory: &File, name: &OsStr) -> std::io::Result<File> {
    let mut components = Path::new(name).components();
    if !matches!(components.next(), Some(std::path::Component::Normal(_)))
        || components.next().is_some()
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "expected one artifact basename",
        ));
    }
    let name = CString::new(name.as_encoded_bytes())?;
    // SAFETY: directory is held open, name is NUL-terminated, and the returned
    // fresh descriptor is immediately owned. NONBLOCK prevents FIFO hangs.
    let fd = unsafe {
        libc::openat(
            directory.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    if !file.metadata()?.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "artifact is not a regular file",
        ));
    }
    Ok(file)
````
<!-- /fragment -->

<a id="confinement-contract"></a>
## Construct a mandatory boundary

`FilesystemGrants` names multiple writable directories and explicit runtime files.
`command_with_grants` canonicalizes every writable directory, rejects the
filesystem root, resolves the executable the argv names and validates runtime
reads as regular files. It gives the native backend those paths and appends
the executable with its exact arguments. Dispatch supplies scratch-directory
environment values through the launch's explicit grants. Policy preparation or spawn failure returns `LaunchError`. If the
backend starts and then refuses its setup, supervision instead returns an
`Ended` carrying its unsuccessful status and usually no completion signal. The caller must
inspect that result before accepting outputs. Neither failure path retries
without confinement.

`FilesystemGrants` is the shared backend input. `confinement_available` runs
the required system launcher around `/usr/bin/true` with empty explicit grants,
capturing its diagnostics. It temporarily repairs an ignored SIGCHLD for the
wait and restores it afterwards; it leaves the caller's signal mask unchanged.
Dispatch uses this probe before selection, so a backend denied permission to
apply its sandbox refuses without evaluating policy or recording a run. The
probe cannot guarantee subsequent grant setup or exclude intervening resource
changes. `confinement_system_reads`
is the same inventory the backends grant: a caller protecting policy, settings
or state checks those implicit reads too. This runner knows no owner paths and
performs no policy selection. Dispatch checks canonical overlap before its
worker evaluates anything and refuses an owner resource beneath a system tree.

<!-- fragment «confinement-contract» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="43-179" parent="confinement-policy" -->
````rust
}

/// Multiple writable directories and literal read-only runtime files. The
/// caller decides which owner resources must remain outside these grants.
pub struct FilesystemGrants<'a> {
    pub writable: &'a [PathBuf],
    pub runtime_read: &'a [PathBuf],
}

/// System filesystem resources exposed by the native backend. A caller that
/// protects owner resources must check these implicit grants too.
#[cfg(target_os = "macos")]
pub fn confinement_system_reads() -> &'static [&'static str] {
    &[
        "/private/etc/ssl/cert.pem",
        "/private/preboot/Cryptexes/OS",
        "/System",
        "/usr/bin",
        "/usr/sbin",
        "/usr/lib",
        "/usr/libexec",
        "/usr/share",
        "/bin",
        "/sbin",
        "/Library/Apple",
        "/opt/homebrew/Cellar",
        "/usr/local/lib",
        "/private/var/db/timezone",
        "/dev/random",
        "/dev/urandom",
        "/private/etc/passwd",
        "/private/etc/group",
        "/private/etc/hosts",
        "/private/etc/resolv.conf",
        "/private/etc/services",
        "/private/etc/protocols",
        "/private/etc/localtime",
        "/dev/null",
        "/dev/zero",
    ]
}

#[cfg(target_os = "linux")]
pub fn confinement_system_reads() -> &'static [&'static str] {
    &[
        "/usr/bin",
        "/usr/sbin",
        "/usr/lib",
        "/usr/lib64",
        "/usr/libexec",
        "/usr/share",
        "/bin",
        "/sbin",
        "/lib",
        "/lib64",
        "/etc/ld.so.cache",
        "/etc/passwd",
        "/etc/group",
        "/etc/nsswitch.conf",
        "/etc/resolv.conf",
        "/etc/hosts",
        "/etc/ssl/certs",
        "/proc",
        "/dev",
    ]
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub fn confinement_system_reads() -> &'static [&'static str] {
    &[]
}

/// Establish an empty native sandbox around a system no-op before selection.
/// Presence alone does not prove that the backend can apply its policy.
pub fn confinement_available() -> Result<(), LaunchError> {
    let mut command = platform_command(&[], &[])?;
    command
        .arg("/usr/bin/true")
        .env_clear()
        .current_dir("/")
        .stdin(std::process::Stdio::null());
    // An inherited ignored SIGCHLD would auto-reap the probe. Repair it only
    // for this wait, then restore the caller's disposition, including on error.
    let ignored = crate::run::restore_child_watching();
    let output = command.output();
    if ignored {
        // SAFETY: restoring the disposition inspected before the probe.
        unsafe {
            libc::signal(libc::SIGCHLD, libc::SIG_IGN);
        }
    }
    let output =
        output.map_err(|error| LaunchError::new(format!("cannot probe confinement: {error}")))?;
    if !output.status.success() {
        return Err(LaunchError::new(format!(
            "confinement backend could not establish the probe sandbox ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(())
}

pub(crate) fn command_with_grants(
    argv: &Argv,
    policy: &FilesystemGrants<'_>,
) -> Result<Command, LaunchError> {
    let roots: Vec<PathBuf> = policy
        .writable
        .iter()
        .map(|path| {
            let path = canonical(path)?;
            if !path.is_dir() || path.parent().is_none() {
                return Err(LaunchError::new(
                    "confinement requires writable directories other than /",
                ));
            }
            Ok(path)
        })
        .collect::<Result<_, _>>()?;
    let program = executable(argv.program())?;
    let mut reads = vec![program.clone()];
    for path in policy.runtime_read {
        let path = canonical(path)?;
        if !path.is_file() {
            return Err(LaunchError::new(format!(
                "runtime grant {} must name a regular file; stage task inputs instead of granting directories",
                path.display()
            )));
        }
        reads.push(path);
    }
    let mut command = platform_command(&roots, &reads)?;
    command.arg(program).args(argv.args());
    Ok(command)
}

````
<!-- /fragment -->

<a id="confinement-resource-resolution"></a>
## Resolve executable and resource paths

`canonical` turns missing or inaccessible resources into a diagnostic naming
the path. `executable` takes an absolute program path and refuses any other: it
looks no name up, on PATH or anywhere else, so the file the caller chose is the
file that is granted and run. A caller that holds a bare name resolves it first,
by whatever rule it trusts. The resolved executable becomes an explicit read
grant; the caller still owns its argument vector. For the example, the
credential and executable must resolve before any child is spawned.

<!-- fragment «confinement-resource-resolution» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="180-202" parent="confinement-policy" -->
````rust
fn canonical(path: &Path) -> Result<PathBuf, LaunchError> {
    path.canonicalize().map_err(|error| {
        LaunchError::new(format!(
            "cannot resolve sandbox resource {}: {error}; supply an existing resource",
            path.display()
        ))
    })
}

/// The file a confined launch grants and runs: the caller's absolute path. A
/// name is refused, never looked up, so the file the caller chose is the file
/// that runs.
fn executable(program: &OsStr) -> Result<PathBuf, LaunchError> {
    let path = Path::new(program);
    if !path.is_absolute() {
        return Err(LaunchError::new(format!(
            "a confined launch takes an absolute program path, and {program:?} is not one; \
             resolve the program to a file before launching it"
        )));
    }
    canonical(path)
}

````
<!-- /fragment -->

<a id="confinement-macos"></a>
## Apply the macOS policy

The macOS backend builds a default-deny Seatbelt policy and supplies paths as
parameters rather than interpolating them into policy syntax. A denied operation
returns `ENOENT`, so optional configuration probes see unavailable content as
absent while its bytes remain unreadable. The policy admits process execution,
installed runtime resources and read-only literal grants for the executable and
runtime files. Invocation contents are writable; file metadata remains more
widely visible than file content.

The allowed system calls and services support a native harness without granting
its unrelated user files. Explicit DNS socket and Mach-service access, an
`AF_SYSTEM` protocol-2 socket, and selected host and routing sysctls accompany
outbound IP networking. The public CA certificate and trust services support
TLS. These grants let a harness contact its service while preserving the
example's filesystem boundary; network destinations are not restricted.

<!-- fragment «confinement-macos» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="203-278" parent="confinement-policy" -->
````rust
#[cfg(target_os = "macos")]
fn platform_command(roots: &[PathBuf], reads: &[PathBuf]) -> Result<Command, LaunchError> {
    // Seatbelt is inherited by descendants. Parameters keep path text out of
    // the policy language, including quotes and newlines in native filenames.
    // Reference: OpenAI's upstream seatbelt_base_policy.sbpl (Apache-2.0),
    // https://github.com/openai/codex/tree/main/codex-rs/sandboxing/src .
    // Hidden resources look absent, as in Linux's empty namespace. Harnesses
    // can probe optional configuration without reading it or failing startup.
    let mut profile = String::from(
        r#"(version 1)
(deny default (with errno ENOENT))
(allow process-exec process-fork)
(allow signal (target same-sandbox))
(allow process-info* (target same-sandbox))
(allow system-mac-syscall (mac-policy-name "vnguard"))
(allow system-mac-syscall (require-all (mac-policy-name "Sandbox") (mac-syscall-number 67)))
(allow sysctl-read (sysctl-name-prefix "hw.") (sysctl-name "kern.osrelease") (sysctl-name "kern.ostype") (sysctl-name "kern.osversion") (sysctl-name "kern.osproductversion") (sysctl-name "kern.argmax"))
(allow mach-lookup (global-name "com.apple.system.opendirectoryd.libinfo") (global-name "com.apple.trustd") (global-name "com.apple.trustd.agent") (global-name "com.apple.system.logger"))
(allow network-outbound (remote ip "*:*"))
(allow network-outbound (literal "/private/var/run/mDNSResponder"))
(allow mach-lookup (global-name "com.apple.SystemConfiguration.DNSConfiguration") (global-name "com.apple.SystemConfiguration.configd") (global-name "com.apple.networkd"))
(allow system-socket (require-all (socket-domain AF_SYSTEM) (socket-protocol 2)))
(allow sysctl-read (sysctl-name "kern.hostname") (sysctl-name "kern.version") (sysctl-name-prefix "net.routetable."))
(allow file-read-metadata)
(allow file-read* (literal "/"))
(allow file-read* file-write-data (literal "/dev/null") (literal "/dev/zero"))
"#,
    );
    let backend = "/usr/bin/sandbox-exec";
    if !Path::new(backend).is_file() {
        return Err(LaunchError::new("filesystem confinement requires /usr/bin/sandbox-exec; restore the system Seatbelt launcher"));
    }
    let mut command = Command::new(backend);
    for path in confinement_system_reads() {
        profile.push_str(&format!(
            "(allow file-read* file-map-executable (subpath \"{path}\"))\n"
        ));
    }
    for (index, root) in roots.iter().enumerate() {
        let name = format!("WRITABLE_{index}");
        profile.push_str(&format!(
            "(allow file-read* file-write* file-map-executable (subpath (param \"{name}\")))\n"
        ));
        command.arg("-D").arg(parameter(&name, root));
    }
    // Explicit denies preserve read-only files beneath broader writable roots.
    // Ancestor unlink denies prevent moving a directory to bypass the literal.
    // https://github.com/openai/codex/blob/main/codex-rs/sandboxing/src/seatbelt.rs
    for (index, path) in reads.iter().enumerate() {
        let name = format!("RUNTIME_{index}");
        profile.push_str(&format!(
            "(allow file-read* file-map-executable (literal (param \"{name}\")))\n"
        ));
        profile.push_str(&format!(
            "(deny file-write* (literal (param \"{name}\")))\n"
        ));
        command.arg("-D").arg(parameter(&name, path));
        for (ancestor_index, ancestor) in path.ancestors().skip(1).enumerate() {
            let anchor = format!("{name}_ANCESTOR_{ancestor_index}");
            profile.push_str(&format!(
                "(deny file-write-unlink (literal (param \"{anchor}\")))\n"
            ));
            command.arg("-D").arg(parameter(&anchor, ancestor));
        }
    }
    command.args(["-p", &profile, "--"]);
    Ok(command)
}

#[cfg(target_os = "macos")]
fn parameter(name: &str, path: &Path) -> std::ffi::OsString {
    let mut value = std::ffi::OsString::from(format!("{name}="));
    value.push(path);
    value
}

````
<!-- /fragment -->

<a id="confinement-linux"></a>
## Build the Linux filesystem view

The Linux backend requires a system bubblewrap executable. It starts from a
restricted mount view, binds installed runtime paths read-only, creates proc and
dev views, and binds only the invocation root read-write. The explicit runtime
files are read-only binds. Final read-only remounts freeze the namespace's
root tmpfs, `/dev` and `/proc`, so automatically created parent directories and
those synthetic mounts do not provide extra places to create files. The
invocation bind remains writable. User, PID, IPC and UTS namespaces and
die-with-parent behavior accompany the filesystem view;
the outer runner already creates the POSIX session used by supervision.

<!-- fragment «confinement-linux» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="279-320" parent="confinement-policy" -->
````rust
#[cfg(target_os = "linux")]
fn platform_command(roots: &[PathBuf], reads: &[PathBuf]) -> Result<Command, LaunchError> {
    // Start with an empty mount namespace, never a read-only bind of `/`.
    // https://github.com/containers/bubblewrap/blob/main/README.md
    let backend = ["/usr/bin/bwrap", "/bin/bwrap"].into_iter().find(|path| Path::new(path).is_file())
        .ok_or_else(|| LaunchError::new("filesystem confinement requires bubblewrap at /usr/bin/bwrap; install the system bubblewrap package"))?;
    let mut command = Command::new(backend);
    // The confined runner already creates the session. Keeping the payload in the
    // monitor's group lets its cleanup wait observe every ordinary descendant.
    command.args([
        "--die-with-parent",
        "--unshare-user",
        "--unshare-pid",
        "--unshare-ipc",
        "--unshare-uts",
    ]);
    for &path in confinement_system_reads() {
        if !matches!(path, "/proc" | "/dev") && Path::new(path).exists() {
            command.args(["--ro-bind", path, path]);
        }
    }
    command.args(["--proc", "/proc", "--dev", "/dev"]);
    for root in roots {
        command.arg("--bind").arg(root).arg(root);
    }
    for path in reads {
        command.arg("--ro-bind").arg(path).arg(path);
    }
    // Auto-created parent directories belong to the namespace's tmpfs root.
    // Freeze that root too, leaving only the invocation bind writable.
    command.args([
        "--remount-ro",
        "/",
        "--remount-ro",
        "/dev",
        "--remount-ro",
        "/proc",
    ]);
    command.arg("--");
    Ok(command)
}

````
<!-- /fragment -->

<a id="confinement-unavailable"></a>
## Refuse an unsupported platform

The fallback implementation returns a launch error on platforms with neither
native backend. This preserves the meaning of `run_confined_observed`: the caller cannot
mistake an unsupported environment for the confined result required before
reading and exporting the example's output.

<!-- fragment «confinement-unavailable» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="321-326" parent="confinement-policy" -->
````rust
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn platform_command(_: &[PathBuf], _: &[PathBuf]) -> Result<Command, LaunchError> {
    Err(LaunchError::new(
        "standalone confinement is unavailable on this platform; the task was not launched",
    ))
}
````
<!-- /fragment -->

[Previous: What ends a launch](06-what-ends-a-launch.md) | [Contents](README.md)
