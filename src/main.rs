use std::io::{Error, ErrorKind};
extern crate chrono;
extern crate clap;
extern crate grass;
extern crate pest;
extern crate html_minifier;
#[macro_use]
extern crate pest_derive;

mod cli;
mod coal;
mod serve;
mod build;

fn main() -> std::io::Result<()>{
    let matches = cli::build().get_matches();
    if let Some(ref matches) = matches.subcommand_matches("serve") {
        serve::start( 
            matches.value_of("source").unwrap().to_string(), 
            matches.value_of("port").unwrap().to_string()
        )
    } else  if let Some(ref matches) = matches.subcommand_matches("build") {
        build::start( 
            matches.value_of("source").unwrap().to_string(), 
            matches.value_of("dest").unwrap().to_string(),
            matches.value_of("version").unwrap().to_string(),
        )
    } else {
        Err(Error::new(ErrorKind::Other, "Invalid Mode"))
    }

}
