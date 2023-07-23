#!/bin/bash

SCRIPTPATH="$(
  cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
  pwd -P
)"

cd "${SCRIPTPATH}/.."

set -eux

contains() {
  local e match="$1"
  shift
  for e in "$@"; do
    [ "$e" = "$match" ] && return 0
  done
  return 1
}

# cargo_test(toolchain, target, arch)
cargo_test() {
  rm -rf Cargo.lock target
  "${SCRIPTPATH}/docker-run.sh" "$1" "$3" \
    cargo test -vvv \
    --target "$2" \
    --no-default-features \
    --release
  "${SCRIPTPATH}/docker-run.sh" "$1" "$3" \
    cargo test -vvv \
    --target "$2" \
    --all-features \
    --release
}

# cargo_clippy(toolchain, arch, target)
cargo_clippy() {
  "${SCRIPTPATH}/docker-run.sh" "$1" "$2" \
    cargo -vvv clippy \
    --target "$3" \
    -- -D warnings
}

# test_nightly(target, arch)
test_nightly() {
  cargo_clippy nightly "$2" "$1"
  cargo_test nightly "$@"
}

# test_stable(target, arch, toolchain?)
test_stable() {
  cargo_clippy stable "$2" "$1"
  cargo_test stable "$@"
}

test_x86_64() {
  test_stable x86_64-unknown-linux-gnu x86_64
  test_stable x86_64-linux-android x86_64
}

test_x86() {
  test_stable i686-unknown-linux-gnu x86
  test_stable i686-linux-android x86
}

test_arm() {
  test_stable armv7-unknown-linux-gnueabi arm
  test_stable armv7-linux-androideabi arm
}

test_aarch64() {
  test_stable aarch64-unknown-linux-gnu aarch64
  test_stable aarch64-linux-android aarch64
}

test_loongarch64() {
  test_nightly loongarch64-unknown-linux-gnu loongarch64
}

test_riscv64() {
  test_stable riscv64gc-unknown-linux-gnu riscv64
}

test_powerpc() {
  test_stable powerpc-unknown-linux-gnu powerpc
}

test_powerpc64() {
  test_stable powerpc64-unknown-linux-gnu powerpc64
}

test_mips() {
  test_stable mips-unknown-linux-gnu mips
}

test_mips64() {
  test_stable mips64-unknown-linux-gnuabi64 mips64
}

test_s390x() {
  test_stable s390x-unknown-linux-gnu s390x
}

if [ $# -eq 0 ]; then
  set -- arm loongarch64 x86_64 x86 aarch64 riscv64 powerpc powerpc64 mips \
    mips64 s390x
fi

archs=()
for arch in "$@"; do
  if ! contains "$arch" "${archs[@]}"; then
    archs+=("$arch")
    "test_${arch}"
  fi
done
