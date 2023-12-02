pub mod go {
    use crate::server_compiler::server_compilers::ServerCompiler;

    pub fn get_lang() -> ServerCompiler {
        let mut lang = ServerCompiler::new("golang");
        lang.set_file_extension("go");
        lang
    }
}
