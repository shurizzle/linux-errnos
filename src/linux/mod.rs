//! Error numbers contained in linux/arch/*.

#[cfg(any(
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

#[cfg(feature = "alpha")]
pub mod alpha;
#[cfg(feature = "arc")]
pub mod arc {
    //! Error number for arch `arc`. Activated with feature `arc`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(feature = "arm", all(target_os = "linux", target_arch = "arm")))]
pub mod arm {
    //! Error number for arch `arm`. Activated with feature `arm` or with target_os `linux` and target_arch `arm`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    feature = "aarch64",
    feature = "arm64",
    all(target_os = "linux", target_arch = "aarch64")
))]
pub mod arm64 {
    //! Error number for arch `arm64`. Activated with features `aarch64` and `arm64` or with target_os `linux` and target_arch `aarch64`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(feature = "csky")]
pub mod csky {
    //! Error number for arch `csky`. Activated with feature `csky`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(feature = "hexagon", all(target_os = "linux", target_arch = "hexagon")))]
pub mod hexagon {
    //! Error number for arch `hexagon`. Activated with feature `hexagon` or with target_os `linux` and target_arch `hexagon`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(feature = "ia64")]
pub mod ia64 {
    //! Error number for arch `ia64`. Activated with feature `ia64`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    feature = "loongarch",
    feature = "loongarch64",
    all(target_os = "linux", target_arch = "loongarch64")
))]
pub mod loongarch {
    //! Error number for arch `loongarch`. Activated with features `loongarch` and `loongarch64` or with target_os `linux` and target_arch `loongarch64`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(feature = "m68k", all(target_os = "linux", target_arch = "m68k")))]
pub mod m68k {
    //! Error number for arch `m68k`. Activated with feature `m68k` or with target_os `linux` and target_arch `m68k`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(feature = "microblaze")]
pub mod microblaze {
    //! Error number for arch `microblaze`. Activated with feature `microblaze`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    feature = "mips",
    feature = "mips64",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64"))
))]
pub mod mips;
#[cfg(feature = "nios2")]
pub mod nios2 {
    //! Error number for arch `nios2`. Activated with feature `nios2`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(feature = "openrisc")]
pub mod openrisc {
    //! Error number for arch `openrisc`. Activated with feature `openrisc`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(feature = "parisc")]
pub mod parisc;
#[cfg(any(
    feature = "powerpc",
    feature = "powerpc64",
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    )
))]
pub mod powerpc;
#[cfg(any(
    feature = "riscv",
    feature = "riscv32",
    feature = "riscv64",
    all(
        target_os = "linux",
        any(target_arch = "riscv32", target_arch = "riscv64")
    )
))]
pub mod riscv {
    //! Error number for arch `riscv`. Activated with features `riscv`, `riscv32` and `riscv64` or with target_os `linux` and target_arch `riscv32` and `riscv64`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    feature = "s390",
    feature = "s390x",
    all(target_os = "linux", target_arch = "s390x")
))]
pub mod s390 {
    //! Error number for arch `s390`. Activated with features `s390` and `s390x` or with target_os `linux` and target_arch `s390x`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(feature = "sh")]
pub mod sh {
    //! Error number for arch `sh`. Activated with feature `sh`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    feature = "sparc",
    feature = "sparc64",
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    )
))]
pub mod sparc;
#[cfg(feature = "um")]
pub mod um {
    //! Error number for arch `um`. Activated with feature `um`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(any(
    feature = "x86",
    feature = "x86_64",
    all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))
))]
pub mod x86 {
    //! Error number for arch `x86`. Activated with features `x86` and `x86_64` or with target_os `linux` and target_arch `x86` and `x86_64`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
#[cfg(feature = "xtensa")]
pub mod xtensa {
    //! Error number for arch `xtensa`. Activated with feature `xtensa`.
    pub use super::generic::Errno;
    #[cfg(feature = "iter")]
    pub use super::generic::ErrnoIter;
}
