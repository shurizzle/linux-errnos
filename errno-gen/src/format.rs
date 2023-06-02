use std::{
    fmt,
    io::{Read, Write},
    path::Path,
    process::{Command, Stdio},
};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};

pub struct Formatter {
    bin: Box<Path>,
}

impl Formatter {
    pub fn new() -> Result<Self> {
        Ok(Self {
            bin: which::which("rustfmt")
                .wrap_err("Cannot find rustfmt bin")?
                .into_boxed_path(),
        })
    }

    #[inline]
    pub fn format<S: fmt::Display>(&self, code: S) -> Result<String> {
        let mut buf = String::new();
        self.format_in(code, &mut buf).map(|_| buf)
    }

    pub fn format_in<S: fmt::Display>(&self, code: S, buf: &mut String) -> Result<()> {
        let mut child = Command::new(self.bin.as_os_str())
            .arg("--emit")
            .arg("stdout")
            .stderr(Stdio::inherit())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .wrap_err("Failed to format file")?;

        // write code to stdin
        {
            let mut stdin = if let Some(stdin) = child.stdin.take() {
                stdin
            } else {
                bail!("Failed to format file");
            };
            write!(stdin, "{}", code).wrap_err("Failed to format file")?;
            stdin.flush().wrap_err("Failed to format file")?;
        }

        drop(child.stdin.take());
        drop(child.stderr.take());
        match child.stdout.take() {
            Some(mut out) => {
                out.read_to_string(buf)?;
            }
            _ => (),
        }

        let status = child.wait().wrap_err("Failed to format file")?;
        if !status.success() {
            bail!("Failed to format file");
        }

        Ok(())
    }
}
