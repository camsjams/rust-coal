use clap::{App, Arg};

pub fn build<'a, 'b>() -> App<'a, 'b> {
    App::new("")
        // package metadata from Cargo
        .name(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            App::new("serve")
                .about("Serve project for local development")
                .arg(
                    Arg::with_name("source")
                        .help("the folder to use in Coal")
                        .default_value("src")
                        .index(1)
                )
                .arg(
                    Arg::with_name("port")
                        .help("the port to run the dev server on")
                        .default_value("8041")
                        .index(2)
                )
        )
        .subcommand(
            App::new("build")
                .about("Build project for deployment")
                .arg(
                    Arg::with_name("source")
                        .help("the folder to use in Coal")
                        .default_value("src")
                        .index(1)
                )
                .arg(
                    Arg::with_name("dest")
                        .help("the folder to compiled public assets to")
                        .default_value("dist")
                        .index(2)
                )
        )
}
