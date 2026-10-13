"""Subprocess controls: no rejected preservation may change PATH entries or keg bytes."""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPT = Path(__file__).with_name("legacy_driver_transition.py")
REQUIRED = [
    "LICENSE",
    "bin/grove",
    "bin/grove-llm",
    "bin/harness-dispatch",
    "libexec/harness-dispatch/harness-dispatch-policy",
    "libexec/harness-dispatch/sdk/index.ts",
    "libexec/harness-dispatch/sdk/index.d.ts",
    "libexec/harness-dispatch/notices/NOTICES.md",
    "libexec/harness-dispatch/notices/bun-LICENSE.md",
    "libexec/harness-dispatch/notices/sqlite.md",
]


def fixture(root: Path) -> tuple[Path, Path, Path, dict[str, str]]:
    keg, stable, plan = root / "Cellar/grove/22.1.0", root / "stable", root / "plan"
    stable.mkdir()
    for name in REQUIRED + ["examples/extra", "INSTALL_RECEIPT.json"]:
        path = keg / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(f"original {name}\n")
        if name.startswith("bin/"):
            path.write_text(f"#!/bin/sh\necho '{path.name} 22.1.0'\n")
            path.chmod(0o755)
    (keg / REQUIRED[4]).chmod(0o755)
    env = dict(os.environ)
    env["PATH"] = f"{stable}:/usr/bin:/bin"
    env["HOMEBREW_NO_INSTALL_CLEANUP"] = "1"
    env["HOMEBREW_NO_CLEANUP_FORMULAE"] = "other,grove"
    return keg, stable, plan, env


def run(env: dict[str, str], *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(SCRIPT), *args],
        env=env,
        text=True,
        capture_output=True,
        timeout=30,
        check=False,
    )


def prepare(keg: Path, stable: Path, plan: Path, env: dict[str, str]) -> None:
    result = run(
        env, "prepare", "--keg", str(keg), "--stable-bin", str(stable), "--output", str(plan)
    )
    assert result.returncode == 0, result.stderr


def refuse(result: subprocess.CompletedProcess[str], expected: str) -> None:
    assert result.returncode != 0, f"admitted {expected}"
    assert expected in result.stderr, result.stderr


def check(root: Path, case: str) -> None:
    keg, stable, plan, env = fixture(root)
    if case.startswith("missing:"):
        (keg / case.split(":", 1)[1]).unlink()
        refuse(
            run(
                env,
                "prepare",
                "--keg",
                str(keg),
                "--stable-bin",
                str(stable),
                "--output",
                str(plan),
            ),
            "required",
        )
        assert not plan.exists()
        return
    if case == "outside-path":
        env["PATH"] = "/usr/bin:/bin"
        refuse(
            run(
                env,
                "prepare",
                "--keg",
                str(keg),
                "--stable-bin",
                str(stable),
                "--output",
                str(plan),
            ),
            "inherited PATH",
        )
        assert not plan.exists()
        return
    if case == "external-symlink":
        (keg / "escape").symlink_to(root / "external")
        refuse(
            run(
                env,
                "prepare",
                "--keg",
                str(keg),
                "--stable-bin",
                str(stable),
                "--output",
                str(plan),
            ),
            "symlink",
        )
        assert not plan.exists()
        return
    if case == "wrong-version":
        (keg / "bin/grove-llm").write_text("#!/bin/sh\necho 'grove-llm 23.0.0'\n")
        refuse(
            run(
                env,
                "prepare",
                "--keg",
                str(keg),
                "--stable-bin",
                str(stable),
                "--output",
                str(plan),
            ),
            "original",
        )
        assert not plan.exists()
        return
    prepare(keg, stable, plan, env)
    if case == "complete":
        manifest = json.loads((plan / "manifest.json").read_text())
        assert "examples/extra" in manifest["inventory"]
        assert (plan / "keg/examples/extra").read_bytes() == (keg / "examples/extra").read_bytes()
        assert not (stable / "harness-dispatch").exists()
        assert "HOMEBREW_NO_INSTALL_CLEANUP" in (plan / "release-env.sh").read_text()
        return
    if case == "release-environment":
        sourced_env = dict(env)
        sourced_env.pop("HOMEBREW_NO_INSTALL_CLEANUP")
        sourced_env["HOMEBREW_NO_CLEANUP_FORMULAE"] = "other"
        shell = subprocess.run(
            [
                "/bin/sh",
                "-c",
                '. "$1"; exec "$2" -c "import os,json; print(json.dumps(dict(os.environ)))"',
                "fixture",
                str(plan / "release-env.sh"),
                sys.executable,
            ],
            env=sourced_env,
            capture_output=True,
            text=True,
            timeout=15,
            check=False,
        )
        assert shell.returncode == 0, shell.stderr
        exported = json.loads(shell.stdout)
        assert exported["HOMEBREW_NO_INSTALL_CLEANUP"] == "1"
        assert exported["HOMEBREW_NO_CLEANUP_FORMULAE"] == "other,grove"
        assert Path(exported["GROVE_ORIGINAL_LLM"]) == (keg / "bin/grove-llm").resolve()
        return
    if case == "repeat-prepare":
        refuse(
            run(
                env,
                "prepare",
                "--keg",
                str(keg),
                "--stable-bin",
                str(stable),
                "--output",
                str(plan),
            ),
            "exists",
        )
        return
    if case in {"no-auto-protection", "no-formula-protection"}:
        env.pop(
            "HOMEBREW_NO_INSTALL_CLEANUP"
            if case == "no-auto-protection"
            else "HOMEBREW_NO_CLEANUP_FORMULAE"
        )
        refuse(run(env, "apply", "--plan", str(plan)), "cleanup")
        assert list(stable.iterdir()) == []
        return
    if case in {"existing-first", "existing-second", "dangling-link"}:
        entry = stable / ("grove-llm" if case == "existing-second" else "harness-dispatch")
        if case == "dangling-link":
            entry.symlink_to(root / "missing")
        else:
            entry.write_text("owner")
        refuse(run(env, "apply", "--plan", str(plan)), "existing")
        assert len(list(stable.iterdir())) == 1
        assert entry.is_symlink() or entry.read_text() == "owner"
        return
    if case in {"changed-keg", "changed-backup", "missing-keg", "mode-change"}:
        subject = plan / "keg" if case == "changed-backup" else keg
        if case == "missing-keg":
            (subject / "bin/harness-dispatch").unlink()
        elif case == "mode-change":
            (subject / "bin/harness-dispatch").chmod(0o644)
        else:
            (subject / "examples/extra").write_text("changed")
        refuse(run(env, "apply", "--plan", str(plan)), "inventory")
        assert list(stable.iterdir()) == []
        return
    if case == "shadowed-path":
        shadow = root / "shadow"
        shadow.mkdir()
        program = shadow / "harness-dispatch"
        program.write_text("#!/bin/sh\nexit 0\n")
        program.chmod(0o755)
        env["PATH"] = f"{shadow}:{env['PATH']}"
        refuse(run(env, "apply", "--plan", str(plan)), "shadows")
        assert list(stable.iterdir()) == []
        return
    assert run(env, "apply", "--plan", str(plan)).returncode == 0
    if case == "applied":
        assert run(env, "verify", "--plan", str(plan)).returncode == 0
        for name in ["harness-dispatch", "grove-llm"]:
            assert (stable / name).resolve() == (keg / "bin" / name).resolve()
        refuse(run(env, "apply", "--plan", str(plan)), "existing")
        return
    if case == "cleanup-live":
        refuse(run(env, "cleanup", "--plan", str(plan)), "driver-ended")
        assert len(list(stable.iterdir())) == 2
        return
    if case == "cleanup-owner-change":
        (stable / "grove-llm").unlink()
        (stable / "grove-llm").write_text("replacement")
        refuse(run(env, "cleanup", "--plan", str(plan), "--driver-ended"), "owned")
        assert (stable / "harness-dispatch").is_symlink()
        return
    if case == "cleanup-same-target-replacement":
        entry = stable / "grove-llm"
        target = os.readlink(entry)
        entry.unlink()
        entry.symlink_to(target)
        refuse(run(env, "verify", "--plan", str(plan)), "owned")
        refuse(run(env, "cleanup", "--plan", str(plan), "--driver-ended"), "owned")
        assert (stable / "harness-dispatch").is_symlink() and entry.is_symlink()
        return
    if case == "cleanup-ended":
        assert run(env, "cleanup", "--plan", str(plan), "--driver-ended").returncode == 0
        assert list(stable.iterdir()) == []
        assert keg.exists() and (plan / "keg").exists()
        return
    raise AssertionError(case)


def main() -> None:
    cases = [
        "release-environment",
        "complete",
        "repeat-prepare",
        "outside-path",
        "external-symlink",
        "wrong-version",
        "no-auto-protection",
        "no-formula-protection",
        "existing-first",
        "existing-second",
        "dangling-link",
        "changed-keg",
        "changed-backup",
        "missing-keg",
        "mode-change",
        "applied",
        "cleanup-live",
        "cleanup-owner-change",
        "cleanup-same-target-replacement",
        "cleanup-ended",
        "shadowed-path",
    ]
    cases += [f"missing:{name}" for name in REQUIRED]
    for case in cases:
        with tempfile.TemporaryDirectory() as scratch:
            check(Path(scratch), case)
        print(f"PASS {case}")
    print(f"legacy transition controls: {len(cases)} passed")


if __name__ == "__main__":
    main()
