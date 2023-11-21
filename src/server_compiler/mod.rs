pub mod server_compilers {
    pub struct Route<'a> {
        method: &'a str,
        path: &'a str,
        
    }

    pub trait Compiler {
        fn create_router_code<'a>(routes: &'a [Route], code: &'a str) -> &'a str;
    }
}