#!/bin/bash

set -eux

if [ $# -eq 0 ]; then
  set -- arm loongarch64 x86_64 x86 aarch64 riscv64 powerpc powerpc64 mips \
    mips64 s390x
fi

contains() {
  local e match="$1"
  shift
  for e in "$@"; do
    [ "$e" = "$match" ] && return 0
  done
  return 1
}

install_arm() {
  rustup target add armv7-{unknown-linux-gnu,linux-android}eabi --toolchain stable
}

install_loongarch64() {
  rustup target add loongarch64-unknown-linux-gnu --toolchain nightly
}

install_x86_64() {
  rustup target add x86_64-{unknown-linux-gnu,linux-android} --toolchain stable
}

install_x86() {
  rustup target add i686-{unknown-linux-gnu,linux-android} --toolchain stable
}

install_aarch64() {
  rustup target add aarch64-{unknown-linux-gnu,linux-android} --toolchain stable
}

install_riscv64() {
  rustup target add riscv64gc-unknown-linux-gnu --toolchain stable
}

install_powerpc() {
  rustup target add powerpc-unknown-linux-gnu --toolchain stable
}

install_powerpc64() {
  rustup target add powerpc64-unknown-linux-gnu --toolchain stable
}

install_mips() {
  rustup target add mips-unknown-linux-gnu --toolchain stable
}

install_mips64() {
  rustup target add mips64-unknown-linux-gnuabi64 --toolchain stable
}

install_s390x() {
  rustup target add s390x-unknown-linux-gnu --toolchain stable
}

archs=()
for arch in "$@"; do
  if ! contains "$arch" "${archs[@]}"; then
    archs+=("$arch")
    "install_${arch}"
  fi
done
