use std::{fs::Metadata, path::Path, str::FromStr};

use anyhow::{Context, Result};

use super::server_compilers::Methods;

#[derive(Debug, Clone)]
pub struct FileData {
    pub file_name: String,
    pub file_extension: String,
    pub router_path: String,
    pub file_path: String,
    pub metadata: Metadata,
    pub method: Methods,
}

pub fn get_file_data(parent_path: &Path, child_path: &Path) -> Result<FileData> {
    // Get the parent path as a string, this will be used to convert the child path into a router path
    let p_path_str = parent_path
        .to_str()
        .context("Couldn't convert parent path to &str")?;

    // Get the files path as a string
    let f_path_str = child_path
        .to_str()
        .context("Couldn't convert child path to str")?;

    // Gets the files name as a string
    let f_name_str = child_path
        .file_name()
        .context("Couldn't get file name")?
        .to_str()
        .context("Couldn't convert name to &str")?;

    // Gets the file extension
    let f_ext = child_path
        .extension()
        .context("Couldn't get file extension from path")?
        .to_str()
        .context("Couldn't convert file extension to &str")?;

    // Get the router path
    let f_router_path = f_path_str.replace(p_path_str, "").replace(f_name_str, "");

    let metadata = child_path.metadata()?;

    let method_str = f_name_str.replace(&format!(".{}", f_ext), "");

    let method = Methods::from_str(&method_str).unwrap_or(Methods::Unknown(method_str));

    Ok(FileData {
        file_name: String::from(f_name_str),
        file_extension: String::from(f_ext),
        router_path: f_router_path,
        file_path: String::from(f_path_str),
        metadata,
        method,
    })
}
