"""Rehearse upgrade/unlink using actual old bytes and fake agents, never a Grove driver."""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

import legacy_driver_transition as transition

# The managed fake process stays alive with a descendant after signalling,
# so success must include actual supervision/reaping, not just file creation.
AGENT = r"""
import json, os, subprocess, sys, time
from pathlib import Path
mode, root_arg, ravel_arg, python_arg = sys.argv[1:]
root, ravel = Path(root_arg), Path(ravel_arg)
descendant = subprocess.Popen(['/bin/sleep', '60'])
record = {'pid': os.getpid(), 'descendant': descendant.pid,
          'controls': {k: v for k, v in os.environ.items()
                       if k.startswith(('HARNESS_DISPATCH_', 'RAVEL_', 'GROVE_'))}}
if mode == 'nested':
    assert not any(k.startswith('HARNESS_DISPATCH_') for k in record['controls']), record
    assert not any(k.startswith('GROVE_') for k in record['controls']), record
    assert 'RAVEL_EXIT_FILE' in record['controls']
    legacy = json.loads((root / 'legacy.json').read_text())
    before = legacy['controls']['HARNESS_DISPATCH_EXIT_FILE']
    assert not Path(before).exists()
    ended = subprocess.run(['harness-dispatch', 'exit'], capture_output=True, text=True)
    assert ended.returncode == 0 and not Path(before).exists(), ended
    subprocess.run([str(ravel), 'end-session', 'independent-result'], check=True)
    assert Path(record['controls']['RAVEL_EXIT_FILE']).read_text() == 'independent-result'
    assert not Path(before).exists()
    (root / 'nested.json').write_text(json.dumps(record))
else:
    assert 'HARNESS_DISPATCH_EXIT_FILE' in record['controls'], record
    assert not any(k.startswith('RAVEL_') for k in record['controls']), record
    assert not any(k.startswith('GROVE_') for k in record['controls']), record
    (root / 'legacy.json').write_text(json.dumps(record))
    refused = subprocess.run([str(ravel), 'end-session', 'must-refuse'], capture_output=True)
    assert refused.returncode != 0
    result = subprocess.run([str(ravel), 'start-session', '--kind', 'fixture',
                             '--config', str(root / 'nested-policy.ts'), '--prompt', 'unchanged',
                             '--exit-dir', str(root / 'ravel-channel')],
                            capture_output=True, text=True, timeout=20)
    (root / 'nested.stdout').write_text(result.stdout)
    (root / 'nested.stderr').write_text(result.stderr)
    assert result.returncode == 0 and result.stdout == 'independent-result', result
    channel = Path(record['controls']['HARNESS_DISPATCH_EXIT_FILE'])
    assert not channel.exists()
    subprocess.run(['harness-dispatch', 'exit'], check=True)
    assert channel.is_file()
    (root / 'legacy-signalled').write_text(str(channel))
# Reap the child on TERM; the supervisor must still terminate/reap this agent.
import signal
def stop(signum, frame):
    descendant.wait(timeout=5)
    sys.exit(0)
signal.signal(signal.SIGTERM, stop)
time.sleep(60)
"""


def policy(path: Path, sdk: str, mode: str, root: Path, ravel: Path) -> None:
    args = [str(root / "agent.py"), mode, str(root), str(ravel), sys.executable]
    path.write_text(
        f'import {{ definePolicy }} from "{sdk}";\n'
        'export const policy = definePolicy({schemaVersion: 2, version: "legacy-rehearsal",\n'
        'select(request) { return {status: "selected", '
        f"program: {json.dumps(sys.executable)}, args: {json.dumps(args)}, "
        'provider: "fixture", model: "none", effort: "none", reason: "fake agent"}; }});\n'
    )


def command(
    args: list[str], env: dict[str, str], cwd: Path, log: str
) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(
        args, env=env, cwd=cwd, capture_output=True, text=True, timeout=30, check=False
    )
    (cwd / f"{log}.stdout").write_text(result.stdout)
    (cwd / f"{log}.stderr").write_text(result.stderr)
    return result


def absent(pid: int) -> None:
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return
    raise AssertionError(f"managed process still exists after reap: {pid}")


def rehearse(original: Path, ravel: Path, output: Path) -> None:
    if output.exists() or output.is_symlink():
        raise transition.TransitionError(f"rehearsal output exists: {output}")
    before = transition.inventory(original.resolve())
    output.mkdir(parents=True)
    output = output.resolve()
    original = original.resolve()
    ravel = ravel.resolve(strict=True)
    # Independent Ravel remains at its actual separate installation/version.
    ravel_keg = ravel.parent.parent
    ravel_before = transition.inventory(ravel_keg)
    keg = output / "prefix/Cellar/grove/22.1.0"
    shutil.copytree(original, keg, symlinks=True)
    assert transition.inventory(keg) == before
    stable, prefix_bin = output / "stable-bin", output / "prefix/bin"
    stable.mkdir()
    prefix_bin.mkdir()
    tools = output / "tools"
    tools.mkdir()
    jj = shutil.which("jj")
    if jj is None:
        raise transition.TransitionError("rehearsal requires jj for isolated helper admission")
    (tools / "jj").symlink_to(Path(jj).resolve())
    for name in ("grove", "grove-llm", "harness-dispatch"):
        (prefix_bin / name).symlink_to(keg / "bin" / name)
    for name in ("home", "tmp", "legacy-channel", "ravel-channel", "store", "project"):
        (output / name).mkdir()
    env = transition.clean_environment()
    env.update(
        HOME=str(output / "home"),
        CODEX_HOME=str(output / "home/codex"),
        XDG_CONFIG_HOME=str(output / "home/.config"),
        XDG_STATE_HOME=str(output / "home/.local/state"),
        TMPDIR=str(output / "tmp"),
        TMP=str(output / "tmp"),
        TEMP=str(output / "tmp"),
        PATH=f"{stable}:{prefix_bin}:{tools}:/usr/bin:/bin",
        HOMEBREW_NO_INSTALL_CLEANUP="1",
        HOMEBREW_NO_CLEANUP_FORMULAE="grove",
    )
    script = str(Path(transition.__file__).resolve())
    result = command(
        [
            sys.executable,
            script,
            "prepare",
            "--keg",
            str(keg),
            "--stable-bin",
            str(stable),
            "--output",
            str(output / "plan"),
        ],
        env,
        output,
        "prepare",
    )
    assert result.returncode == 0, result.stderr
    result = command(
        [sys.executable, script, "apply", "--plan", str(output / "plan")], env, output, "apply"
    )
    assert result.returncode == 0, result.stderr
    # Homebrew upgrade removes public old links and adds the new two-front layout.
    # It does not alter the complete canonical old keg while cleanup is suppressed.
    for name in ("grove", "grove-llm", "harness-dispatch"):
        (prefix_bin / name).unlink()
    new_keg = output / "prefix/Cellar/grove/23.0.0/bin"
    new_keg.mkdir(parents=True)
    for name in ("grove", "grove-llm"):
        (new_keg / name).write_text("#!/bin/sh\necho new-version-fixture\n")
        (new_keg / name).chmod(0o755)
        (prefix_bin / name).symlink_to(new_keg / name)
    assert not (prefix_bin / "harness-dispatch").exists()
    result = command(
        [sys.executable, script, "verify", "--plan", str(output / "plan")],
        env,
        output,
        "verify-after-upgrade",
    )
    assert result.returncode == 0, result.stderr
    (output / "agent.py").write_text(AGENT)
    policy(output / "legacy-policy.ts", "harness-dispatch/sdk", "legacy", output, ravel)
    policy(output / "nested-policy.ts", "ravel/sdk", "nested", output, ravel)
    # Baits deliberately replace any real inherited channels. Neither may change.
    for name in ("legacy-bait", "ravel-bait", "grove-bait"):
        (output / name).write_text("untouched")
    env.update(
        HARNESS_DISPATCH_EXIT_FILE=str(output / "legacy-bait"),
        RAVEL_EXIT_FILE=str(output / "ravel-bait"),
        GROVE_LAUNCH_DIR=str(output / "grove-bait"),
    )
    # Direct test launch helpers scrub authority, including the above known-wrong baits.
    env = {
        key: value
        for key, value in env.items()
        if key not in transition.CONTROLS and not key.startswith(("HARNESS_DISPATCH_", "RAVEL_"))
    }
    front = keg / "bin/harness-dispatch"
    ended = output / "legacy-channel/ending.json"
    result = command(
        [
            str(front),
            "run",
            "--kind",
            "fixture",
            "--prompt",
            "next fake session",
            "--config",
            str(output / "legacy-policy.ts"),
            "--state-dir",
            str(output / "store"),
            "--exit-dir",
            str(output / "legacy-channel"),
            "--ending-file",
            str(ended),
        ],
        env,
        output,
        "legacy-run",
    )
    assert result.returncode == 0, result.stderr
    ending = json.loads(ended.read_text())
    assert ending["measurements"]["ending"]["value"] == "exit_signal", ending
    assert (output / "legacy-signalled").exists()
    for name in ("legacy", "nested"):
        record = json.loads((output / f"{name}.json").read_text())
        absent(record["pid"])
        absent(record["descendant"])
    record = json.loads((output / "legacy.json").read_text())
    legacy_channel = Path(record["controls"]["HARNESS_DISPATCH_EXIT_FILE"])
    assert legacy_channel.is_relative_to(output / "legacy-channel")
    nested = json.loads((output / "nested.json").read_text())
    assert Path(nested["controls"]["RAVEL_EXIT_FILE"]).is_relative_to(output / "ravel-channel")
    exported = command(
        [
            str(front),
            "record",
            "show",
            "--run",
            record["controls"]["HARNESS_DISPATCH_RUN_ID"],
            "--state-dir",
            str(output / "store"),
            "--json",
        ],
        env,
        output,
        "old-store",
    )
    assert exported.returncode == 0, exported.stderr
    stored = json.loads(exported.stdout)
    assert "exit_signal" in json.dumps(stored) and "execution_confirmed" in json.dumps(stored)
    for name in ("legacy-bait", "ravel-bait", "grove-bait"):
        assert (output / name).read_text() == "untouched"
    helper = keg / "bin/grove-llm"
    version = command([str(helper), "--version"], env, output, "original-helper-version")
    assert version.stdout.strip() == "grove-llm 22.1.0"
    outside = command([str(helper), "record-teardown"], env, output, "outside-teardown")
    assert outside.returncode == 0 and "recorded nothing" in outside.stderr
    project = output / "project"
    initialized = command([str(tools / "jj"), "git", "init"], env, project, "jj-init")
    assert initialized.returncode == 0, initialized.stderr
    stale_env = env | {"GROVE_LAUNCH_DIR": str(output / "legacy-channel")}
    stale = command([str(helper), "record-teardown"], stale_env, project, "stale-admission")
    assert stale.returncode != 0 and not (output / "legacy-channel/teardown").exists()
    assert "stale" in stale.stderr, stale.stderr
    # Actual missing-worker and deleted-keg controls, after all positive routes finish.
    worker = keg / "libexec/harness-dispatch/harness-dispatch-policy"
    hidden = worker.with_name("hidden-worker")
    worker.rename(hidden)
    missing = command(
        [str(front), "inspect", "--kind", "fixture", "--config", str(output / "legacy-policy.ts")],
        env,
        output,
        "missing-worker",
    )
    assert missing.returncode != 0
    refused = command(
        [sys.executable, script, "verify", "--plan", str(output / "plan")],
        env,
        output,
        "missing-worker-inventory",
    )
    assert refused.returncode != 0
    hidden.rename(worker)
    hidden_keg = keg.with_name("hidden-keg")
    keg.rename(hidden_keg)
    refused = command(
        [sys.executable, script, "verify", "--plan", str(output / "plan")],
        env,
        output,
        "missing-keg",
    )
    assert refused.returncode != 0
    hidden_keg.rename(keg)
    result = command(
        [sys.executable, script, "cleanup", "--plan", str(output / "plan"), "--driver-ended"],
        env,
        output,
        "cleanup-after-reap",
    )
    assert result.returncode == 0 and not (stable / "harness-dispatch").exists()
    assert transition.inventory(keg) == before
    assert transition.inventory(original) == before
    assert transition.inventory(ravel_keg) == ravel_before
    (output / "protected-before.json").write_text(
        json.dumps({"original": before, "ravel": ravel_before}, indent=2)
    )
    (output / "protected-after.json").write_text(
        json.dumps(
            {"original": transition.inventory(original), "ravel": transition.inventory(ravel_keg)},
            indent=2,
        )
    )
    print(
        "PASS actual original canonical launch after upgrade/unlink; bare exit; old store; reaping"
    )
    print("PASS nested independent Ravel result/channel; wrong ending isolation; untouched baits")
    print(
        "PASS original helper/version/no-op/stale refusal; worker/keg refusals; post-reap cleanup"
    )
    print(f"PASS complete original and Ravel inventories unchanged; logs: {output}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--keg", type=Path, required=True)
    parser.add_argument("--ravel", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    rehearse(args.keg, args.ravel, args.output)


if __name__ == "__main__":
    main()
