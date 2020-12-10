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
                        .short("s")
                        .long("source")
                        .takes_value(true)
                        .default_value("src")
                )
                .arg(
                    Arg::with_name("port")
                        .help("the port to run the dev server on")
                        .short("p")
                        .long("port")
                        .takes_value(true)
                        .default_value("8041")
                )
        )
        .subcommand(
            App::new("build")
                .about("Build project for deployment")
                .arg(
                    Arg::with_name("source")
                        .help("the folder to use in Coal")
                        .short("s")
                        .long("source")
                        .takes_value(true)
                        .default_value("src")
                )
                .arg(
                    Arg::with_name("dest")
                        .help("the folder to compiled public assets to")
                        .short("d")
                        .long("dest")
                        .takes_value(true)
                        .default_value("dist")
                )
                .arg(
                    Arg::with_name("version")
                        .help("the build to insert into html")
                        .short("v")
                        .long("version")
                        .takes_value(true)
                        .default_value(env!("CARGO_PKG_VERSION"))
                )
                .arg(
                    Arg::with_name("root")
                        .help("the folder root to insert before assets")
                        .short("r")
                        .long("root")
                        .takes_value(true)
                        .default_value("")
                )
        )
}
