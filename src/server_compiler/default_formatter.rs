pub fn default_formatter(code: Vec<String>) -> Result<String, String> {
    Ok(code.join("\n"))
}