pub mod go {
    use anyhow::Result;
    use regex::Regex;

    use crate::server_compiler::server_compilers::{Route, ServerCompiler, create_safe_file_variable};

    // import var, import path
    const IMPORT_TEMPLATE: &str = "{} \"{}\"";
    // parent router, method, router path, import var
    const ROUTER_LINE_TEMPLATE: &str = "{}.{}({}, {})";
    //
    const ROUTER_GROUP_TEMPLATE: &str = "{} := {}.Group({}); {";

    fn generate_code(sc: &ServerCompiler, routes: Vec<Route>) -> Result<Vec<String>> {
        let imports: Vec<String> = vec![];
        let router_code: Vec<String> = vec![];

        for route in routes {
            let (var, import_route) = create_import_var(&route.get_path());
            println!("{}", format!("{}.{}({}, {})", "pvar", {route.get_method()}, {route.get_path()}, var + "." + &route.get_method().to_string()));
        }

        Ok(router_code)
    }

    // takes in a file path, returns import var and import path
    fn create_import_var(route: &str) -> (String, String) {
        let route_import_path = format!("routes/{}", route).replace("\\", "/").replace("//", "/");
        (create_safe_file_variable(&route_import_path), route_import_path)
    }

    pub fn get_lang() -> ServerCompiler {
        let mut lang = ServerCompiler::new("golang");
        lang.set_file_extension("go");
        lang.set_code_generator(generate_code);
        lang
    }
}
