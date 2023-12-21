use std::path::Path;
use anyhow::Result;

use crate::{f14fs, server_compiler::get_file_data::get_file_data};

use super::server_compilers::{Route, ServerCompiler};

pub fn default_get_routes(sc: &ServerCompiler, p: &Path) -> Result<Vec<Route>> {
    let ignored_files = sc.get_ignored_files();
    let mut routes: Vec<Route> = vec![];
    let files = f14fs::read_dir(p);

    println!("impl error handling here");
    let unwrapped_files = files.unwrap();


    for file in unwrapped_files {
        let f_data = get_file_data(p, file.path().as_path())?;

        let replace_string = format!(".{}", f_data.file_extension);
        if ignored_files.contains(&f_data.file_name.replace(&replace_string, "")) || ignored_files.contains(&f_data.file_path) {
            continue;
        }
        
        routes.push(Route::from_file_data(f_data));
    }

    Ok(routes)
}

