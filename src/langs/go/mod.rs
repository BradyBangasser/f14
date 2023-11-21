pub mod golang {
    pub struct GoAeroCompiler();

    impl Compiler for GoAeroCompiler {
        fn create_router_code<'a>(routes: &'a [Route], code: &'a str) -> &'a str {
            return code;
        }
    }
}
