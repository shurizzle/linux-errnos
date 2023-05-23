use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
    fmt,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use atoi::FromRadix10;
use bindgen::callbacks::ParseCallbacks;
use color_eyre::{
    eyre::{bail, eyre, Context},
    Result,
};

pub fn interesting_files<P: AsRef<Path>, S: AsRef<str>>(
    srcdir: P,
    arch: S,
) -> Result<Vec<Box<Path>>> {
    let srcdir = srcdir.as_ref();
    let arch = arch.as_ref();
    let archdir = {
        let mut p = PathBuf::from(srcdir);
        p.push("arch");
        p.push(arch);
        p.push("include");
        p
    };
    let linuxdir = {
        let mut p = PathBuf::from(srcdir);
        p.push("include");
        p
    };

    {
        let mut srcdir = archdir.clone();

        srcdir.push("asm");
        std::fs::create_dir_all(&srcdir).wrap_err("Failed to generate bindings")?;
        srcdir.push("errno.h");
        if !srcdir.exists() {
            let mut f = File::create(srcdir).wrap_err("Failed to generate bindings")?;
            f.write_all(b"#include <uapi/asm-generic/errno.h>\n")
                .wrap_err("Failed to generate bindings")?;
        }
    }

    {
        let mut srcdir = archdir.clone();
        srcdir.push("asm-generic");
        std::fs::create_dir_all(&srcdir).wrap_err("Failed to generate bindings")?;
        srcdir.push("errno-base.h");
        if !srcdir.exists() {
            let mut f = File::create(srcdir).wrap_err("Failed to generate bindings")?;
            f.write_all(b"#include <uapi/asm-generic/errno-base.h>\n")
                .wrap_err("Failed to generate bindings")?;
        }
    }

    #[derive(Debug)]
    struct RegisterFiles(Rc<RefCell<Vec<Box<Path>>>>);
    impl ParseCallbacks for RegisterFiles {
        fn include_file(&self, filename: &str) {
            self.0
                .borrow_mut()
                .push(PathBuf::from(filename).into_boxed_path());
        }
    }

    let visited = Rc::new(RefCell::<Vec<Box<Path>>>::new(vec![{
        let mut p = linuxdir.clone();
        p.push("linux");
        p.push("errno.h");
        p.into_boxed_path()
    }]));

    _ = bindgen::builder()
        .detect_include_paths(false)
        .clang_arg("-nostdinc")
        .clang_arg("-I")
        .clang_arg(archdir.as_os_str().to_string_lossy().to_string())
        .clang_arg("-I")
        .clang_arg(linuxdir.as_os_str().to_string_lossy().to_string())
        .parse_callbacks(Box::new(RegisterFiles(visited.clone())))
        .header_contents("__bindings.h", "#include <linux/errno.h>")
        .generate()
        .wrap_err("Failed to generate bindings")?;

    let mut visited = Rc::try_unwrap(visited)
        .ok()
        .ok_or_else(|| eyre!("Failed to generate bindings"))?
        .into_inner();
    visited.sort();
    visited.dedup();

    Ok(visited)
}

#[derive(Debug)]
pub enum ParsedErrno {
    Define(Box<str>, i32, Option<Box<str>>),
    Alias(Box<str>, Box<str>, Option<Box<str>>),
}

fn parse_line(line: &str) -> Option<ParsedErrno> {
    fn space0(b: &str) -> &str {
        b.trim_start()
    }

    fn space1(b: &str) -> Option<&str> {
        if b.chars().next()?.is_whitespace() {
            Some(b.trim_start())
        } else {
            None
        }
    }

    fn int<T: FromRadix10>(b: &str) -> Option<(T, &str)> {
        let (res, size) = T::from_radix_10(b.as_bytes());
        if size == 0 {
            None
        } else {
            Some((res, unsafe { b.get_unchecked(size..) }))
        }
    }

    fn id(b: &str) -> Option<(Box<str>, &str)> {
        b.char_indices()
            .find(|&(i, c)| {
                if i == 0 {
                    !c.is_ascii_alphabetic() && c != '_'
                } else {
                    !c.is_ascii_alphanumeric() && c != '_'
                }
            })
            .and_then(|(i, _)| {
                let name = unsafe { b.get_unchecked(..i) };
                if !name.is_empty() && name.starts_with('E') && name != "EMAXERRNO" {
                    Some((name.to_string().into_boxed_str(), unsafe {
                        b.get_unchecked(i..)
                    }))
                } else {
                    None
                }
            })
            .or_else(|| {
                if b.is_empty() {
                    None
                } else {
                    Some((b.to_string().into_boxed_str(), unsafe {
                        b.get_unchecked(b.len()..)
                    }))
                }
            })
    }

    fn comment(b: &str) -> Option<Box<str>> {
        let b = b.trim();
        if let Some(b) = b.strip_prefix("/*") {
            let b = b.strip_suffix("*/")?.trim();
            if b.is_empty() {
                None
            } else {
                Some(b.to_string().into_boxed_str())
            }
        } else if let Some(b) = b.strip_prefix("//") {
            let b = b.trim();
            if b.is_empty() {
                None
            } else {
                Some(b.to_string().into_boxed_str())
            }
        } else {
            None
        }
    }

    let line = space0(line);
    let line = line.strip_prefix('#')?;
    let line = space0(line);
    let line = line.strip_prefix("define")?;
    let line = space1(line)?;
    let (name, line) = id(line)?;
    let line = space1(line)?;

    if let Some((value, line)) = int::<i32>(line) {
        Some(ParsedErrno::Define(name, value, comment(line)))
    } else if line.starts_with('E') {
        let (alias, line) = id(line)?;
        Some(ParsedErrno::Alias(
            name,
            alias.to_string().into_boxed_str(),
            comment(line),
        ))
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Bindings {
    pub defines: Vec<(Box<str>, i32, Box<str>)>,
    pub aliases: Vec<(Box<str>, Box<str>)>,
}

impl PartialEq for Bindings {
    fn eq(&self, other: &Self) -> bool {
        if self.defines.len() != other.defines.len() || self.aliases.len() != other.aliases.len() {
            return false;
        }

        for a in &self.defines {
            if let Some(b) = other.defines.iter().find(|(n, _, _)| *n == a.0) {
                if a.1 != b.1 {
                    return false;
                }
            }
        }

        for a in &self.aliases {
            if let Some(b) = other.aliases.iter().find(|(n, _)| *n == a.0) {
                if a.1 != b.1 {
                    return false;
                }
            }
        }

        true
    }
}

impl fmt::Display for Bindings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#![allow(dead_code)]\n")?;
        writeln!(f, "// This file automatically generate. Do not edit.\n")?;
        writeln!(f, "crate::macros::def_errno!();\n")?;

        writeln!(f, "impl Errno {{")?;
        let mut min: Option<i32> = None;
        let mut max: Option<i32> = None;
        let mut errs = Vec::<i32>::new();
        for (name, no, desc) in &self.defines {
            max = max.map_or(Some(*no), |max| Some(max.max(*no)));
            min = min.map_or(Some(*no), |min| Some(min.min(*no)));
            errs.push(*no);
            writeln!(f, "    /// {}", desc)?;
            writeln!(f, "    pub const {}: Self = Self({});", name, no)?;
        }
        for (alias, name) in &self.aliases {
            writeln!(f, "    /// Alias for {}", name)?;
            writeln!(f, "    pub const {}: Self = Self::{};", alias, name)?;
        }
        if let Some(min) = min.take() {
            writeln!(f, "\n    pub const MIN: i32 = {};", min)?;
        }
        if let Some(max) = max.take() {
            writeln!(f, "    pub const MAX: i32 = {};", max)?;
        }
        writeln!(f, "\n    const ALL: [i32; {}] = {:?};", errs.len(), errs)?;
        writeln!(f,
        "\n    pub(crate) fn name_and_description(&self) -> Option<(&'static str, &'static str)> {{"
    )?;
        writeln!(f, "        match self.0 {{")?;
        for (name, no, desc) in &self.defines {
            writeln!(f, "            {} => Some(({:?}, {:?})),", no, name, desc)?;
        }
        writeln!(f, "            _ => None,")?;
        writeln!(f, "        }}")?;
        writeln!(f, "    }}")?;
        writeln!(f, "}}")
    }
}

pub fn generate_bindings<P: AsRef<Path>, A: AsRef<str>>(srcdir: P, arch: A) -> Result<Bindings> {
    let srcdir = srcdir.as_ref();
    let arch = arch.as_ref();

    let mut defs = HashMap::new();
    let mut aliases = HashMap::new();
    let mut nos = BTreeSet::new();
    for file in interesting_files(srcdir, arch)? {
        let mut buf = String::new();
        let mut f = BufReader::new(
            File::open(&file)
                .wrap_err_with(|| format!("Failed to generate bindings {}", file.display()))?,
        );

        while f
            .read_line(&mut buf)
            .wrap_err("Failed to generate bindings")?
            != 0
        {
            if let Some(e) = parse_line(buf.as_str()) {
                match e {
                    ParsedErrno::Alias(name, alias, _doc) => {
                        if defs.contains_key(&name) {
                            bail!("{} defined multiple times", name);
                        }
                        if let Some(other) = aliases.insert(name, alias) {
                            bail!("{} defined multiple times", other);
                        }
                    }
                    ParsedErrno::Define(name, value, doc) => {
                        if aliases.contains_key(&name) {
                            bail!("{} defined multiple times", name);
                        }
                        let desc = if let Some(desc) = doc {
                            desc
                        } else {
                            match name.as_ref() {
                                "ERESTARTSYS" => "Restart syscall",
                                "ERESTARTNOINTR" => "Restart if no interrupt",
                                _ => bail!("No description for {name}"),
                            }
                            .to_string()
                            .into_boxed_str()
                        };

                        if nos.contains(&value) {
                            bail!("Errno {} defined multiple times", value);
                        } else {
                            nos.insert(value);
                        }

                        if let Some((other, _)) = defs.insert(name, (value, desc)) {
                            bail!("{} defined multiple times", other)
                        }
                    }
                }
            }

            buf.clear();
        }
    }

    let mut aliases = {
        let mut t: Vec<(Box<str>, Box<str>)> = Vec::with_capacity(aliases.len());
        for (a, b) in aliases {
            if !defs.contains_key(&b) {
                bail!("Alias to unknown error {}", b);
            }
            t.push((a, b));
        }
        t
    };
    aliases.sort_by(|a, b| Ord::cmp(a.0.as_ref(), b.0.as_ref()));
    let mut defines: Vec<(Box<str>, i32, Box<str>)> =
        defs.into_iter().map(|(a, (b, c))| (a, b, c)).collect();
    defines.sort_by_key(|t| t.1);

    Ok(Bindings { defines, aliases })
}

pub fn generic_bindings<P: AsRef<Path>>(srcdir: P) -> Result<Bindings> {
    {
        let srcdir = srcdir.as_ref();
        let mut srcdir = PathBuf::from(srcdir);
        srcdir.push("arch");
        srcdir.push("generic");
        srcdir.push("include");
    }
    generate_bindings(srcdir, "generic")
}
