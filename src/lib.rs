//! Error numbers for every arch supported by linux.

#![cfg_attr(all(not(doc), not(feature = "std")), no_std)]
#![cfg_attr(doc, feature(doc_cfg, doc_auto_cfg, doc_cfg_hide))]
#![cfg_attr(doc, doc(cfg_hide(doc)))]
pub mod linux;
pub(crate) mod macros;

#[cfg(any(doc, feature = "arm", all(target_os = "linux", target_arch = "arm")))]
pub mod arm {
    //! Error numbers for arch `arm`.
    pub use super::linux::arm::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::arm::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub use arm::Errno;
#[cfg(all(target_os = "linux", target_arch = "arm", any(doc, feature = "iter")))]
pub use arm::ErrnoIter;
#[cfg(any(
    doc,
    feature = "aarch64",
    all(target_os = "linux", target_arch = "aarch64")
))]
pub mod aarch64 {
    //! Error numbers for arch `aarch64`.
    pub use super::linux::arm64::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::arm64::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub use aarch64::Errno;
#[cfg(all(
    target_os = "linux",
    target_arch = "aarch64",
    any(doc, feature = "iter")
))]
pub use aarch64::ErrnoIter;
#[cfg(any(
    doc,
    feature = "hexagon",
    all(target_os = "linux", target_arch = "hexagon")
))]
pub mod hexagon {
    //! Error numbers for arch `hexagon`.
    pub use super::linux::hexagon::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::hexagon::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "hexagon"))]
pub use hexagon::Errno;
#[cfg(all(
    target_os = "linux",
    target_arch = "hexagon",
    any(doc, feature = "iter")
))]
pub use hexagon::ErrnoIter;
#[cfg(any(
    doc,
    feature = "loongarch64",
    all(target_os = "linux", target_arch = "loongarch64")
))]
pub mod loongarch64 {
    //! Error numbers for arch `loongarch64`.
    pub use super::linux::loongarch::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::loongarch::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "loongarch64"))]
pub use loongarch64::Errno;
#[cfg(all(
    target_os = "linux",
    target_arch = "loongarch64",
    any(doc, feature = "iter")
))]
pub use loongarch64::ErrnoIter;
#[cfg(any(doc, feature = "m68k", all(target_os = "linux", target_arch = "m68k")))]
pub mod m68k {
    //! Error numbers for arch `m68k`.
    pub use super::linux::m68k::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::m68k::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "m68k"))]
pub use m68k::Errno;
#[cfg(all(target_os = "linux", target_arch = "m68k", any(doc, feature = "iter")))]
pub use m68k::ErrnoIter;
#[cfg(any(doc, feature = "mips", all(target_os = "linux", target_arch = "mips")))]
pub mod mips {
    //! Error numbers for arch `mips`.
    pub use super::linux::mips::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::mips::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "mips"))]
pub use mips::Errno;
#[cfg(all(target_os = "linux", target_arch = "mips", any(doc, feature = "iter")))]
pub use mips::ErrnoIter;
#[cfg(any(
    doc,
    feature = "mips64",
    all(target_os = "linux", target_arch = "mips64")
))]
pub mod mips64 {
    //! Error numbers for arch `mips64`.
    pub use super::linux::mips::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::mips::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "mips64"))]
pub use mips64::Errno;
#[cfg(all(
    target_os = "linux",
    target_arch = "mips64",
    any(doc, feature = "iter")
))]
pub use mips64::ErrnoIter;
#[cfg(any(
    doc,
    feature = "powerpc",
    all(target_os = "linux", target_arch = "powerpc")
))]
pub mod powerpc {
    //! Error numbers for arch `powerpc`.
    pub use super::linux::powerpc::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::powerpc::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "powerpc"))]
pub use powerpc::Errno;
#[cfg(all(
    target_os = "linux",
    target_arch = "powerpc",
    any(doc, feature = "iter")
))]
pub use powerpc::ErrnoIter;
#[cfg(any(
    doc,
    feature = "powerpc64",
    all(target_os = "linux", target_arch = "powerpc64")
))]
pub mod powerpc64 {
    //! Error numbers for arch `powerpc64`.
    pub use super::linux::powerpc::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::powerpc::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "powerpc64"))]
pub use powerpc64::Errno;
#[cfg(all(
    target_os = "linux",
    target_arch = "powerpc64",
    any(doc, feature = "iter")
))]
pub use powerpc64::ErrnoIter;
#[cfg(any(
    doc,
    feature = "riscv32",
    all(target_os = "linux", target_arch = "riscv32")
))]
pub mod riscv32 {
    //! Error numbers for arch `riscv32`.
    pub use super::linux::riscv::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::riscv::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "riscv32"))]
pub use riscv32::Errno;
#[cfg(all(
    target_os = "linux",
    target_arch = "riscv32",
    any(doc, feature = "iter")
))]
pub use riscv32::ErrnoIter;
#[cfg(any(
    doc,
    feature = "riscv64",
    all(target_os = "linux", target_arch = "riscv64")
))]
pub mod riscv64 {
    //! Error numbers for arch `riscv64`.
    pub use super::linux::riscv::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::riscv::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "riscv64"))]
pub use riscv64::Errno;
#[cfg(all(
    target_os = "linux",
    target_arch = "riscv64",
    any(doc, feature = "iter")
))]
pub use riscv64::ErrnoIter;
#[cfg(any(
    doc,
    feature = "s390x",
    all(target_os = "linux", target_arch = "s390x")
))]
pub mod s390x {
    //! Error numbers for arch `s390x`.
    pub use super::linux::s390::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::s390::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "s390x"))]
pub use s390x::Errno;
#[cfg(all(target_os = "linux", target_arch = "s390x", any(doc, feature = "iter")))]
pub use s390x::ErrnoIter;
#[cfg(any(
    doc,
    feature = "sparc",
    all(target_os = "linux", target_arch = "sparc")
))]
pub mod sparc {
    //! Error numbers for arch `sparc`.
    pub use super::linux::sparc::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::sparc::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "sparc"))]
pub use sparc::Errno;
#[cfg(all(target_os = "linux", target_arch = "sparc", any(doc, feature = "iter")))]
pub use sparc::ErrnoIter;
#[cfg(any(
    doc,
    feature = "sparc64",
    all(target_os = "linux", target_arch = "sparc64")
))]
pub mod sparc64 {
    //! Error numbers for arch `sparc64`.
    pub use super::linux::sparc::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::sparc::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "sparc64"))]
pub use sparc64::Errno;
#[cfg(all(
    target_os = "linux",
    target_arch = "sparc64",
    any(doc, feature = "iter")
))]
pub use sparc64::ErrnoIter;
#[cfg(any(doc, feature = "x86", all(target_os = "linux", target_arch = "x86")))]
pub mod x86 {
    //! Error numbers for arch `x86`.
    pub use super::linux::x86::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::x86::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub use x86::Errno;
#[cfg(all(target_os = "linux", target_arch = "x86", any(doc, feature = "iter")))]
pub use x86::ErrnoIter;
#[cfg(any(
    doc,
    feature = "x86_64",
    all(target_os = "linux", target_arch = "x86_64")
))]
pub mod x86_64 {
    //! Error numbers for arch `x86_64`.
    pub use super::linux::x86::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::x86::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use x86_64::Errno;
#[cfg(all(
    target_os = "linux",
    target_arch = "x86_64",
    any(doc, feature = "iter")
))]
pub use x86_64::ErrnoIter;
