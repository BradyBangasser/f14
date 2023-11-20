pub mod server_compilers {
    pub struct Route {

    }

    pub trait Compiler {

        fn create_router_code<'a>(routes: &'a [Route], code: &'a str) -> &'a str;
    }

    pub struct GoAeroCompiler();

    impl Compiler for GoAeroCompiler {
        fn create_router_code<'a>(routes: &'a [Route], code: &'a str) -> &'a str {
            return code
        }
    }

}