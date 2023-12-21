use std::{io::Write, fs, path::Path};

use anyhow::{Result, anyhow};

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

    pub fn exe<T>(&self, log_to_stdout: bool, log_to_file: bool, log_folder: T) -> Result<String>
    where 
        String: From<T>
    {
        // Make this windows friendly
        println!("Attempting to compile and run {} client framework", self.framework);
        let mut cmd = std::process::Command::new("sh");
        cmd.arg("src/client_compiler/exe.sh");
        cmd.arg(&self.build_cmd);
        cmd.args(&self.args);

        let output = cmd.output()?;

        if log_to_stdout {
            std::io::stdout().write_all(&output.stdout)?;
            std::io::stdout().write_all(&output.stderr)?;
        }

        // Change this to file piping on the exe script
        if log_to_file {
            let str_folder_path: String = log_folder.into();
            let log_folder_path = Path::new(&str_folder_path);
            let log_err_file_path = log_folder_path.join("stderr.log");
            let log_out_file_path = log_folder_path.join("stdout.log");

            let result = fs::create_dir(log_folder_path);

            if result.is_err() {
                let ferr = result.err().unwrap().to_string().to_lowercase();
                let match_results: Vec<&str> = ferr.matches("file exists").collect();

                if match_results.len() < 1 {
                    return Err(anyhow!("Couldn't create log folder at {}: {}", str_folder_path, ferr));
                }
            }

            fs::write(log_out_file_path, &output.stdout)?;
            fs::write(log_err_file_path, &output.stderr)?;
        }

        Ok(String::from_utf8(output.stdout)?)
    }
}