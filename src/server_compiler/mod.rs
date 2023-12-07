mod default_formatter;
mod default_get_routes;
mod default_writer;
pub mod get_file_data;

pub mod server_compilers {
    use std::path::Path;
    use anyhow::{Result, anyhow};

    use super::{
        default_formatter::default_formatter,
        default_get_routes::default_get_routes, default_writer::default_writer, get_file_data::FileData,
    };

    #[derive(strum_macros::Display, strum_macros::EnumString, Debug, Clone)]
    pub enum Methods {
        GET,
        POST,
        Unknown(String),
    }

    #[derive(Clone)]
    pub struct Route {
        method: Methods,
        path: String,
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
        writer: fn(&Self, String, &Path),
        pub file_extension: Option<String>,
        pub detect: Option<fn(&str) -> bool>,
    }

    impl ServerCompiler {
        pub fn compile(&self, path: &Path) -> Result<bool> {
            let routes = (&self.get_routes)(self, path)?;

            if self.generate_router_code.is_none() {
                return Err(anyhow!("Couldn't generate router code because generate_router_code function is not defined"));
            }

            let code_vector = (&self.generate_router_code.unwrap())(self, routes)?;

            let code = (&self.formatter)(self, code_vector)?;

            (&self.writer)(self, code, path);

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
