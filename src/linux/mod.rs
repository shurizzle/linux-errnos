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
    pub use super::generic::*;
}
#[cfg(any(feature = "arm", all(target_os = "linux", target_arch = "arm")))]
pub mod arm {
    pub use super::generic::*;
}
#[cfg(any(
    feature = "aarch64",
    feature = "arm64",
    all(target_os = "linux", target_arch = "aarch64")
))]
pub mod arm64 {
    pub use super::generic::*;
}
#[cfg(feature = "csky")]
pub mod csky {
    pub use super::generic::*;
}
#[cfg(any(feature = "hexagon", all(target_os = "linux", target_arch = "hexagon")))]
pub mod hexagon {
    pub use super::generic::*;
}
#[cfg(feature = "ia64")]
pub mod ia64 {
    pub use super::generic::*;
}
#[cfg(feature = "loongarch")]
pub mod loongarch {
    pub use super::generic::*;
}
#[cfg(any(feature = "m68k", all(target_os = "linux", target_arch = "m68k")))]
pub mod m68k {
    pub use super::generic::*;
}
#[cfg(feature = "microblaze")]
pub mod microblaze {
    pub use super::generic::*;
}
#[cfg(any(
    feature = "mips",
    feature = "mips64",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64"))
))]
pub mod mips;
#[cfg(feature = "nios2")]
pub mod nios2 {
    pub use super::generic::*;
}
#[cfg(feature = "openrisc")]
pub mod openrisc {
    pub use super::generic::*;
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
    pub use super::generic::*;
}
#[cfg(any(
    feature = "s390",
    feature = "s390x",
    all(target_os = "linux", target_arch = "s390x")
))]
pub mod s390 {
    pub use super::generic::*;
}
#[cfg(feature = "sh")]
pub mod sh {
    pub use super::generic::*;
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
    pub use super::generic::*;
}
#[cfg(any(
    feature = "x86",
    feature = "x86_64",
    all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))
))]
pub mod x86 {
    pub use super::generic::*;
}
#[cfg(feature = "xtensa")]
pub mod xtensa {
    pub use super::generic::*;
}
