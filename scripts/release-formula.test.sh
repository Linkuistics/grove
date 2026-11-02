#!/usr/bin/env bash
# Test a native archive/template without changing any keg, tap or release.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=scripts/release-common.sh
source "$root/scripts/release-common.sh"
# The formula test checks the TypeScript set against the 1.1 prefix and the
# Rhai set against the 2.0 prefix, each named explicitly, because a build of
# Ravel's 2.0 feature line reports `ravel 1.1.0` until the cut.
[[ $# == 3 ]] || {
  echo 'usage: release-formula.test.sh NATIVE_ARCHIVE RAVEL_1_1_PREFIX RAVEL_2_0_PREFIX' >&2
  exit 1
}
archive="$1"
ravel_1_1="$(cd "$2" && pwd -P)/bin/ravel"
ravel_2_0="$(cd "$3" && pwd -P)/bin/ravel"
[[ -x "$ravel_1_1" && -x "$ravel_2_0" ]] || { echo 'each Ravel prefix needs bin/ravel' >&2; exit 1; }
[[ -x "${ravel_1_1%/bin/ravel}/libexec/ravel/ravel-policy" ]] || { echo 'the 1.1 prefix holds no 1.x worker' >&2; exit 1; }
[[ ! -e "${ravel_2_0%/bin/ravel}/libexec/ravel" ]] || { echo 'the 2.0 prefix holds a 1.x worker' >&2; exit 1; }
top="$(basename "$archive" .tar.xz)"
version="${top#grove-v}"
version="${version%-aarch64-apple-darwin}"
[[ "$top" == "grove-v$version-aarch64-apple-darwin" ]] || { echo 'expected native Grove archive' >&2; exit 1; }
assert_archive "$archive" "$top"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
extract_exactly "$archive" "$scratch"
mkdir "$scratch/prefix" "$scratch/test"
# Tests never fetch a formula URL. Checksum placeholders are valid fixture
# values; the production release build renders all three real archive hashes.
sha="$(shasum -a 256 "$archive" | cut -d' ' -f1)"
sed -e "s/@VERSION@/$version/g" -e "s/@SHA_[A-Z_]*@/$sha/g" \
  "$root/scripts/templates/grove.rb.tmpl" >"$scratch/grove.rb"
HOMEBREW_NO_AUTO_UPDATE=1 brew ruby "$root/scripts/release-formula.test.rb" \
  "$scratch/grove.rb" "$scratch/$top" "$scratch/prefix" "$scratch/test" "$ravel_1_1" "$ravel_2_0"
