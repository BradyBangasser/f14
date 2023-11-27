pub mod server_compilers {
    pub struct Route<'a> {
        method: &'a str,
        path: &'a str,
        
    }

    impl std::fmt::Display for Route<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}: {}", self.method, self.path)
        }
    }

    pub trait Compiler {
        fn create_router_code<'a>(routes: &'a [Route], code: &'a str) -> &'a str;
    }
}