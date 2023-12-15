pub mod go {
    use anyhow::Result;
    use regex::Regex;

    use crate::server_compiler::server_compilers::{Route, ServerCompiler, create_safe_file_variable};

    // impl route grouping, middleware, static hosting, and multi func route handlers
    fn generate_code(_sc: &ServerCompiler, routes: Vec<Route>) -> Result<Vec<String>> {
        let mut mutatable_routes = routes.clone();
        mutatable_routes.sort_unstable();
        let mut imports: Vec<String> = vec![String::from("package routes"), String::from("import ("), String::from(r#""github.com/gin-gonic/gin""#)];
        let mut router_code: Vec<String> = vec!["func LoadRoutes(router *gin.Engine) {".to_string()];

        for route in mutatable_routes {
            let (var, import_route) = create_import_var(&route.get_path());
            if route.get_path() == "/" {
                router_code.push(format!(r#"{}.{}("/", {})"#, "router", route.get_method(), route.get_method()))
            } else {
                let import_line = format!(r#"{} "{}""#, var, import_route);

                if !imports.contains(&import_line) {
                    imports.push(import_line);
                }

                router_code.push(format!(r#"{}.{}("{}", {})"#, "router", {route.get_method()}, {route.get_path()}, var + "." + &route.get_method().to_string()));
            }
        }

        imports.push(String::from(")"));
        router_code.push(String::from("}"));

        imports.append(&mut router_code);

        Ok(imports)
    }

    // takes in a file path, returns import var and import path
    fn create_import_var(route: &str) -> (String, String) {
        let mut route_import_path = format!("test/routes/{}", route).replace("\\", "/").replace("//", "/");
        route_import_path = Regex::new(r"\/$").unwrap().replace(&route_import_path, "").into();
        (create_safe_file_variable(&route_import_path), route_import_path)
    }

    pub fn get_lang() -> ServerCompiler {
        let mut lang = ServerCompiler::new("golang");
        lang.set_file_extension("go").set_code_generator(generate_code).set_ignored_files(vec![String::from("build")]);

        lang
    }
}
