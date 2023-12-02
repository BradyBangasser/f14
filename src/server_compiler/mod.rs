mod default_formatter;
mod default_generate_router_code;
mod default_get_routes;
mod default_error_handler;
mod default_writer;

pub mod server_compilers {
    use std::path::Path;

    use super::{
        default_formatter::default_formatter,
        default_generate_router_code::default_generate_router_code,
        default_get_routes::default_get_routes, default_writer::default_writer, default_error_handler::default_error_handler,
    };

    #[derive(Clone)]
    pub struct Route {
        method: String,
        path: String,
    }

    impl Route {
        pub fn new(method: &str, path: &Path) -> Self {
            Route {
                method: method.to_string().to_uppercase(),
                path: path
                    .to_str()
                    .expect("Path couldn't be converted to string")
                    .to_string(),
            }
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
        get_routes: fn(&str) -> Result<Vec<Route>, String>, // This many need to have a customer error type
        generate_router_code: fn(Vec<Route>) -> Result<Vec<String>, String>, // And this too
        formatter: fn(Vec<String>) -> Result<String, String>, // And this
        error_handler: fn(String),
        writer: fn(String, &str),
        pub file_extension: Option<String>,
        pub detect: Option<fn(&str) -> bool>
    }

    impl ServerCompiler {
        pub fn compile(&self, path: &str) {
            let routes = (&self.get_routes)(path);
            
            if routes.is_err() {
                (self.error_handler)(routes.err().unwrap());
                return
            }

            let code_vector = (&self.generate_router_code)(routes.unwrap());

            if code_vector.is_err() {
                (self.error_handler)(code_vector.err().unwrap());
                return
            }
            let code = (&self.formatter)(code_vector.unwrap());

            if code.is_err() {
                (self.error_handler)(code.err().unwrap());
                return
            }

            (&self.writer)(code.unwrap(), path);
        }

        pub fn new(lang_name: &str) -> Self {
            ServerCompiler {
                language: lang_name.to_uppercase().to_string(),
                get_routes: default_get_routes,
                generate_router_code: default_generate_router_code,
                formatter: default_formatter,
                writer: default_writer,
                error_handler: default_error_handler,
                file_extension: None,
                detect: None,
            }
        }

        pub fn set_file_extension(&mut self, extension: &str) {
            self.file_extension = Some(extension.to_string());
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
