use core::fmt;
use std::{collections::HashMap, rc::Rc};

use crate::generate::Id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Platform {
    X86,
    X86_64,
    Arm,
    AArch64,
    Hexagon,
    S390x,
    Powerpc,
    Powerpc64,
    Mips,
    Mips64,
    M68k,
    Riscv32,
    Riscv64,
    Sparc,
    Sparc64,
    LoongArch64,
}

impl Platform {
    pub fn linux_arch(&self) -> &str {
        match self {
            Self::X86 | Self::X86_64 => "x86",
            Self::Arm => "arm",
            Self::AArch64 => "arm64",
            Self::Hexagon => "hexagon",
            Self::S390x => "s390",
            Self::Powerpc | Self::Powerpc64 => "powerpc",
            Self::Mips | Self::Mips64 => "mips",
            Self::M68k => "m68k",
            Self::Riscv32 | Self::Riscv64 => "riscv",
            Self::Sparc | Self::Sparc64 => "sparc",
            Self::LoongArch64 => "loongarch",
        }
    }

    pub fn rust_arch(&self) -> &str {
        match self {
            Self::X86 => "x86",
            Self::X86_64 => "x86_64",
            Self::Arm => "arm",
            Self::AArch64 => "aarch64",
            Self::Hexagon => "hexagon",
            Self::S390x => "s390x",
            Self::Powerpc => "powerpc",
            Self::Powerpc64 => "powerpc64",
            Self::Mips => "mips",
            Self::Mips64 => "mips64",
            Self::M68k => "m68k",
            Self::Riscv32 => "riscv32",
            Self::Riscv64 => "riscv64",
            Self::Sparc => "sparc",
            Self::Sparc64 => "sparc64",
            Self::LoongArch64 => "loongarch64",
        }
    }

    pub fn is_android(&self) -> bool {
        matches!(self, Self::X86 | Self::X86_64 | Self::Arm | Self::AArch64)
    }

    pub fn os_condition(&self) -> Condition {
        let mut cond = Condition::target_os(Os::Linux);
        if self.is_android() {
            cond = cond.or(Condition::target_os(Os::Android));
        }
        cond
    }

    pub fn arch_condition(&self) -> Condition {
        self.os_condition()
            .and(Condition::target_arch((*self).into()))
    }

    pub fn mod_condition(&self) -> Condition {
        self.arch_condition()
            .or(Condition::feature(Rc::from(self.rust_arch())))
            .or(Condition::doc())
    }

    pub fn add_to_cfg<C: CfgBuild>(&self, cfg: &mut C) {
        cfg.add_platform(Os::Linux, Condition::target_arch((*self).into()));
        if self.is_android() {
            cfg.add_platform(Os::Android, Condition::target_arch((*self).into()));
        }
        cfg.add_feature(Rc::from(self.rust_arch()))
    }

    #[inline]
    pub fn iter() -> PlatformIterator {
        PlatformIterator::new()
    }
}

impl From<Platform> for Arch {
    fn from(value: Platform) -> Self {
        match value {
            Platform::X86 => Arch::X86,
            Platform::X86_64 => Arch::X86_64,
            Platform::Arm => Arch::Arm,
            Platform::AArch64 => Arch::AArch64,
            Platform::Hexagon => Arch::Hexagon,
            Platform::S390x => Arch::S390x,
            Platform::Powerpc => Arch::Powerpc,
            Platform::Powerpc64 => Arch::Powerpc64,
            Platform::Mips => Arch::Mips,
            Platform::Mips64 => Arch::Mips64,
            Platform::M68k => Arch::M68k,
            Platform::Riscv32 => Arch::Riscv32,
            Platform::Riscv64 => Arch::Riscv64,
            Platform::Sparc => Arch::Sparc,
            Platform::Sparc64 => Arch::Sparc64,
            Platform::LoongArch64 => Arch::LoongArch64,
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#[cfg({})]", self.mod_condition())?;
        writeln!(f, "pub mod {} {{", Id(self.rust_arch()))?;
        writeln!(f, "    //! Error numbers for arch `{}`.", self.rust_arch())?;
        writeln!(
            f,
            "    pub use super::linux::{}::Errno;",
            Id(self.linux_arch())
        )?;
        writeln!(f, "    #[cfg(any(doc, feature = \"iter\"))]")?;
        writeln!(
            f,
            "    pub use super::linux::{}::ErrnoIter;",
            Id(self.linux_arch())
        )?;
        writeln!(f, "}}")?;
        let cond = self.arch_condition();
        writeln!(f, "#[cfg({})]", cond)?;
        writeln!(f, "pub use {}::Errno;", Id(self.rust_arch()))?;
        let cond = cond.and(Condition::doc().or(Condition::feature(Rc::from("iter"))));
        writeln!(f, "#[cfg({})]", cond)?;
        writeln!(f, "pub use {}::ErrnoIter;", Id(self.rust_arch()))
    }
}

pub struct Lib;

impl fmt::Display for Lib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "#![cfg_attr(all(not(doc), not(feature = \"std\")), no_std)]"
        )?;
        writeln!(
            f,
            "#![cfg_attr(doc, feature(doc_cfg, doc_auto_cfg, doc_cfg_hide))]"
        )?;
        writeln!(f, "#![cfg_attr(doc, doc(cfg_hide(doc)))]")?;
        writeln!(f, "pub mod linux;")?;
        writeln!(f, "pub(crate) mod macros;")?;

        let mut cond = PlatformBuilder::new();

        for p in Platform::iter() {
            p.add_to_cfg(&mut cond);
            writeln!(f)?;
            fmt::Display::fmt(&p, f)?;
        }

        let test_cond = cond.build();
        let cond = test_cond.as_ref().map(|cond| {
            cond.clone()
                .and(Condition::feature(Rc::from("libc-compat")).or(Condition::doc()))
        });

        writeln!(f)?;
        if let Some(cond) = cond.as_ref() {
            writeln!(f, "#[cfg({})]", cond)?;
        }
        writeln!(f, "#[link(name = \"c\")]")?;
        writeln!(f, "extern \"C\" {{")?;
        writeln!(
            f,
            "    #[cfg_attr(target_os = \"linux\", link_name = \"__errno_location\")]"
        )?;
        writeln!(
            f,
            "    #[cfg_attr(target_os = \"android\", link_name = \"__errno\")]"
        )?;
        writeln!(f, "    fn errno() -> *mut i32;")?;
        writeln!(f, "}}")?;

        writeln!(f)?;
        if let Some(cond) = cond.as_ref() {
            writeln!(f, "#[cfg({})]", cond)?;
        }
        writeln!(f, "impl Errno {{")?;
        writeln!(f, "    /// Returns a new `Errno` from last OS error.")?;
        writeln!(f, "    pub fn last_os_error() -> Self {{")?;
        writeln!(f, "        Self(unsafe {{ *errno() }})")?;
        writeln!(f, "    }}")?;
        writeln!(f, "}}")?;

        if let Some(cond) = test_cond.as_ref() {
            writeln!(f)?;
            writeln!(f, "#[cfg({})]", cond)?;
            writeln!(f, "#[test]")?;
            writeln!(f, "fn basic() {{")?;
            writeln!(f, "    #[cfg(feature = \"libc-compat\")]")?;
            writeln!(f, "    {{")?;
            writeln!(f, "        _ = Errno::last_os_error();")?;
            writeln!(f, "    }}")?;
            writeln!(f, "    _ = Errno::EINVAL;")?;
            writeln!(f, "}}")?;
        }

        Ok(())
    }
}

pub struct PlatformIterator(core::iter::Copied<core::slice::Iter<'static, Platform>>);

impl PlatformIterator {
    const ALL: &[Platform] = [
        Platform::X86,
        Platform::X86_64,
        Platform::Arm,
        Platform::AArch64,
        Platform::Hexagon,
        Platform::S390x,
        Platform::Powerpc,
        Platform::Powerpc64,
        Platform::Mips,
        Platform::Mips64,
        Platform::M68k,
        Platform::Riscv32,
        Platform::Riscv64,
        Platform::Sparc,
        Platform::Sparc64,
        Platform::LoongArch64,
    ]
    .as_slice();

    #[inline]
    fn new() -> Self {
        Self(Self::ALL.iter().copied())
    }
}

impl Iterator for PlatformIterator {
    type Item = Platform;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.0.last()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth(n)
    }

    #[inline]
    fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item),
    {
        self.0.for_each(f)
    }

    #[inline]
    fn collect<B: FromIterator<Self::Item>>(self) -> B {
        self.0.collect()
    }

    #[inline]
    fn partition<B, F>(self, f: F) -> (B, B)
    where
        B: Default + Extend<Self::Item>,
        F: FnMut(&Self::Item) -> bool,
    {
        self.0.partition(f)
    }

    #[inline]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.0.fold(init, f)
    }

    #[inline]
    fn reduce<F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        self.0.reduce(f)
    }

    #[inline]
    fn all<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        self.0.all(f)
    }

    #[inline]
    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        self.0.any(f)
    }

    #[inline]
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.0.find(predicate)
    }

    #[inline]
    fn find_map<B, F>(&mut self, f: F) -> Option<B>
    where
        F: FnMut(Self::Item) -> Option<B>,
    {
        self.0.find_map(f)
    }

    #[inline]
    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.0.position(predicate)
    }

    #[inline]
    fn rposition<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        self.0.rposition(predicate)
    }

    #[inline]
    fn max(self) -> Option<Self::Item> {
        self.0.max()
    }

    #[inline]
    fn min(self) -> Option<Self::Item> {
        self.0.min()
    }

    #[inline]
    fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> B,
    {
        self.0.max_by_key(f)
    }

    #[inline]
    fn max_by<F>(self, compare: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering,
    {
        self.0.max_by(compare)
    }

    #[inline]
    fn min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> B,
    {
        self.0.min_by_key(f)
    }

    #[inline]
    fn min_by<F>(self, compare: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering,
    {
        self.0.min_by(compare)
    }

    #[inline]
    fn cmp<I>(self, other: I) -> std::cmp::Ordering
    where
        I: IntoIterator<Item = Self::Item>,
    {
        self.0.cmp(other)
    }

    #[inline]
    fn partial_cmp<I>(self, other: I) -> Option<std::cmp::Ordering>
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
    {
        self.0.partial_cmp(other)
    }

    #[inline]
    fn eq<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<I::Item>,
    {
        self.0.eq(other)
    }

    #[inline]
    fn ne<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<I::Item>,
    {
        self.0.ne(other)
    }

    #[inline]
    fn lt<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
    {
        self.0.lt(other)
    }

    #[inline]
    fn le<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
    {
        self.0.le(other)
    }

    #[inline]
    fn gt<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
    {
        self.0.gt(other)
    }

    #[inline]
    fn ge<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialOrd<I::Item>,
    {
        self.0.ge(other)
    }
}

impl ExactSizeIterator for PlatformIterator {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl DoubleEndedIterator for PlatformIterator {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.0.nth_back(n)
    }

    #[inline]
    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.0.rfold(init, f)
    }

    #[inline]
    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        self.0.rfind(predicate)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Os {
    Linux,
    Android,
}

impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::Linux => "linux",
            Self::Android => "android",
        };
        write!(f, "{:?}", repr)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Arch {
    X86,
    X86_64,
    Arm,
    AArch64,
    Hexagon,
    S390x,
    Powerpc,
    Powerpc64,
    Mips,
    Mips64,
    M68k,
    Riscv32,
    Riscv64,
    Sparc,
    Sparc64,
    LoongArch64,
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Self::X86 => "x86",
            Self::X86_64 => "x86_64",
            Self::Arm => "arm",
            Self::AArch64 => "aarch64",
            Self::Hexagon => "hexagon",
            Self::S390x => "s390x",
            Self::Powerpc => "powerpc",
            Self::Powerpc64 => "powerpc64",
            Self::Mips => "mips",
            Self::Mips64 => "mips64",
            Self::M68k => "m68k",
            Self::Riscv32 => "riscv32",
            Self::Riscv64 => "riscv64",
            Self::Sparc => "sparc",
            Self::Sparc64 => "sparc64",
            Self::LoongArch64 => "loongarch64",
        };
        write!(f, "{:?}", repr)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ConditionRepr {
    All(Vec<Self>),
    Any(Vec<Self>),
    TargetOs(Os),
    TargetArch(Arch),
    Feature(Rc<str>),
    Doc,
}

impl fmt::Display for ConditionRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = match self {
            Self::All(v) => {
                if !v.is_empty() {
                    write!(f, "all(")?;
                }
                v
            }
            Self::Any(v) => {
                if !v.is_empty() {
                    write!(f, "any(")?;
                }
                v
            }
            Self::TargetOs(os) => return write!(f, "target_os = {}", os),
            Self::TargetArch(arch) => return write!(f, "target_arch = {}", arch),
            Self::Feature(name) => return write!(f, "feature = {:?}", Rc::as_ref(name)),
            Self::Doc => return write!(f, "doc"),
        };

        let mut it = v.iter();

        if let Some(cond) = it.next() {
            fmt::Display::fmt(cond, f)?;

            for cond in it {
                write!(f, ", {}", cond)?;
            }

            write!(f, ")")?;
        }

        Ok(())
    }
}

impl ConditionRepr {
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::Any(mut v1), Self::Any(v2)) => {
                v1.extend(v2);
                v1.sort();
                v1.dedup();
                Self::Any(v1)
            }
            (Self::Any(mut v), other) => {
                v.push(other);
                v.sort();
                v.dedup();
                Self::Any(v)
            }
            (other, Self::Any(mut v)) => {
                v.insert(0, other);
                v.sort();
                v.dedup();
                Self::Any(v)
            }
            (a, b) if a == b => a,
            (a, b) => {
                let mut v = vec![a, b];
                v.sort();
                Self::Any(v)
            }
        }
    }

    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::All(mut v1), Self::All(v2)) => {
                v1.extend(v2);
                v1.sort();
                v1.dedup();
                Self::All(v1)
            }
            (Self::All(mut v), other) => {
                v.push(other);
                v.sort();
                v.dedup();
                Self::All(v)
            }
            (other, Self::All(mut v)) => {
                v.insert(0, other);
                v.sort();
                v.dedup();
                Self::All(v)
            }
            (a, b) if a == b => a,
            (a, b) => {
                let mut v = vec![a, b];
                v.sort();
                Self::All(v)
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Condition(ConditionRepr);

impl Condition {
    pub fn doc() -> Self {
        Self(ConditionRepr::Doc)
    }

    pub fn target_os(os: Os) -> Self {
        Self(ConditionRepr::TargetOs(os))
    }

    pub fn target_arch(arch: Arch) -> Self {
        Self(ConditionRepr::TargetArch(arch))
    }

    pub fn feature(name: Rc<str>) -> Self {
        Self(ConditionRepr::Feature(name))
    }

    pub fn or(self, other: Condition) -> Self {
        Self(self.0.or(other.0))
    }

    pub fn and(self, other: Condition) -> Self {
        Self(self.0.and(other.0))
    }
}

impl fmt::Debug for Condition {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Condition {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub trait CfgBuild {
    fn add_feature(&mut self, name: Rc<str>);

    fn add_platform(&mut self, os: Os, cond: Condition);
}

pub struct PlatformBuilder(HashMap<Os, Vec<Condition>>);

impl PlatformBuilder {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn build(self) -> Option<Condition> {
        fn build_condition<I: IntoIterator<Item = Condition>>(it: I) -> Option<Condition> {
            let mut it = it.into_iter();
            it.next().map(|init| it.fold(init, Condition::or))
        }

        let Self(mut platforms) = self;

        let linux = platforms.remove(&Os::Linux);
        let android = platforms.remove(&Os::Android);

        let (linux, android, both) = match (linux, android) {
            (Some(mut linux), Some(mut android)) => {
                let mut i = 0;
                let mut both = Vec::new();
                while i < linux.len() {
                    let i2 = {
                        let l = linux.get(i).unwrap();
                        match android.binary_search(l) {
                            Ok(i2) => Some(i2),
                            Err(_) => None,
                        }
                    };

                    if let Some(i2) = i2 {
                        let e = linux.remove(i);
                        _ = android.remove(i2);
                        if let Err(pos) = both.binary_search(&e) {
                            both.insert(pos, e);
                        }
                    } else {
                        i += 1;
                    }
                }

                let linux = if linux.is_empty() { None } else { Some(linux) };
                let android = if android.is_empty() {
                    None
                } else {
                    Some(android)
                };
                let both = if both.is_empty() { None } else { Some(both) };

                (linux, android, both)
            }
            (l, a) => (l, a, None),
        };

        let mut cond = linux
            .and_then(build_condition)
            .map(|linux| Condition::target_os(Os::Linux).and(linux));

        if let Some(android) = android.and_then(build_condition) {
            let android = Condition::target_os(Os::Android).and(android);
            cond = if let Some(cond) = cond {
                Some(cond.or(android))
            } else {
                Some(android)
            };
        }

        if let Some(both) = both.and_then(build_condition) {
            let both = Condition::target_os(Os::Linux)
                .or(Condition::target_os(Os::Android))
                .and(both);
            cond = if let Some(cond) = cond {
                Some(cond.or(both))
            } else {
                Some(both)
            };
        }

        cond
    }
}

impl CfgBuild for PlatformBuilder {
    fn add_platform(&mut self, os: Os, cond: Condition) {
        let conds = self.0.entry(os).or_insert(Vec::new());
        if let Err(pos) = conds.binary_search(&cond) {
            conds.insert(pos, cond);
        }
    }

    fn add_feature(&mut self, _name: Rc<str>) {}
}

enum RawLinuxModType {
    Generic,
    Original,
}

struct RawLinuxMod {
    name: Rc<str>,
    features: Vec<Rc<str>>,
    platforms: PlatformBuilder,
    r#type: RawLinuxModType,
}

enum LinuxModDecl {
    Original(Option<Condition>, Rc<str>),
    Generic(Option<Condition>, Rc<str>),
}

pub struct LinuxMod(Vec<LinuxModDecl>);

impl CfgBuild for RawLinuxMod {
    fn add_platform(&mut self, os: Os, cond: Condition) {
        self.platforms.add_platform(os, cond)
    }

    fn add_feature(&mut self, name: Rc<str>) {
        if let Err(pos) = self.features.binary_search(&name) {
            self.features.insert(pos, name);
        }
    }
}

impl RawLinuxMod {
    #[inline]
    pub fn generic(name: Rc<str>) -> Self {
        Self {
            name: name.clone(),
            features: vec![name],
            platforms: PlatformBuilder::new(),
            r#type: RawLinuxModType::Generic,
        }
    }

    #[inline]
    pub fn original(name: Rc<str>) -> Self {
        Self {
            name: name.clone(),
            features: vec![name],
            platforms: PlatformBuilder::new(),
            r#type: RawLinuxModType::Original,
        }
    }

    pub fn build(self) -> LinuxModDecl {
        fn build_condition<I: IntoIterator<Item = Condition>>(it: I) -> Option<Condition> {
            let mut it = it.into_iter();
            it.next().map(|init| it.fold(init, Condition::or))
        }

        let Self {
            name,
            features,
            platforms,
            r#type,
        } = self;
        let mut cond = build_condition(features.into_iter().map(Condition::feature));

        cond = if let Some(cond) = cond {
            Some(if let Some(p) = platforms.build() {
                cond.or(p)
            } else {
                cond
            })
        } else {
            platforms.build()
        };

        let cond = cond.map(|cond| cond.or(Condition::doc()));

        match r#type {
            RawLinuxModType::Generic => LinuxModDecl::Generic(cond, name),
            RawLinuxModType::Original => LinuxModDecl::Original(cond, name),
        }
    }
}

impl fmt::Display for LinuxModDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinuxModDecl::Original(cond, name) => {
                if let Some(cond) = cond.as_ref() {
                    write!(f, "#[cfg(")?;
                    fmt::Display::fmt(cond, f)?;
                    writeln!(f, ")]")?;
                }
                writeln!(f, "pub mod {};", Id(name))
            }
            LinuxModDecl::Generic(cond, name) => {
                if let Some(cond) = cond.as_ref() {
                    write!(f, "#[cfg(")?;
                    fmt::Display::fmt(cond, f)?;
                    writeln!(f, ")]")?;
                }

                writeln!(f, "pub mod {} {{", Id(name))?;
                writeln!(f, "    //! Error numbers for arch `{}`.", name)?;
                writeln!(f, "    pub use super::generic::Errno;")?;
                writeln!(f, "    #[cfg(any(doc, feature = \"iter\"))]")?;
                writeln!(f, "    pub use super::generic::ErrnoIter;")?;
                writeln!(f, "}}")
            }
        }
    }
}

impl fmt::Display for LinuxMod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self.0.iter();

        if let Some(m) = it.next() {
            fmt::Display::fmt(m, f)?;

            for m in it {
                writeln!(f)?;
                fmt::Display::fmt(m, f)?;
            }
        }
        Ok(())
    }
}

pub struct LinuxModBuilder {
    generic: RawLinuxMod,
    other: HashMap<Rc<str>, RawLinuxMod>,
}

enum LinuxModMutRepr<'a> {
    Original(&'a mut RawLinuxMod),
    Generic(&'a mut RawLinuxMod, &'a mut RawLinuxMod),
}

pub struct LinuxModMut<'a>(LinuxModMutRepr<'a>);

impl<'a> LinuxModMut<'a> {
    pub fn name(&self) -> Rc<str> {
        match self.0 {
            LinuxModMutRepr::Original(ref m) => m.name.clone(),
            LinuxModMutRepr::Generic(_, ref m) => m.name.clone(),
        }
    }
}

impl<'a> CfgBuild for LinuxModMutRepr<'a> {
    fn add_platform(&mut self, os: Os, cond: Condition) {
        match self {
            Self::Original(m) => m.add_platform(os, cond),
            Self::Generic(g, m) => {
                g.add_platform(os, cond.clone());
                m.add_platform(os, cond);
            }
        }
    }

    fn add_feature(&mut self, name: Rc<str>) {
        match self {
            Self::Original(m) => m.add_feature(name),
            Self::Generic(g, m) => {
                g.add_feature(name.clone());
                m.add_feature(name.clone());
            }
        }
    }
}

impl<'a> CfgBuild for LinuxModMut<'a> {
    #[inline]
    fn add_platform(&mut self, os: Os, cond: Condition) {
        self.0.add_platform(os, cond)
    }

    #[inline]
    fn add_feature(&mut self, name: Rc<str>) {
        self.0.add_feature(name)
    }
}

impl LinuxModBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let g: Rc<str> = Rc::from("generic");
        let generic = RawLinuxMod {
            name: g.clone(),
            features: vec![g],
            platforms: PlatformBuilder::new(),
            r#type: RawLinuxModType::Original,
        };

        Self {
            generic,
            other: HashMap::new(),
        }
    }

    pub fn add_generic(&mut self, name: Rc<str>) {
        assert_ne!(Rc::as_ref(&name), "generic");
        let m = RawLinuxMod::generic(name.clone());
        if self.other.insert(name.clone(), m).is_none() {
            self.generic.features.push(name);
        }
    }

    pub fn add_original(&mut self, name: Rc<str>) {
        assert_ne!(Rc::as_ref(&name), "generic");
        let m = RawLinuxMod::original(name.clone());
        self.other.insert(name, m);
    }

    #[inline]
    pub fn generic(&mut self) -> LinuxModMut {
        LinuxModMut(LinuxModMutRepr::Original(&mut self.generic))
    }

    #[inline]
    pub fn contains<K: AsRef<str>>(&self, name: K) -> bool {
        let name = name.as_ref();

        name == "generic" || self.other.contains_key(name)
    }

    pub fn get<K: AsRef<str>>(&mut self, name: K) -> Option<LinuxModMut> {
        let name = name.as_ref();

        if name == "generic" {
            Some(self.generic())
        } else if let Some(v) = self.other.get_mut(name) {
            Some(LinuxModMut(match v.r#type {
                RawLinuxModType::Generic => LinuxModMutRepr::Generic(&mut self.generic, v),
                RawLinuxModType::Original => LinuxModMutRepr::Original(v),
            }))
        } else {
            None
        }
    }

    pub fn build(self) -> LinuxMod {
        let Self { generic, other } = self;
        let mut other = other.into_values().collect::<Vec<_>>();
        other.sort_by_key(|f| f.name.clone());
        let mut m = Vec::new();

        m.push(generic.build());
        for r in other {
            m.push(r.build());
        }

        LinuxMod(m)
    }

    pub fn iter_mut(&mut self) -> LinuxModBuildIterMut {
        self.into_iter()
    }
}

pub struct LinuxModBuildIterMut<'a> {
    generic: &'a mut RawLinuxMod,
    generic_gone: bool,
    inner: std::collections::hash_map::ValuesMut<'a, Rc<str>, RawLinuxMod>,
}

impl<'a> IntoIterator for &'a mut LinuxModBuilder {
    type Item = LinuxModMut<'a>;

    type IntoIter = LinuxModBuildIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LinuxModBuildIterMut {
            generic: &mut self.generic,
            generic_gone: false,
            inner: self.other.values_mut(),
        }
    }
}

impl<'a> Iterator for LinuxModBuildIterMut<'a> {
    type Item = LinuxModMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.generic_gone {
            let next = self.inner.next()?;
            Some(LinuxModMut(match next.r#type {
                RawLinuxModType::Generic => {
                    LinuxModMutRepr::Generic(unsafe { &mut *(self.generic as *mut _) }, next)
                }
                RawLinuxModType::Original => LinuxModMutRepr::Original(next),
            }))
        } else {
            self.generic_gone = true;
            Some(LinuxModMut::<'a>(LinuxModMutRepr::<'a>::Original(unsafe {
                &mut *(self.generic as *mut _)
            })))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = ExactSizeIterator::len(self);
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for LinuxModBuildIterMut<'a> {
    fn len(&self) -> usize {
        self.inner.len() + (!self.generic_gone) as usize
    }
}
