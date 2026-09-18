# Confined noninteractive jobs
<!-- book-page id="confined-jobs" slice="confined-jobs" order="11" -->
[Previous: What passes through](10-what-passes-through.md) | [Contents](README.md)

<a id="confined-job"></a>
## A separate process mode and filesystem policy

`run_confined` combines the noninteractive process mode with mandatory filesystem
confinement. `run_noninteractive` supplies only the process mode. Both keep stdin
at EOF, direct stdout and stderr to a caller-owned regular file, create a new
POSIX session, and mark inherited descriptors above stderr close-on-exec. They
do not hand over a controlling terminal. The interactive `run` and `run_observed`
path described earlier retains its foreground-job behavior.

<a id="worked-confined-job"></a>
## Worked example: run a confined job and read its result

A caller creates `/tmp/invocation/work`, `/tmp/invocation/tmp` and
`/tmp/invocation/control`, opens the work directory and a regular
`/tmp/invocation/transcript.log`, and stages `input.txt` containing `alpha` plus a
newline. Suppose channel allocation chooses
`/tmp/invocation/control/signal-0123456789abcdef0123456789abcdef`.
`Launch.cwd` is `/tmp/invocation/work`, `channel_var` is `TASK_DONE`, and the
scrub list removes inherited `GROVE_SIGNAL_FILE` and `GROVE_RUN_SIGNAL_FILE`.
The escalation uses a two-second completion grace and five-second kill grace.
`Confinement.writable` is `/tmp/invocation`; its runtime-read list contains the
existing file `/home/reader/.config/harness/token`.

The configured template expands to program `/bin/sh` and two arguments: `-c`
and the following command body. It copies the staged input and acknowledges
completion through the exact channel granted by this launch.

```sh
cat input.txt > result.md
printf 'done\n' > "$TASK_DONE"
```

The caller passes that `Launch`, the log and the confinement policy to
`run_confined`. On macOS the runner invokes `sandbox-exec` with a default-deny
profile; on Linux it invokes bubblewrap with a restricted filesystem view.
Neither path retries through ordinary execution if confinement fails. The child
can read `input.txt` and write `result.md`; the parent's unrelated project is
outside the granted filesystem set.

Assume the shell writes `done` plus a newline to the channel and exits with
status 0 before the completion grace expires. The supervisor observes leader
exit with `waitid` and `WNOWAIT`, retaining the leader's identity while killing
remaining members of its process group. It reaps the leader, then polls for the
group's disappearance using signal zero only. A reused PID therefore cannot
redirect a later destructive cleanup signal at an unrelated process group.
The returned `Ended` has `end: End::Exited`, a successful exit status and a
present token whose `as_str()` is `"done"`.

The caller then opens `result.md` with `regular_file_at` through the held work
directory and reads `alpha` plus a newline, even if the child renamed that
directory's pathname. A symlink, FIFO or non-regular result is refused. This is
the example's observable end: a supervised outcome and bytes available through
a stable file handle. Deciding whether to export those bytes remains the
caller's responsibility. Cancellation kills the detached job immediately and
returns an interrupted outcome; a cleanup failure returns an error. Processes
that deliberately leave their process group are outside this cleanup guarantee.

The following comparison states which boundary each entry point supplies.

| Entry point | Streams and process control | Filesystem policy |
|---|---|---|
| `run`, `run_observed` | Interactive process group and terminal handover | Caller environment |
| `run_noninteractive` | New session, EOF stdin, captured output | No added filesystem confinement |
| `run_confined` | Same noninteractive mode | Required native backend and explicit runtime grants |

The policy implementation below supplies command construction and stable artifact
reads; the job and watch chapters own spawning, cancellation and reaping.

<!-- fragment «confinement-policy» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="1-219" parent="source-confinement" -->
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

<!-- fragment «held-directory-read» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="1-44" parent="confinement-policy" -->
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
}

````
<!-- /fragment -->

<a id="confinement-contract"></a>
## Construct a mandatory boundary

`Confinement` names one writable invocation directory and explicit runtime files.
`command` canonicalizes that root, rejects a filesystem root as the invocation,
resolves the configured executable and validates each runtime grant as a file.
It gives the native backend those resolved paths, redirects temporary-directory
variables into scratch state, and appends the executable with its exact expanded
arguments. Policy preparation or spawn failure returns `LaunchError`. If the
backend starts and then refuses its setup, supervision instead returns an
`Ended` carrying its unsuccessful status and usually no token. The caller must
inspect that result before accepting outputs. Neither failure path retries
without confinement.

<!-- fragment «confinement-contract» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="45-78" parent="confinement-policy" -->
````rust
/// Writable invocation root and explicitly granted, read-only runtime files.
/// User data belongs in staged inputs. Runtime grants never confer writes.
pub struct Confinement<'a> {
    pub writable: &'a Path,
    pub runtime_read: &'a [PathBuf],
}

pub(crate) fn command(argv: &Argv, policy: &Confinement<'_>) -> Result<Command, LaunchError> {
    let root = canonical(policy.writable)?;
    if !root.is_dir() || root.parent().is_none() {
        return Err(LaunchError::new(
            "confinement requires a private invocation directory",
        ));
    }
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
    let mut command = platform_command(&root, &reads)?;
    for name in ["TMPDIR", "TMP", "TEMP"] {
        command.env(name, root.join("tmp"));
    }
    command.arg(program).args(argv.args());
    Ok(command)
}

````
<!-- /fragment -->

<a id="confinement-resource-resolution"></a>
## Resolve executable and resource paths

`canonical` turns missing or inaccessible resources into a diagnostic naming
the path. `executable` resolves a path-bearing program directly or searches PATH
for a bare program name. The selected executable becomes an explicit read grant;
configuration still owns its argument vector. For the example, the credential
and executable must resolve before any child is spawned.

<!-- fragment «confinement-resource-resolution» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="79-103" parent="confinement-policy" -->
````rust
fn canonical(path: &Path) -> Result<PathBuf, LaunchError> {
    path.canonicalize().map_err(|error| {
        LaunchError::new(format!(
            "cannot resolve sandbox resource {}: {error}; supply an existing resource",
            path.display()
        ))
    })
}

fn executable(program: &std::ffi::OsStr) -> Result<PathBuf, LaunchError> {
    let path = Path::new(program);
    if path.components().count() > 1 {
        return canonical(path);
    }
    for directory in std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()) {
        let candidate = directory.join(path);
        if candidate.is_file() {
            return canonical(&candidate);
        }
    }
    Err(LaunchError::new(format!(
        "configured executable {program:?} is not on PATH; install it before running the task"
    )))
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

<!-- fragment «confinement-macos» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="104-155" parent="confinement-policy" -->
````rust
#[cfg(target_os = "macos")]
fn platform_command(root: &Path, reads: &[PathBuf]) -> Result<Command, LaunchError> {
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
(allow file-read* (literal "/private/etc/ssl/cert.pem"))
(allow file-read* file-map-executable (subpath "/private/preboot/Cryptexes/OS") (subpath "/System") (subpath "/usr/bin") (subpath "/usr/sbin") (subpath "/usr/lib") (subpath "/usr/libexec") (subpath "/usr/share") (subpath "/bin") (subpath "/sbin") (subpath "/Library/Apple") (subpath "/opt/homebrew/Cellar") (subpath "/usr/local/lib"))
(allow file-read* (subpath "/private/var/db/timezone") (literal "/dev/random") (literal "/dev/urandom") (literal "/private/etc/passwd") (literal "/private/etc/group") (literal "/private/etc/hosts") (literal "/private/etc/resolv.conf") (literal "/private/etc/services") (literal "/private/etc/protocols") (literal "/private/etc/localtime"))
(allow file-read* file-write-data (literal "/dev/null") (literal "/dev/zero"))
(allow file-read* file-write* file-map-executable (subpath (param "INVOCATION")))
"#,
    );
    let mut command = Command::new("/usr/bin/sandbox-exec");
    command.arg("-D").arg(parameter("INVOCATION", root));
    for (index, path) in reads.iter().enumerate() {
        let name = format!("RUNTIME_{index}");
        profile.push_str(&format!(
            "(allow file-read* file-map-executable (literal (param \"{name}\")))\n"
        ));
        command.arg("-D").arg(parameter(&name, path));
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

<!-- fragment «confinement-linux» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="156-213" parent="confinement-policy" -->
````rust
#[cfg(target_os = "linux")]
fn platform_command(root: &Path, reads: &[PathBuf]) -> Result<Command, LaunchError> {
    // Start with an empty mount namespace, never a read-only bind of `/`.
    // https://github.com/containers/bubblewrap/blob/main/README.md
    let backend = ["/usr/bin/bwrap", "/bin/bwrap"].into_iter().find(|path| Path::new(path).is_file())
        .ok_or_else(|| LaunchError::new("filesystem confinement requires bubblewrap at /usr/bin/bwrap; install the system bubblewrap package"))?;
    let mut command = Command::new(backend);
    // run_confined already creates the session. Keeping the payload in the
    // monitor's group lets its cleanup wait observe every ordinary descendant.
    command.args([
        "--die-with-parent",
        "--unshare-user",
        "--unshare-pid",
        "--unshare-ipc",
        "--unshare-uts",
    ]);
    for path in [
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
    ] {
        if Path::new(path).exists() {
            command.args(["--ro-bind", path, path]);
        }
    }
    command.args(["--proc", "/proc", "--dev", "/dev"]);
    command.arg("--bind").arg(root).arg(root);
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
native backend. This preserves the meaning of `run_confined`: the caller cannot
mistake an unsupported environment for the confined result required before
reading and exporting the example's output.

<!-- fragment «confinement-unavailable» owner="confined-jobs" source="crates/keyed-launch/src/confinement.rs" lines="214-219" parent="confinement-policy" -->
````rust
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn platform_command(_: &Path, _: &[PathBuf]) -> Result<Command, LaunchError> {
    Err(LaunchError::new(
        "standalone confinement is unavailable on this platform; the task was not launched",
    ))
}
````
<!-- /fragment -->

[Previous: What passes through](10-what-passes-through.md) | [Contents](README.md)
