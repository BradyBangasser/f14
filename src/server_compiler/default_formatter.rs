use super::{server_compilers::ServerCompiler, multiply_string::multiply_string};
use anyhow::Result;
use regex::Regex;

pub fn default_formatter(_sc: &ServerCompiler, code: Vec<String>) -> Result<String> {
    let mut mutatable_code = code;

    let open_bracket_regex = Regex::new(r"[{\[(]$")?;
    let close_bracjet_regex = Regex::new(r"^[}\])]")?;
    
    let mut tab_index: u32 = 0;

    for line in mutatable_code.iter_mut() {
        if close_bracjet_regex.is_match(line) && tab_index > 0 {
            tab_index -= 1;
        }

        let mut tabs = multiply_string(&"\t", tab_index);
        tabs.push_str(&line);
        *line = tabs;

        if open_bracket_regex.is_match(line) {
            tab_index += 1;
        }
    }

    Ok(mutatable_code.join("\n"))
}