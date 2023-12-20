use anyhow::Result;

pub struct ClientCompiler {
    framework: String,
    build_cmd: String,
    args: Vec<String>,
}

impl ClientCompiler {
    pub fn new<A, B, C>(framework: A, build_cmd: C, args: B) -> ClientCompiler
    where
        Vec<String>: From<B>,
        String: From<A>,
        String: From<C>,
    {
        ClientCompiler {
            framework: framework.into(),
            build_cmd: build_cmd.into(),
            args: args.into()
        }
    }

    pub fn exe(&self) -> Result<String> {
        let mut cmd = std::process::Command::new(&self.build_cmd);
        cmd.args(&self.args);

        let output = cmd.output()?;

        Ok(String::from_utf8(output.stdout).unwrap())
    }
}