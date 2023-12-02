use std::{fs::DirEntry, path::Path};

pub fn read_dir(p: &Path) -> Result<Vec<DirEntry>, String> {
    let mut routes: Vec<DirEntry> = vec![];

    let read_dir_result = p.read_dir();

    match read_dir_result {
        Ok(dir) => {
            for file_result in dir {
                match file_result {
                    Ok(file) => {
                        let file_type = file.file_type().unwrap();
                        let file_name = file.file_name();

                        if file_type.is_dir() {
                            let route_path = p.join(file_name.to_str().unwrap());
                            let nested_result = read_dir(route_path.as_path());
                            if nested_result.is_err() {
                                return nested_result;
                            } else {
                                routes.append(nested_result.unwrap().as_mut());
                            }
                        } else {
                            routes.push(file);
                        }
                    }

                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            }

            Ok(routes)
        }
        Err(err) => Err(err.to_string()),
    }
}
