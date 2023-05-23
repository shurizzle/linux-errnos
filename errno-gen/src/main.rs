mod format;
mod generate;
mod kernel_org;

use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use generate::{generate_bindings, generic_bindings, Bindings};

use crate::format::Formatter;

const MAPPING: &[(&str, &[&str])] = [
    ("x86", ["x86", "x86_64"].as_slice()),
    ("arm", ["arm"].as_slice()),
    ("arm64", ["aarch64"].as_slice()),
    ("hexagon", ["hexagon"].as_slice()),
    ("s390", ["s390x"].as_slice()),
    ("powerpc", ["powerpc", "powerpc64"].as_slice()),
    ("mips", ["mips", "mips64"].as_slice()),
    ("m68k", ["m68k"].as_slice()),
    ("riscv", ["riscv32", "riscv64"].as_slice()),
    ("sparc", ["sparc", "sparc64"].as_slice()),
]
.as_slice();

fn main() -> Result<()> {
    color_eyre::install()?;

    let outdir = {
        let mut args = std::env::args();
        if args.len() != 2 {
            bail!("Please provide the output directory");
        }
        _ = args.next();
        args.next().unwrap()
    };

    let tempdir = tempfile::Builder::new()
        .tempdir()
        .wrap_err("Cannot create a temporary directory")?;

    real_main(tempdir.path(), outdir.as_str())
}

fn real_main<P1: AsRef<Path>, P2: AsRef<Path>>(srcdir: P1, outdir: P2) -> Result<()> {
    let srcdir = srcdir.as_ref();
    let outdir = outdir.as_ref();

    let formatter = Formatter::new()?;

    std::fs::create_dir_all(srcdir).wrap_err("Failed to create source directory")?;

    kernel_org::download_latest(srcdir)?;

    let generic = generic_bindings(srcdir)?;

    let mut archs = HashMap::new();
    let mut archdir = PathBuf::from(srcdir);
    archdir.push("arch");
    for entry in std::fs::read_dir(archdir).wrap_err("Failed to generate bindings")? {
        let entry = entry.wrap_err("Failed to generate bindings")?;

        {
            let md = entry.metadata().wrap_err("Failed to generate bindings")?;
            if !md.is_dir() || md.is_symlink() {
                continue;
            }
        }

        let path = entry.path();

        let arch = if let Some(name) = path.file_name() {
            if name == "generic" {
                continue;
            }
            name.to_string_lossy().into_owned()
        } else {
            continue;
        };

        let bindings = generate_bindings(srcdir, &arch)?;

        if bindings == generic {
            archs.insert(arch, B::Generic);
        } else {
            archs.insert(arch, B::Arch(bindings));
        }
    }

    let generic = formatter.format(generic)?;

    let outdir = {
        let mut path = PathBuf::from(outdir);
        path.push("src");
        let mut p = path.clone();
        p.push("linux");

        std::fs::create_dir_all(p).wrap_err("Failed to write results")?;

        path
    };

    {
        let mut outfile = outdir.clone();
        outfile.push("linux");
        outfile.push("generic.rs");
        write_if_ne(outfile, generic.as_bytes())?;
    }

    let mut generic_cond = Cond::default();
    let mut platforms = Vec::new();
    let mut features = vec![];
    {
        let generic = Rc::new("generic".to_string().into_boxed_str());
        generic_cond.platforms.push(generic.clone());
        features.push(generic);
    }
    let mut rust_archs = Vec::new();
    rust_archs.extend_from_slice("#![cfg_attr(not(feature = \"std\"), no_std)]\n\n".as_bytes());
    rust_archs.extend_from_slice("pub(crate) mod macros;\npub mod linux;\n\n".as_bytes());
    for (plat, bind) in archs {
        let archs = 'mapping: {
            for (p, archs) in MAPPING.iter().copied() {
                if p == plat {
                    if archs.is_empty() {
                        break 'mapping None;
                    } else {
                        break 'mapping Some(
                            archs
                                .iter()
                                .copied()
                                .map(|x| Rc::new(x.to_string().into_boxed_str()))
                                .collect::<Vec<Rc<Box<str>>>>(),
                        );
                    }
                }
            }
            None
        };
        let plat = Rc::new(plat.into_boxed_str());

        {
            let mut cond = Cond::default();
            cond.platforms.push(plat.clone());
            features.push(plat.clone());
            if let Some(archs) = archs.as_ref() {
                cond.archs.extend_from_slice(archs.as_slice());
                features.extend_from_slice(archs.as_slice());
            }

            match bind {
                B::Generic => {
                    generic_cond.platforms.push(plat.clone());
                    if let Some(archs) = archs.as_ref() {
                        generic_cond.archs.extend_from_slice(archs.as_slice());
                    }

                    _ = writeln!(
                        platforms,
                        "{cond}pub mod {plat} {{ pub use super::generic::*; }}"
                    );
                }
                B::Arch(bind) => {
                    {
                        let formatted = formatter.format(bind)?;
                        let mut outfile = outdir.clone();
                        outfile.push("linux");
                        outfile.push(format!("{}.rs", plat));
                        write_if_ne(outfile, formatted.as_bytes())?;
                    }
                    _ = writeln!(platforms, "{cond}pub mod {plat};");
                }
            }
        }

        if let Some(archs) = archs {
            for arch in archs {
                let mut cond = Cond::default();
                cond.archs.push(arch.clone());
                _ = writeln!(
                    rust_archs,
                    "{cond}pub mod {arch} {{ pub use super::linux::{plat}::*; }}"
                );
                _ = writeln!(rust_archs, "#[cfg(all(target_os = \"linux\", target_arch = {arch:?}))]\npub use {arch}::*;");
            }
        }
    }

    let platforms = {
        let mut t = format!("{generic_cond}pub mod generic;\n\n").into_bytes();
        t.append(&mut platforms);
        _ = platforms;
        formatter.format(unsafe { String::from_utf8_unchecked(t) })?
    };

    {
        let mut path = outdir.clone();
        path.push("linux");
        path.push("mod.rs");
        write_if_ne(path, platforms)?;
    }

    let archs = {
        let t = formatter.format(unsafe { String::from_utf8_unchecked(rust_archs) })?;
        _ = rust_archs;
        t
    };

    {
        let mut path = outdir;
        path.push("lib.rs");
        write_if_ne(path, archs)?;
    }

    {
        features.sort();
        features.dedup();
        for feat in features.iter() {
            println!("{} = []", feat);
        }
        print!("all = [");
        for (i, feat) in features.into_iter().enumerate() {
            if i != 0 {
                print!(", ");
            }
            print!("{:?}", Rc::as_ref(&feat).as_ref());
        }
        println!("]");
    }

    Ok(())
}

#[derive(Debug)]
enum B {
    Generic,
    Arch(Bindings),
}

fn write_if_ne<P: AsRef<Path>, B: AsRef<[u8]>>(path: P, content: B) -> Result<()> {
    let path = path.as_ref();
    let content = content.as_ref();

    match slurp(path) {
        Err(err) if err.kind() != std::io::ErrorKind::NotFound => {
            return Err(err).wrap_err("Failed to write results")
        }
        Ok(buf) if content == buf => return Ok(()),
        _ => (),
    }

    let mut f = File::create(path).wrap_err("Failed to write results")?;
    std::io::Write::write_all(&mut f, content).wrap_err("Failed to write results")?;

    Ok(())
}

pub fn slurp<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let len = f.seek(SeekFrom::End(0))? as usize;
    f.seek(SeekFrom::Start(0))?;

    let mut buf = Vec::<u8>::with_capacity(len);
    unsafe {
        f.read_exact(std::slice::from_raw_parts_mut(buf.as_mut_ptr(), len))?;
        buf.set_len(len);
    }

    Ok(buf)
}

#[derive(Debug, Default)]
struct Cond {
    platforms: Vec<Rc<Box<str>>>,
    archs: Vec<Rc<Box<str>>>,
}

impl fmt::Display for Cond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut archs = self.archs.clone();
        archs.sort();
        archs.dedup();
        let mut plats = self.platforms.clone();
        plats.extend_from_slice(archs.as_slice());
        plats.sort();
        plats.dedup();

        if plats.is_empty() {
            return Ok(());
        }

        write!(f, "#[cfg(")?; // open 1
        if !self.archs.is_empty() {
            write!(f, "any(")?; // open 2
        }

        if plats.len() > 1 && self.archs.is_empty() {
            write!(f, "any(")?; // open 3
        }
        for (i, plat) in plats.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "feature = {:?}", Rc::as_ref(plat).as_ref())?;
        }
        if plats.len() > 1 && self.archs.is_empty() {
            write!(f, ")")?; // close 3
        }

        if !self.archs.is_empty() {
            write!(f, ", all(target_os = \"linux\", ")?; // open 4

            if archs.len() > 1 {
                write!(f, "any(")?; // open 5
            }
            for (i, arch) in archs.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "target_arch = {:?}", Rc::as_ref(arch).as_ref())?;
            }
            if archs.len() > 1 {
                write!(f, ")")?; // close 5
            }

            write!(f, "))")?; // close 4 + 2
        }

        writeln!(f, ")]") // close 1
    }
}
