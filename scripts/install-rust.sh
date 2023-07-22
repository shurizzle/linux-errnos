#!/bin/bash

set -eux

toolchains=()

contains() {
  local e match="$1"
  shift
  for e in "$@"; do
    [ "$e" = "$match" ] && return 0
  done
  return 1
}

for arch in "$@"; do
  if [ "$arch" = loongarch64 ]; then
    if ! contains nightly "${toolchains[@]}"; then
      toolchains+=(nightly)
    fi
  else
    if ! contains stable "${toolchains[@]}"; then
      toolchains+=(stable)
    fi
  fi
done

if [ "${#toolchains[@]}" -eq 0 ]; then
  toolchains=(stable nightly)
fi

rustup update "${toolchains[@]}" --no-self-update
for tc in "${toolchains[@]}"; do
  rustup component add clippy rust-src --toolchain "$tc"
done

if [ -n "${GITHUB_ENV:-}" ]; then
  (
    echo 'CARGO_INCREMENTAL=0'
    echo 'CARGO_PROFILE_DEV_DEBUG=0'
    echo 'CARGO_PROFILE_TEST_DEBUG=0'
  ) >"${GITHUB_ENV}"
fi
