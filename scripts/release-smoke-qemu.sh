#!/usr/bin/env bash
#
# One Linux target's installed smoke test in the glibc-2.17 userland under a
# pinned user-mode QEMU, for a target Docker cannot run itself.
# scripts/release-smoke.sh runs this as root in a helper container of Docker's
# own architecture; it is not an entry point of its own.
#
#   release-smoke-qemu.sh INPUTS TARGET ARCHIVE
#
# INPUTS, read-only, holds everything the run reads: rootfs.tar, the floor
# image's filesystem for TARGET's architecture; qemu, the pinned emulator;
# interpreter, the binfmt_misc interpreter release-smoke.sh builds, which runs
# qemu with the options this target needs; release-smoke-target.sh,
# installed-smoke.sh, the glibc probes in probes/, and ARCHIVE.
#
# The userland is the same image the container route runs, unpacked into
# /floor and entered with chroot as uid 1000. Its foreign executables reach
# qemu through binfmt_misc, and the registration must not touch the Docker
# host's own, which every other container of that architecture uses. So it
# goes into a private binfmt_misc instance, which the kernel (6.7 and later)
# gives each user namespace that mounts one. The namespace maps uids and gids
# 0-65535 to themselves, so root inside it is the container's root, and uid
# 1000 in the chroot owns nothing there, as in the container route.
#
# Before the smoke test, the guest's own /proc/self/maps must show a vDSO and
# no mapping at or above the target's user address limit but the vsyscall
# page. Docker Desktop's QEMU 8.1.5 gives x86-64 guests no vDSO, and QEMU
# 10.2.3 without a guest base maps them above 2^47 (docs/design/
# harness-selection-and-execution/runtime-evidence.md, *Installed smoke*); either
# would otherwise run the smoke test in an environment no x86-64 kernel makes.

set -euo pipefail
IFS=$'\n\t'

readonly FLOOR=/floor

die() {
  echo "release-smoke: ${TARGET:-qemu}: FAIL: $*" >&2
  exit 1
}

note() {
  echo "release-smoke: $TARGET: $*"
}

# binfmt_misc's magic and mask for ARCH's ELF executables, as QEMU's own
# scripts/qemu-binfmt-conf.sh registers them, then the lowest address ARCH's
# user space never reaches.
# https://gitlab.com/qemu-project/qemu/-/blob/v10.2.3/scripts/qemu-binfmt-conf.sh
guest() {
  case "$1" in
    x86_64)
      printf '%s\n' \
        '\x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x3e\x00' \
        '\xff\xff\xff\xff\xff\xfe\xfe\x00\xff\xff\xff\xff\xff\xff\xff\xff\xfe\xff\xff\xff' \
        800000000000
      ;;
    *) die "no user-mode guest is known for $1" ;;
  esac
}

# Run as the container's root: unpack the userland, then enter a new user and
# mount namespace. A process inside that namespace may map only its own uid,
# and `unshare --map-users` needs newuidmap, which this image lacks, so the
# container's root, outside the namespace, writes the identity maps. The child
# waits on a FIFO until then, and re-executes this script so that it holds its
# capabilities in the namespace: execve grants them to uid 0, and the child
# was unmapped when it last executed.
outside() {
  local inputs="$1" archive="$3" arch
  arch="${TARGET%%-*}"
  guest "$arch" >/dev/null
  mkdir "$FLOOR" "$FLOOR/qemu"
  tar -xf "$inputs/rootfs.tar" -C "$FLOOR" || die "cannot unpack the floor image's filesystem"
  cp "$inputs/qemu" "$inputs/interpreter" "$FLOOR/qemu/"
  note "userland unpacked; $("$FLOOR/qemu/qemu" --version | head -n 1)"

  local sync own pid status=0
  sync="$(mktemp -d)"
  mkfifo "$sync/mapped"
  own="$(readlink /proc/self/ns/user)"
  # shellcheck disable=SC2016 # the child shell expands its own arguments
  unshare --user --mount /bin/bash -c 'read -r _ <"$1" && shift && exec /bin/bash "$@"' \
    _ "$sync/mapped" "$0" --in-namespace "$inputs" "$TARGET" "$archive" &
  pid=$!
  until [[ "$(readlink "/proc/$pid/ns/user" 2>/dev/null)" != "$own" ]]; do :; done
  kill -0 "$pid" 2>/dev/null || die "unshare could not create a user namespace"
  echo '0 0 65536' >"/proc/$pid/uid_map"
  echo '0 0 65536' >"/proc/$pid/gid_map"
  echo >"$sync/mapped"
  wait "$pid" || status=$?
  return "$status"
}

# Run as root in the namespace: register the interpreter, bind the inputs,
# /proc and /dev into the userland, check the guest's address space, and run
# the smoke test there as uid 1000.
in_namespace() {
  local inputs="$1" archive="$3" arch magic mask limit binfmt
  arch="${TARGET%%-*}"
  { read -r magic && read -r mask && read -r limit; } < <(guest "$arch")
  binfmt="$(mktemp -d)"
  mount -t binfmt_misc binfmt_misc "$binfmt" || die "cannot mount a private binfmt_misc instance"
  # F opens the interpreter now, outside the chroot; the interpreter finds qemu
  # inside it. P passes the executable's own argv[0], which this qemu takes
  # from the argument after the executable's path.
  printf '%s' ":release-smoke-$arch:M::$magic:$mask:$FLOOR/qemu/interpreter:PF" >"$binfmt/register" ||
    die "cannot register the interpreter"
  mkdir -p "$FLOOR/smoke"
  # A bind keeps the read-only flag of release-smoke.sh's volume, which the
  # namespace cannot clear.
  mount --bind "$inputs" "$FLOOR/smoke"
  mount --rbind /proc "$FLOOR/proc"
  mount --rbind /dev "$FLOOR/dev"

  local maps line end vdso=0
  maps="$(chroot --userspec=1000:1000 "$FLOOR" /usr/bin/cat /proc/self/maps)" ||
    die "the userland's cat did not run under $FLOOR/qemu/interpreter"
  while IFS= read -r line; do
    case "$line" in
      *'[vsyscall]') continue ;;
      *'[vdso]') vdso=1 ;;
    esac
    end="${line%% *}"
    end="${end#*-}"
    # Fifteen hex digits at most, so bash's signed arithmetic cannot wrap.
    ((${#end} < 16 && 16#$end <= 16#$limit)) || die "the guest has a mapping above 0x$limit: $line"
  done <<<"$maps"
  ((vdso)) || die "the guest has no vDSO: $maps"
  note "guest address space below 0x$limit, with a vDSO"

  exec chroot --userspec=1000:1000 "$FLOOR" \
    /usr/bin/env -i PATH=/usr/bin:/bin /bin/bash /smoke/release-smoke-target.sh \
    "$TARGET" "/smoke/$archive" /smoke/installed-smoke.sh /smoke/probes
}

main() {
  local phase=outside
  if [[ "${1:-}" == --in-namespace ]]; then
    phase=in_namespace
    shift
  fi
  (($# == 3)) || {
    echo "usage: release-smoke-qemu.sh INPUTS TARGET ARCHIVE" >&2
    exit 1
  }
  TARGET="$2"
  "$phase" "$@"
}

main "$@"
