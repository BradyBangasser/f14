use std::{path::Path, fs::ReadDir};

use crate::f14fs;

use super::server_compilers::Route;

pub fn default_get_routes(p: &str) -> Result<Vec<Route>, String> {
    let routes: Vec<Route> = vec![];
    let files = f14fs::read_dir(Path::new(p));

    if files.is_err() {
        return Err(files.err().unwrap());
    }


    Ok(routes)
}

