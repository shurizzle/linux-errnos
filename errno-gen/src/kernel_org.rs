#![allow(dead_code)]

use std::{fmt, io::Read, path::Path};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use egzreader::EgzReader;
use serde::Deserialize;
use tar::Archive;

#[derive(Deserialize)]
struct Release {
    pub version: String,
    pub moniker: String,
}

#[derive(Deserialize)]
struct Releases {
    pub releases: Vec<Release>,
}

pub fn latest_version() -> Result<Box<str>> {
    let releases: Releases = serde_json::from_reader(
        ureq::get("https://www.kernel.org/releases.json")
            .call()
            .wrap_err("Failed to fetch releases.json")?
            .into_reader(),
    )
    .wrap_err("Failed to parse releases.json")?;

    for release in releases.releases {
        if release.moniker == "mainline" {
            return Ok(release.version.into_boxed_str());
        }
    }
    bail!("mainline kernel version not found")
}

fn url_for_version<S: fmt::Display>(version: S) -> Box<str> {
    format!("https://git.kernel.org/torvalds/t/linux-{}.tar.gz", version).into_boxed_str()
}

pub fn fetch_kernel<S: fmt::Display>(
    version: S,
) -> Result<Archive<EgzReader<Box<dyn Read + Send + Sync>>>> {
    ureq::get(&url_for_version(version))
        .call()
        .wrap_err("Failed to fetch kernel")
        .map(|r| Archive::new(EgzReader::new(r.into_reader())))
}

fn download<T: fmt::Display, P: AsRef<Path>>(version: T, srcdir: P) -> Result<()> {
    eprintln!("Downloading kernel {} tarball...", version);
    for entry in fetch_kernel(version)?
        .entries()
        .wrap_err("Failed to read kernel archive")?
    {
        let mut entry = entry.wrap_err("Failed to read kernel archive")?;
        if !entry.path_bytes().ends_with(b".h") {
            continue;
        }

        let path = {
            let path = entry.path_bytes();
            let path = path.as_ref();
            let path = if let Some(pos) = memchr::memchr(b'/', path) {
                unsafe { path.get_unchecked((pos + 1)..) }
            } else {
                continue;
            };

            fn is_arch_include(path: &[u8]) -> bool {
                if let Some(path) = path.strip_prefix(b"arch/") {
                    if let Some(pos) = memchr::memchr(b'/', path) {
                        let path = unsafe { path.get_unchecked((pos + 1)..) };
                        path.starts_with(b"include/")
                    } else {
                        false
                    }
                } else {
                    false
                }
            }

            if !(path.starts_with(b"include/linux/")
                || path.starts_with(b"include/uapi/")
                || is_arch_include(path))
            {
                continue;
            }

            {
                let path = {
                    use std::os::unix::prelude::OsStrExt;
                    let mut p = std::path::PathBuf::new();
                    p.push(srcdir.as_ref());
                    p.push(std::ffi::OsStr::from_bytes(path));
                    p
                };

                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)
                        .wrap_err("Failed while writing source files")?;
                }
                path
            }
        };

        entry
            .unpack(path)
            .wrap_err("Failed while writing source files")?;
    }
    Ok(())
}

#[inline]
pub fn download_latest<P: AsRef<Path>>(srcdir: P) -> Result<()> {
    download(latest_version()?, srcdir)
}
