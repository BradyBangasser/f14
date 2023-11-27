pub mod go {
    use crate::server_compiler::server_compilers::{Compiler, Route};

    pub struct GoAeroCompiler();

    impl Compiler for GoAeroCompiler {
        fn create_router_code<'a>(routes: &'a [Route], code: &'a str) -> &'a str {
            for route in routes {
                println!("{}", route);
            }
            return code;
        }
    }
}
