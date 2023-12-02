mod server_compiler;
mod langs;
pub mod f14fs;

use std::path::Path;

use clap::{Command, Arg};
use langs::langs as lang;

fn main() {
    let mut cmd = Command::new("tomcat");
    cmd = cmd.args([
        Arg::new("test")
        .short('t')
        .value_name("name")
        .long("test")
    ]);

    let matches = cmd.get_matches();
    let test = matches.get_one::<String>("test").expect("No Test?");

    lang::register_default_langs();
    let selected_lang = lang::detect_lang("test/go/main.go").ok().expect("No language could be loaded");
    selected_lang.compile("test/go");
    println!("{}", test)
}
