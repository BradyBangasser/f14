pub mod server_compiler;
pub mod client_compiler;
mod langs;
pub mod f14fs;

use std::path::Path;

// use clap::{Command, Arg};
use langs::langs as lang;
use client_compiler::ClientCompiler;

fn main() {
    // let mut cmd = Command::new("tomcat");
    // cmd = cmd.args([
    //     Arg::new("test")
    //     .short('t')
    //     .value_name("name")
    //     .long("test")
    // ]);

    // let matches = cmd.get_matches();
    // let _test = matches.get_one::<String>("test").expect("No Test?");

    lang::register_default_langs();
    let selected_lang = lang::detect_lang("test/server/go/main.go").ok().expect("No language could be loaded");
    let result = selected_lang.compile(Path::new("test/server/go/routes"));

    let cc = ClientCompiler::new("test", "ls", [String::from("-a")]);
    let client_result = cc.exe(true, true, "log");

    if client_result.is_ok() {
        println!("Client Build Success, result:\n{}", client_result.unwrap());
    } else {
        println!("Client Build Failed: {}", client_result.err().unwrap());
    }

    if result.is_ok() {
        println!("Build Success");
    } else {
        println!("Build Failed: {}", result.err().unwrap());
    }
}

