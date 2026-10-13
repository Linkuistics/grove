"""Publish a completed sibling directory atomically, refusing existing destinations."""

from __future__ import annotations

import argparse
import ctypes
import os
import sys
from pathlib import Path


def publish(source: Path, destination: Path) -> None:
    """Rename without replacement or cross-filesystem copy fallback."""
    if not source.is_dir() or source.is_symlink():
        raise ValueError(f"source must be a regular directory: {source}")
    library = ctypes.CDLL(None, use_errno=True)
    if sys.platform == "darwin":
        # RENAME_EXCL, macOS >= 10.12; no directory-merging semantics.
        # https://github.com/apple-oss-distributions/xnu/blob/main/bsd/sys/stdio.h
        rename = library.renamex_np
        rename.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint]
        rename.restype = ctypes.c_int
        result = rename(os.fsencode(source), os.fsencode(destination), 4)
    elif sys.platform == "linux":
        # AT_FDCWD=-100, RENAME_NOREPLACE=1; glibc >= 2.28 on the build host.
        # https://man7.org/linux/man-pages/man2/rename.2.html
        rename = library.renameat2
        rename.argtypes = [
            ctypes.c_int,
            ctypes.c_char_p,
            ctypes.c_int,
            ctypes.c_char_p,
            ctypes.c_uint,
        ]
        rename.restype = ctypes.c_int
        result = rename(-100, os.fsencode(source), -100, os.fsencode(destination), 1)
    else:
        raise ValueError(f"atomic publication is unsupported on {sys.platform}")
    if result != 0:
        error = ctypes.get_errno()
        raise OSError(error, os.strerror(error), str(destination))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "source", type=Path, help="completed directory beside destination"
    )
    parser.add_argument(
        "destination", type=Path, help="fresh output; any existing entry refuses"
    )
    arguments = parser.parse_args()
    try:
        publish(arguments.source, arguments.destination)
    except (OSError, ValueError, AttributeError) as error:
        print(f"atomic-publish: refusing publication: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
