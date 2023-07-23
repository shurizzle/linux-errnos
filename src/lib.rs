//! Error numbers for every arch supported by linux..

#![cfg_attr(all(not(doc), not(feature = "std")), no_std)]
#![cfg_attr(doc, feature(doc_cfg, doc_auto_cfg, doc_cfg_hide))]
#![cfg_attr(doc, doc(cfg_hide(doc)))]
pub mod linux;
pub(crate) mod macros;

#[cfg(any(
    all(any(target_os = "linux", target_os = "android"), target_arch = "x86"),
    feature = "x86",
    doc
))]
pub mod x86 {
    //! Error numbers for arch `x86`.
    pub use super::linux::x86::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::x86::ErrnoIter;
}
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "x86"))]
pub use x86::Errno;
#[cfg(all(
    any(target_os = "linux", target_os = "android"),
    any(feature = "iter", doc),
    target_arch = "x86"
))]
pub use x86::ErrnoIter;

#[cfg(any(
    all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "x86_64"
    ),
    feature = "x86_64",
    doc
))]
pub mod x86_64 {
    //! Error numbers for arch `x86_64`.
    pub use super::linux::x86::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::x86::ErrnoIter;
}
#[cfg(all(
    any(target_os = "linux", target_os = "android"),
    target_arch = "x86_64"
))]
pub use x86_64::Errno;
#[cfg(all(
    any(target_os = "linux", target_os = "android"),
    any(feature = "iter", doc),
    target_arch = "x86_64"
))]
pub use x86_64::ErrnoIter;

#[cfg(any(
    all(any(target_os = "linux", target_os = "android"), target_arch = "arm"),
    feature = "arm",
    doc
))]
pub mod arm {
    //! Error numbers for arch `arm`.
    pub use super::linux::arm::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::arm::ErrnoIter;
}
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "arm"))]
pub use arm::Errno;
#[cfg(all(
    any(target_os = "linux", target_os = "android"),
    any(feature = "iter", doc),
    target_arch = "arm"
))]
pub use arm::ErrnoIter;

#[cfg(any(
    all(
        any(target_os = "linux", target_os = "android"),
        target_arch = "aarch64"
    ),
    feature = "aarch64",
    doc
))]
pub mod aarch64 {
    //! Error numbers for arch `aarch64`.
    pub use super::linux::arm64::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::arm64::ErrnoIter;
}
#[cfg(all(
    any(target_os = "linux", target_os = "android"),
    target_arch = "aarch64"
))]
pub use aarch64::Errno;
#[cfg(all(
    any(target_os = "linux", target_os = "android"),
    any(feature = "iter", doc),
    target_arch = "aarch64"
))]
pub use aarch64::ErrnoIter;

#[cfg(any(
    all(target_os = "linux", target_arch = "hexagon"),
    feature = "hexagon",
    doc
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
    any(feature = "iter", doc),
    target_os = "linux",
    target_arch = "hexagon"
))]
pub use hexagon::ErrnoIter;

#[cfg(any(
    all(target_os = "linux", target_arch = "s390x"),
    feature = "s390x",
    doc
))]
pub mod s390x {
    //! Error numbers for arch `s390x`.
    pub use super::linux::s390::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::s390::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "s390x"))]
pub use s390x::Errno;
#[cfg(all(any(feature = "iter", doc), target_os = "linux", target_arch = "s390x"))]
pub use s390x::ErrnoIter;

#[cfg(any(
    all(target_os = "linux", target_arch = "powerpc"),
    feature = "powerpc",
    doc
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
    any(feature = "iter", doc),
    target_os = "linux",
    target_arch = "powerpc"
))]
pub use powerpc::ErrnoIter;

#[cfg(any(
    all(target_os = "linux", target_arch = "powerpc64"),
    feature = "powerpc64",
    doc
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
    any(feature = "iter", doc),
    target_os = "linux",
    target_arch = "powerpc64"
))]
pub use powerpc64::ErrnoIter;

#[cfg(any(all(target_os = "linux", target_arch = "mips"), feature = "mips", doc))]
pub mod mips {
    //! Error numbers for arch `mips`.
    pub use super::linux::mips::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::mips::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "mips"))]
pub use mips::Errno;
#[cfg(all(any(feature = "iter", doc), target_os = "linux", target_arch = "mips"))]
pub use mips::ErrnoIter;

#[cfg(any(
    all(target_os = "linux", target_arch = "mips64"),
    feature = "mips64",
    doc
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
    any(feature = "iter", doc),
    target_os = "linux",
    target_arch = "mips64"
))]
pub use mips64::ErrnoIter;

#[cfg(any(all(target_os = "linux", target_arch = "m68k"), feature = "m68k", doc))]
pub mod m68k {
    //! Error numbers for arch `m68k`.
    pub use super::linux::m68k::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::m68k::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "m68k"))]
pub use m68k::Errno;
#[cfg(all(any(feature = "iter", doc), target_os = "linux", target_arch = "m68k"))]
pub use m68k::ErrnoIter;

#[cfg(any(
    all(target_os = "linux", target_arch = "riscv32"),
    feature = "riscv32",
    doc
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
    any(feature = "iter", doc),
    target_os = "linux",
    target_arch = "riscv32"
))]
pub use riscv32::ErrnoIter;

#[cfg(any(
    all(target_os = "linux", target_arch = "riscv64"),
    feature = "riscv64",
    doc
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
    any(feature = "iter", doc),
    target_os = "linux",
    target_arch = "riscv64"
))]
pub use riscv64::ErrnoIter;

#[cfg(any(
    all(target_os = "linux", target_arch = "sparc"),
    feature = "sparc",
    doc
))]
pub mod sparc {
    //! Error numbers for arch `sparc`.
    pub use super::linux::sparc::Errno;
    #[cfg(any(doc, feature = "iter"))]
    pub use super::linux::sparc::ErrnoIter;
}
#[cfg(all(target_os = "linux", target_arch = "sparc"))]
pub use sparc::Errno;
#[cfg(all(any(feature = "iter", doc), target_os = "linux", target_arch = "sparc"))]
pub use sparc::ErrnoIter;

#[cfg(any(
    all(target_os = "linux", target_arch = "sparc64"),
    feature = "sparc64",
    doc
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
    any(feature = "iter", doc),
    target_os = "linux",
    target_arch = "sparc64"
))]
pub use sparc64::ErrnoIter;

#[cfg(any(
    all(target_os = "linux", target_arch = "loongarch64"),
    feature = "loongarch64",
    doc
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
    any(feature = "iter", doc),
    target_os = "linux",
    target_arch = "loongarch64"
))]
pub use loongarch64::ErrnoIter;

#[cfg(all(
    any(
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
                target_arch = "powerpc",
                target_arch = "powerpc64",
                target_arch = "mips",
                target_arch = "mips64",
                target_arch = "m68k",
                target_arch = "riscv32",
                target_arch = "riscv64",
                target_arch = "sparc",
                target_arch = "sparc64",
                target_arch = "loongarch64"
            ),
            target_os = "linux"
        )
    ),
    any(feature = "libc-compat", doc)
))]
#[link(name = "c")]
extern "C" {
    #[cfg_attr(target_os = "linux", link_name = "__errno_location")]
    #[cfg_attr(target_os = "android", link_name = "__errno")]
    fn errno() -> *mut i32;
}

#[cfg(all(
    any(
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
                target_arch = "powerpc",
                target_arch = "powerpc64",
                target_arch = "mips",
                target_arch = "mips64",
                target_arch = "m68k",
                target_arch = "riscv32",
                target_arch = "riscv64",
                target_arch = "sparc",
                target_arch = "sparc64",
                target_arch = "loongarch64"
            ),
            target_os = "linux"
        )
    ),
    any(feature = "libc-compat", doc)
))]
impl Errno {
    /// Returns a new `Errno` from last OS error.
    pub fn last_os_error() -> Self {
        Self(unsafe { *errno() })
    }
}

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
            target_arch = "powerpc",
            target_arch = "powerpc64",
            target_arch = "mips",
            target_arch = "mips64",
            target_arch = "m68k",
            target_arch = "riscv32",
            target_arch = "riscv64",
            target_arch = "sparc",
            target_arch = "sparc64",
            target_arch = "loongarch64"
        ),
        target_os = "linux"
    )
))]
#[test]
fn basic() {
    #[cfg(features = "libc-compat")]
    {
        _ = Errno::last_os_error();
    }
    _ = Errno::EINVAL;
}
