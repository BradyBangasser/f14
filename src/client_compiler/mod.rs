pub struct ClientCompiler {
    framework: String,
    exe_cmd: String,
    args: Vec<String>,
}

impl ClientCompiler {
    pub fn new(framework: &str, exe_cmd: &str, args: Vec<String>) -> ClientCompiler {
        ClientCompiler {
            framework: String::from(framework),
            exe_cmd: String::from(exe_cmd),
            args
        }
    }

    pub fn exe(&self) {

    }
}