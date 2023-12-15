use std::{path::Path, fs};
use anyhow::Result;

use super::server_compilers::ServerCompiler;

pub fn default_writer(_sc: &ServerCompiler, code: String, p: &Path) -> Result<bool> {
    fs::write(p, code)?;
    Ok(true)
}