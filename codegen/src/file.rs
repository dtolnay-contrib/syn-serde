use proc_macro2::TokenStream;
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
use tempfile::Builder;

use crate::Result;

pub(crate) fn write(path: impl AsRef<Path>, content: TokenStream) -> Result<()> {
    let mut formatted = Vec::new();
    writeln!(
        formatted,
        "// This file is @generated by syn-serde-internal-codegen.\n\
         // It is not intended for manual editing.\n"
    )?;

    let outdir = Builder::new().prefix("codegen").tempdir()?;
    let outfile_path = outdir.path().join("expanded");
    fs::write(&outfile_path, content.to_string())?;

    // Run rustfmt
    // https://github.com/dtolnay/cargo-expand/blob/0.4.9/src/main.rs#L181-L182
    let rustfmt_config_path = outdir.path().join("rustfmt.toml");
    let mut rustfmt_config = fs::read_to_string("../rustfmt.toml")?;
    rustfmt_config.push('\n');
    rustfmt_config.push_str("normalize_doc_attributes = true\n");
    rustfmt_config.push_str("format_macro_matchers = true\n");
    fs::write(rustfmt_config_path, rustfmt_config)?;

    // Ignore any errors.
    let _status = Command::new("rustfmt").arg(&outfile_path).stderr(Stdio::null()).status();

    formatted.extend(fs::read(&outfile_path)?);

    if path.as_ref().is_file() && fs::read(&path)? == formatted {
        return Ok(());
    }

    fs::write(path, formatted)?;
    Ok(())
}
