pub mod go {
    use anyhow::Result;

    use crate::server_compiler::server_compilers::{ServerCompiler, Route};


    fn generate_code(sc: &ServerCompiler, routes: Vec<Route>) -> Result<Vec<String>> {
        let imports: Vec<String> = vec![];
        let router_code: Vec<String> = vec![];

        for route in routes {
            println!("{}", route)
        }

        Ok(router_code)
    }

    pub fn get_lang() -> ServerCompiler {
        let mut lang = ServerCompiler::new("golang");
        lang.set_file_extension("go");
        lang.set_code_generator(generate_code);
        lang
    }
}
