use std::io::{Error, ErrorKind};
extern crate chrono;
extern crate clap;
extern crate fs_extra;
extern crate grass;
extern crate html_minifier;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod cli;
mod coal;
mod serve;
mod build;

fn normalize_path(path: String)  -> std::string::String {
    let last_char =  path.chars().last().unwrap();
    if last_char == '/' {
        return path;
    }

    path + "/"
}

fn main() -> std::io::Result<()>{
    let matches = cli::build().get_matches();
    if let Some(ref matches) = matches.subcommand_matches("serve") {
        serve::start( 
            normalize_path(matches.value_of("source").unwrap().to_string()), 
            matches.value_of("port").unwrap().to_string()
        )
    } else  if let Some(ref matches) = matches.subcommand_matches("build") {
        build::start( 
            normalize_path(matches.value_of("source").unwrap().to_string()), 
            normalize_path(matches.value_of("dest").unwrap().to_string()),
            matches.value_of("version").unwrap().to_string(),
            matches.value_of("root").unwrap().to_string(),
        )
    } else {
        Err(Error::new(ErrorKind::Other, "Invalid Mode"))
    }

}
