//! Error numbers contained in linux/arch/*.

#[cfg(any(
    doc,
    feature = "aarch64",
    feature = "arc",
    feature = "arm",
    feature = "arm64",
    feature = "csky",
    feature = "generic",
    feature = "hexagon",
    feature = "ia64",
    feature = "loongarch",
    feature = "loongarch64",
    feature = "m68k",
    feature = "microblaze",
    feature = "nios2",
    feature = "openrisc",
    feature = "riscv",
    feature = "riscv32",
    feature = "riscv64",
    feature = "s390",
    feature = "s390x",
    feature = "sh",
    feature = "um",
    feature = "x86",
    feature = "x86_64",
    feature = "xtensa",
    all(
        target_os = "linux",
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "hexagon",
            target_arch = "loongarch64",
            target_arch = "m68k",
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "s390x",
            target_arch = "x86",
            target_arch = "x86_64"
        )
    )
))]
pub mod generic;

#[cfg(any(doc, feature = "alpha"))]
pub mod alpha;
#[cfg(any(doc, feature = "arc"))]
pub mod arc {
    //! Error number for arch `arc`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(doc, feature = "arm", all(target_os = "linux", target_arch = "arm")))]
pub mod arm {
    //! Error number for arch `arm`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    doc,
    feature = "aarch64",
    feature = "arm64",
    all(target_os = "linux", target_arch = "aarch64")
))]
pub mod arm64 {
    //! Error number for arch `arm64`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(doc, feature = "csky"))]
pub mod csky {
    //! Error number for arch `csky`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    doc,
    feature = "hexagon",
    all(target_os = "linux", target_arch = "hexagon")
))]
pub mod hexagon {
    //! Error number for arch `hexagon`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(doc, feature = "ia64"))]
pub mod ia64 {
    //! Error number for arch `ia64`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    doc,
    feature = "loongarch",
    feature = "loongarch64",
    all(target_os = "linux", target_arch = "loongarch64")
))]
pub mod loongarch {
    //! Error number for arch `loongarch`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(doc, feature = "m68k", all(target_os = "linux", target_arch = "m68k")))]
pub mod m68k {
    //! Error number for arch `m68k`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(doc, feature = "microblaze"))]
pub mod microblaze {
    //! Error number for arch `microblaze`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    doc,
    feature = "mips",
    feature = "mips64",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64"))
))]
pub mod mips;
#[cfg(any(doc, feature = "nios2"))]
pub mod nios2 {
    //! Error number for arch `nios2`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(doc, feature = "openrisc"))]
pub mod openrisc {
    //! Error number for arch `openrisc`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(doc, feature = "parisc"))]
pub mod parisc;
#[cfg(any(
    doc,
    feature = "powerpc",
    feature = "powerpc64",
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    )
))]
pub mod powerpc;
#[cfg(any(
    doc,
    feature = "riscv",
    feature = "riscv32",
    feature = "riscv64",
    all(
        target_os = "linux",
        any(target_arch = "riscv32", target_arch = "riscv64")
    )
))]
pub mod riscv {
    //! Error number for arch `riscv`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    doc,
    feature = "s390",
    feature = "s390x",
    all(target_os = "linux", target_arch = "s390x")
))]
pub mod s390 {
    //! Error number for arch `s390`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(doc, feature = "sh"))]
pub mod sh {
    //! Error number for arch `sh`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    doc,
    feature = "sparc",
    feature = "sparc64",
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    )
))]
pub mod sparc;
#[cfg(any(doc, feature = "um"))]
pub mod um {
    //! Error number for arch `um`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    doc,
    feature = "x86",
    feature = "x86_64",
    all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))
))]
pub mod x86 {
    //! Error number for arch `x86`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(doc, feature = "xtensa"))]
pub mod xtensa {
    //! Error number for arch `xtensa`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
