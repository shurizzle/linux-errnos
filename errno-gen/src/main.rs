mod builder;
mod format;
mod generate;
mod kernel_org;

use std::{
    fmt,
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    rc::Rc,
};

use builder::LinuxModBuilder;
use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use generate::{generate_bindings, generic_bindings};

use crate::{
    builder::{Lib, Platform},
    format::Formatter,
};

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
    let srcdir = tempdir.path();

    real_main(srcdir, outdir.as_str())
}

fn real_main<P1: AsRef<Path>, P2: AsRef<Path>>(srcdir: P1, outdir: P2) -> Result<()> {
    let srcdir = srcdir.as_ref();
    let outdir = outdir.as_ref();

    let formatter = Formatter::new()?;

    std::fs::create_dir_all(srcdir).wrap_err("Failed to create source directory")?;

    kernel_org::download_latest(srcdir)?;

    let mut builder = generate_linux_modules(srcdir, outdir, &formatter)?;
    let mut features = Vec::new();

    for m in &mut builder {
        let name = m.name();
        if let Err(pos) = features.binary_search(&name) {
            features.insert(pos, name);
        }
    }

    for p in Platform::iter() {
        let arch = Rc::from(p.rust_arch());
        if let Err(pos) = features.binary_search(&arch) {
            features.insert(pos, arch);
        }

        let mut arch = if let Some(a) = builder.get(p.linux_arch()) {
            a
        } else {
            bail!("Arch {} not found.", p.linux_arch())
        };

        p.add_to_cfg(&mut arch);
    }

    let linux_mod = builder.build();

    {
        let mut path = outdir.to_path_buf();
        path.push("src");
        path.push("linux");
        path.push("mod.rs");

        write_if_ne(
            path,
            "Error numbers contained in linux/arch/*.",
            &formatter,
            linux_mod,
        )?;
    }

    {
        let mut path = outdir.to_path_buf();
        path.push("src");
        path.push("lib.rs");

        write_if_ne(
            path,
            "Error numbers for every arch supported by linux.",
            &formatter,
            Lib,
        )?;
    }

    for f in features.iter() {
        println!("{} = []", f);
    }
    println!("all = [");
    for f in features.iter().map(Rc::as_ref) {
        println!("  {:?},", f);
    }
    println!("]");

    Ok(())
}

fn generate_linux_modules<P1: AsRef<Path>, P2: AsRef<Path>>(
    srcdir: P1,
    outdir: P2,
    formatter: &Formatter,
) -> Result<LinuxModBuilder> {
    let srcdir = srcdir.as_ref();

    let outdir = {
        let mut path = PathBuf::from(outdir.as_ref());
        path.push("src");
        path.push("linux");

        std::fs::create_dir_all(&path).wrap_err("Failed to write results")?;

        path
    };

    let generic = generic_bindings(srcdir)?;

    {
        let mut outfile = outdir.clone();
        outfile.push("generic.rs");
        write_if_ne(outfile, "Generic error numbers", formatter, &generic)?;
    }
    let mut builder = LinuxModBuilder::new();

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
            builder.add_generic(Rc::from(arch));
        } else {
            let mut outfile = outdir.clone();
            outfile.push(format!("{}.rs", arch));
            write_if_ne(
                outfile,
                format!("Error numbers for arch `{}`", arch),
                formatter,
                bindings,
            )?;
            builder.add_original(Rc::from(arch));
        }
    }

    for entry in std::fs::read_dir(&outdir).wrap_err("Failed to generate bindings")? {
        let entry = entry.wrap_err("Failed to generate bindings")?;

        {
            let md = entry.metadata().wrap_err("Failed to generate bindings")?;
            if !md.is_dir() || md.is_symlink() {
                continue;
            }
        }

        let path = entry.path();

        if let Some(name) = path.file_name() {
            if let Some(name) = name.to_str() {
                if name == "mod.rs" {
                    continue;
                }

                if let Some(name) = name.strip_suffix(".rs") {
                    if builder.contains(name) {
                        continue;
                    }
                }
            }
        } else {
            continue;
        }

        std::fs::remove_file(path).wrap_err("Failed to clean output directories")?;
    }

    Ok(builder)
}

fn write_if_ne<P, B, S>(path: P, desc: S, formatter: &Formatter, content: B) -> Result<()>
where
    P: AsRef<Path>,
    B: fmt::Display,
    S: AsRef<str>,
{
    struct DocFile<'a, T: fmt::Display>(&'a str, T);
    impl<'a, T: fmt::Display> fmt::Display for DocFile<'a, T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "//! {}.\n\n{}", self.0, self.1)
        }
    }

    let content = formatter.format(DocFile(desc.as_ref(), content))?;
    let path = path.as_ref();

    match slurp(path) {
        Err(err) if err.kind() != std::io::ErrorKind::NotFound => {
            return Err(err).wrap_err("Failed to write results")
        }
        Ok(buf) if content.as_bytes() == buf => return Ok(()),
        _ => (),
    }

    let mut f = File::create(path).wrap_err("Failed to write results")?;
    std::io::Write::write_all(&mut f, content.as_bytes()).wrap_err("Failed to write results")
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
