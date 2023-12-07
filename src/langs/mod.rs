pub mod go;
pub mod js;

pub mod langs {
    use std::path::Path;

    use crate::langs::go;
    use crate::server_compiler::server_compilers::ServerCompiler;

    static mut SELECTED_LANG: Option<ServerCompiler> = None;
    static mut LANGS: Vec<ServerCompiler> = vec![];
    pub fn register_lang(lang: ServerCompiler) -> bool {
        if lang.detect.is_none() && lang.file_extension.is_none() {
            return false
        }

        unsafe { LANGS.push(lang) };
        true
    }

    pub fn get_registered_langs() -> Vec<ServerCompiler> {
        let langs_copy = unsafe { LANGS.to_vec() };
        langs_copy
    }

    pub fn register_default_langs() {
        register_lang(go::go::get_lang());
    }

    pub fn detect_lang(main_file_path: &str) -> Result<ServerCompiler, String> {
        let registered_langs = unsafe { LANGS.clone() };

        let path = Path::new(main_file_path);
        let extention = path.extension().expect("The main_file_path needs to have an extention").to_str().expect("Error converting os str to str").to_string();
        for lang in registered_langs {
            if lang.detect.is_some() {
                if lang.detect.unwrap()(main_file_path) {
                    set_lang(&lang);
                    return Ok(lang.clone());
                }
            } else {
                let lang_ext = lang.clone().file_extension.expect("All langs must have a file extention or detector function");

                if lang_ext == extention {
                    set_lang(&lang);
                    return Ok(lang.clone());
                }
            }
        };

        Err("Could not auto detect language".to_string())
    }

    pub fn set_lang(lang: &ServerCompiler) {
        unsafe { SELECTED_LANG = Some(lang.clone()) }
    }

    pub fn get_lang() -> Option<ServerCompiler> {
        unsafe { SELECTED_LANG.clone() }
    }
}
