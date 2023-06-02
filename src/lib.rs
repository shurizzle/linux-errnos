//! Error numbers for every arch supported by linux.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod linux;
pub(crate) mod macros;

#[cfg(any(feature = "arm", all(target_os = "linux", target_arch = "arm")))]
pub mod arm {
    //! Error numbers for arch `arm`. Activated with feature `arm` or with target_os `linux` and target_arch `arm`.
    pub use super::linux::arm::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::arm::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub use arm::Errno;
#[cfg(all(target_os = "linux", target_arch = "arm", feature = "iter"))]
pub use arm::ErrnoIter;
#[cfg(any(feature = "aarch64", all(target_os = "linux", target_arch = "aarch64")))]
pub mod aarch64 {
    //! Error numbers for arch `aarch64`. Activated with feature `aarch64` or with target_os `linux` and target_arch `aarch64`.
    pub use super::linux::arm64::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::arm64::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub use aarch64::Errno;
#[cfg(all(target_os = "linux", target_arch = "aarch64", feature = "iter"))]
pub use aarch64::ErrnoIter;
#[cfg(any(feature = "hexagon", all(target_os = "linux", target_arch = "hexagon")))]
pub mod hexagon {
    //! Error numbers for arch `hexagon`. Activated with feature `hexagon` or with target_os `linux` and target_arch `hexagon`.
    pub use super::linux::hexagon::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::hexagon::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "hexagon"))]
pub use hexagon::Errno;
#[cfg(all(target_os = "linux", target_arch = "hexagon", feature = "iter"))]
pub use hexagon::ErrnoIter;
#[cfg(any(
    feature = "loongarch64",
    all(target_os = "linux", target_arch = "loongarch64")
))]
pub mod loongarch64 {
    //! Error numbers for arch `loongarch64`. Activated with feature `loongarch64` or with target_os `linux` and target_arch `loongarch64`.
    pub use super::linux::loongarch::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::loongarch::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "loongarch64"))]
pub use loongarch64::Errno;
#[cfg(all(target_os = "linux", target_arch = "loongarch64", feature = "iter"))]
pub use loongarch64::ErrnoIter;
#[cfg(any(feature = "m68k", all(target_os = "linux", target_arch = "m68k")))]
pub mod m68k {
    //! Error numbers for arch `m68k`. Activated with feature `m68k` or with target_os `linux` and target_arch `m68k`.
    pub use super::linux::m68k::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::m68k::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "m68k"))]
pub use m68k::Errno;
#[cfg(all(target_os = "linux", target_arch = "m68k", feature = "iter"))]
pub use m68k::ErrnoIter;
#[cfg(any(feature = "mips", all(target_os = "linux", target_arch = "mips")))]
pub mod mips {
    //! Error numbers for arch `mips`. Activated with feature `mips` or with target_os `linux` and target_arch `mips`.
    pub use super::linux::mips::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::mips::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "mips"))]
pub use mips::Errno;
#[cfg(all(target_os = "linux", target_arch = "mips", feature = "iter"))]
pub use mips::ErrnoIter;
#[cfg(any(feature = "mips64", all(target_os = "linux", target_arch = "mips64")))]
pub mod mips64 {
    //! Error numbers for arch `mips64`. Activated with feature `mips64` or with target_os `linux` and target_arch `mips64`.
    pub use super::linux::mips::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::mips::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "mips64"))]
pub use mips64::Errno;
#[cfg(all(target_os = "linux", target_arch = "mips64", feature = "iter"))]
pub use mips64::ErrnoIter;
#[cfg(any(feature = "powerpc", all(target_os = "linux", target_arch = "powerpc")))]
pub mod powerpc {
    //! Error numbers for arch `powerpc`. Activated with feature `powerpc` or with target_os `linux` and target_arch `powerpc`.
    pub use super::linux::powerpc::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::powerpc::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "powerpc"))]
pub use powerpc::Errno;
#[cfg(all(target_os = "linux", target_arch = "powerpc", feature = "iter"))]
pub use powerpc::ErrnoIter;
#[cfg(any(
    feature = "powerpc64",
    all(target_os = "linux", target_arch = "powerpc64")
))]
pub mod powerpc64 {
    //! Error numbers for arch `powerpc64`. Activated with feature `powerpc64` or with target_os `linux` and target_arch `powerpc64`.
    pub use super::linux::powerpc::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::powerpc::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "powerpc64"))]
pub use powerpc64::Errno;
#[cfg(all(target_os = "linux", target_arch = "powerpc64", feature = "iter"))]
pub use powerpc64::ErrnoIter;
#[cfg(any(feature = "riscv32", all(target_os = "linux", target_arch = "riscv32")))]
pub mod riscv32 {
    //! Error numbers for arch `riscv32`. Activated with feature `riscv32` or with target_os `linux` and target_arch `riscv32`.
    pub use super::linux::riscv::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::riscv::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "riscv32"))]
pub use riscv32::Errno;
#[cfg(all(target_os = "linux", target_arch = "riscv32", feature = "iter"))]
pub use riscv32::ErrnoIter;
#[cfg(any(feature = "riscv64", all(target_os = "linux", target_arch = "riscv64")))]
pub mod riscv64 {
    //! Error numbers for arch `riscv64`. Activated with feature `riscv64` or with target_os `linux` and target_arch `riscv64`.
    pub use super::linux::riscv::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::riscv::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "riscv64"))]
pub use riscv64::Errno;
#[cfg(all(target_os = "linux", target_arch = "riscv64", feature = "iter"))]
pub use riscv64::ErrnoIter;
#[cfg(any(feature = "s390x", all(target_os = "linux", target_arch = "s390x")))]
pub mod s390x {
    //! Error numbers for arch `s390x`. Activated with feature `s390x` or with target_os `linux` and target_arch `s390x`.
    pub use super::linux::s390::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::s390::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "s390x"))]
pub use s390x::Errno;
#[cfg(all(target_os = "linux", target_arch = "s390x", feature = "iter"))]
pub use s390x::ErrnoIter;
#[cfg(any(feature = "sparc", all(target_os = "linux", target_arch = "sparc")))]
pub mod sparc {
    //! Error numbers for arch `sparc`. Activated with feature `sparc` or with target_os `linux` and target_arch `sparc`.
    pub use super::linux::sparc::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::sparc::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "sparc"))]
pub use sparc::Errno;
#[cfg(all(target_os = "linux", target_arch = "sparc", feature = "iter"))]
pub use sparc::ErrnoIter;
#[cfg(any(feature = "sparc64", all(target_os = "linux", target_arch = "sparc64")))]
pub mod sparc64 {
    //! Error numbers for arch `sparc64`. Activated with feature `sparc64` or with target_os `linux` and target_arch `sparc64`.
    pub use super::linux::sparc::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::sparc::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "sparc64"))]
pub use sparc64::Errno;
#[cfg(all(target_os = "linux", target_arch = "sparc64", feature = "iter"))]
pub use sparc64::ErrnoIter;
#[cfg(any(feature = "x86", all(target_os = "linux", target_arch = "x86")))]
pub mod x86 {
    //! Error numbers for arch `x86`. Activated with feature `x86` or with target_os `linux` and target_arch `x86`.
    pub use super::linux::x86::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::x86::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub use x86::Errno;
#[cfg(all(target_os = "linux", target_arch = "x86", feature = "iter"))]
pub use x86::ErrnoIter;
#[cfg(any(feature = "x86_64", all(target_os = "linux", target_arch = "x86_64")))]
pub mod x86_64 {
    //! Error numbers for arch `x86_64`. Activated with feature `x86_64` or with target_os `linux` and target_arch `x86_64`.
    pub use super::linux::x86::Errno;
    #[cfg(feature = "iter")]
    pub use super::linux::x86::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use x86_64::Errno;
#[cfg(all(target_os = "linux", target_arch = "x86_64", feature = "iter"))]
pub use x86_64::ErrnoIter;
