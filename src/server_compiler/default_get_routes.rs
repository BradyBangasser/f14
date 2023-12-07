use std::path::Path;
use anyhow::Result;

use crate::{f14fs, server_compiler::get_file_data::get_file_data};

use super::server_compilers::{Route, ServerCompiler};

pub fn default_get_routes(_sc: &ServerCompiler, p: &Path) -> Result<Vec<Route>> {
    let mut routes: Vec<Route> = vec![];
    let files = f14fs::read_dir(p);

    let unwrapped_files = files.unwrap();

    for file in unwrapped_files {
        let f_data = get_file_data(p, file.path().as_path())?;
        
        routes.push(Route::from_file_data(f_data));
    }

    Ok(routes)
}

