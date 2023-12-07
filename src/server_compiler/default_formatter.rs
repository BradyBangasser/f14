use super::server_compilers::ServerCompiler;
use anyhow::Result;

pub fn default_formatter(_sc: &ServerCompiler, code: Vec<String>) -> Result<String> {
    Ok(code.join("\n"))
}