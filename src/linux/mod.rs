//! Error numbers contained in linux/arch/*..

#[cfg(any(
    all(
        any(target_os = "linux", target_os = "android"),
        any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "arm",
            target_arch = "aarch64"
        )
    ),
    all(
        any(
            target_arch = "hexagon",
            target_arch = "s390x",
            target_arch = "m68k",
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "loongarch64"
        ),
        target_os = "linux"
    ),
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
    doc
))]
pub mod generic;

#[cfg(any(feature = "alpha", doc))]
pub mod alpha;

#[cfg(any(feature = "arc", doc))]
pub mod arc {
    //! Error numbers for arch `arc`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(
    all(any(target_os = "linux", target_os = "android"), target_arch = "arm"),
    feature = "arm",
    doc
))]
pub mod arm {
    //! Error numbers for arch `arm`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(
    all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "aarch64"
    ),
    feature = "aarch64",
    feature = "arm64",
    doc
))]
pub mod arm64 {
    //! Error numbers for arch `arm64`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(feature = "csky", doc))]
pub mod csky {
    //! Error numbers for arch `csky`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(
    all(target_os = "linux", target_arch = "hexagon"),
    feature = "hexagon",
    doc
))]
pub mod hexagon {
    //! Error numbers for arch `hexagon`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(feature = "ia64", doc))]
pub mod ia64 {
    //! Error numbers for arch `ia64`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(
    all(target_os = "linux", target_arch = "loongarch64"),
    feature = "loongarch",
    feature = "loongarch64",
    doc
))]
pub mod loongarch {
    //! Error numbers for arch `loongarch`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(all(target_os = "linux", target_arch = "m68k"), feature = "m68k", doc))]
pub mod m68k {
    //! Error numbers for arch `m68k`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(feature = "microblaze", doc))]
pub mod microblaze {
    //! Error numbers for arch `microblaze`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(
    all(any(target_arch = "mips", target_arch = "mips64"), target_os = "linux"),
    feature = "mips",
    feature = "mips64",
    doc
))]
pub mod mips;

#[cfg(any(feature = "nios2", doc))]
pub mod nios2 {
    //! Error numbers for arch `nios2`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(feature = "openrisc", doc))]
pub mod openrisc {
    //! Error numbers for arch `openrisc`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(feature = "parisc", doc))]
pub mod parisc;

#[cfg(any(
    all(
        any(target_arch = "powerpc", target_arch = "powerpc64"),
        target_os = "linux"
    ),
    feature = "powerpc",
    feature = "powerpc64",
    doc
))]
pub mod powerpc;

#[cfg(any(
    all(
        any(target_arch = "riscv32", target_arch = "riscv64"),
        target_os = "linux"
    ),
    feature = "riscv",
    feature = "riscv32",
    feature = "riscv64",
    doc
))]
pub mod riscv {
    //! Error numbers for arch `riscv`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(
    all(target_os = "linux", target_arch = "s390x"),
    feature = "s390",
    feature = "s390x",
    doc
))]
pub mod s390 {
    //! Error numbers for arch `s390`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(feature = "sh", doc))]
pub mod sh {
    //! Error numbers for arch `sh`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(
    all(
        any(target_arch = "sparc", target_arch = "sparc64"),
        target_os = "linux"
    ),
    feature = "sparc",
    feature = "sparc64",
    doc
))]
pub mod sparc;

#[cfg(any(feature = "um", doc))]
pub mod um {
    //! Error numbers for arch `um`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(
    all(
        any(target_os = "linux", target_os = "android"),
        any(target_arch = "x86", target_arch = "x86_64")
    ),
    feature = "x86",
    feature = "x86_64",
    doc
))]
pub mod x86 {
    //! Error numbers for arch `x86`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}

#[cfg(any(feature = "xtensa", doc))]
pub mod xtensa {
    //! Error numbers for arch `xtensa`.
    pub use super::generic::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::generic::ErrnoIter;
}
