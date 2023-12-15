mod default_formatter;
mod default_get_routes;
mod default_writer;
pub mod get_file_data;
pub mod multiply_string;

pub mod server_compilers {
    use std::{path::Path, cmp};
    use anyhow::{Result, anyhow};
    use regex::Regex;

    pub fn create_safe_file_variable(fpath: &str) -> String {
        let re = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
        let ending_re = Regex::new(r"_$").unwrap();

        let sfv = fpath.to_string();

        ending_re.replace_all(&re.replace_all(&sfv, "_").to_string(), "").into()
    }

    use super::{
        default_formatter::default_formatter,
        default_get_routes::default_get_routes, default_writer::default_writer, get_file_data::FileData,
    };

    #[derive(strum_macros::Display, strum_macros::EnumString, Debug, Clone, PartialEq, cmp::Ord, cmp::PartialOrd, cmp::Eq)]
    pub enum Methods {
        GET,
        POST,
        Unknown(String),
    }

    #[derive(Clone, PartialEq, cmp::PartialOrd, cmp::Eq)]
    pub struct Route {
        method: Methods,
        path: String,
    }

    impl Ord for Route {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.get_path().cmp(&other.get_path())
        }
    }

    impl std::fmt::Debug for Route {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Route").field("method", &self.method).field("path", &self.path).finish()
        }
    }

    impl Route {
        pub fn new(method: Methods, path: &Path) -> Self {
            Route {
                method,
                path: path
                    .to_str()
                    .expect("Path couldn't be converted to string")
                    .to_string(),
            }
        }

        pub fn from_file_data(data: FileData) -> Self {
            Self::new(data.method, &Path::new(&data.router_path))
        }

        pub fn get_path(&self) -> String { self.path.clone() }
        pub fn get_method(&self) -> Methods { self.method.clone() }
    }

    impl std::fmt::Display for Route {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}: {}", self.method, self.path)
        }
    }

    #[derive(Clone)]
    pub struct ServerCompiler {
        language: String,
        // Get all of the routes that the compiler will need to generate code for
        get_routes: fn(&Self, &Path) -> Result<Vec<Route>>, // This many need to have a customer error type I spelled custom wrong
        generate_router_code: Option<fn(&Self, Vec<Route>) -> Result<Vec<String>>>, // And this too
        formatter: fn(&Self, Vec<String>) -> Result<String>, // And this
        writer: fn(&Self, String, &Path) -> Result<bool>,
        pub file_extension: Option<String>,
        pub detect: Option<fn(&str) -> bool>,
        ignored_file_names: Vec<String>
    }

    impl ServerCompiler {
        pub fn compile(&self, path: &Path) -> Result<bool> {
            let routes = (&self.get_routes)(self, path)?;

            if self.generate_router_code.is_none() {
                return Err(anyhow!("Couldn't generate router code because generate_router_code function is not defined"));
            }

            let code_vector = (&self.generate_router_code.unwrap())(self, routes)?;

            let code = (&self.formatter)(self, code_vector)?;

            println!("{}", path.display());
            (&self.writer)(self, code, &path.join("build.go"))?;

            return Ok(true)
        }

        pub fn new(lang_name: &str) -> Self {
            ServerCompiler {
                language: lang_name.to_uppercase().to_string(),
                get_routes: default_get_routes,
                formatter: default_formatter,
                writer: default_writer,
                generate_router_code: None,
                file_extension: None,
                detect: None,
                ignored_file_names: vec![],
            }
        }

        pub fn set_file_extension(&mut self, extension: &str) -> &mut Self {
            self.file_extension = Some(extension.to_string());
            self
        }

        pub fn set_code_generator(&mut self, func: fn(&ServerCompiler, Vec<Route>) -> Result<Vec<String>>) -> &mut Self {
            self.generate_router_code = Some(func);
            self
        }

        pub fn set_ignored_files(&mut self, ifv: Vec<String>) -> &mut Self {
            self.ignored_file_names = ifv;
            self
        }

        pub fn get_ignored_files(&self) -> &Vec<String> {
            &self.ignored_file_names
        }
    }

    impl std::fmt::Display for ServerCompiler {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{}", &self.language))
        }
    }

    impl std::fmt::Debug for ServerCompiler {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{}", &self.language))
        }
    }
}
