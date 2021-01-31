use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand,
};

pub fn cli() -> ArgMatches<'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("check").about("Checks NOIDs").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .multiple(true)
                    .min_values(1)
                    .value_name("FILE")
                    .help("Checks a list of NOIDs stored in one or more files. One ID per line"),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("Output file")
                )
            .arg(
                Arg::with_name("workers")
                    .short("w")
                    .long("workers")
                    .value_name("NUMBER OF WORKERS")
                    .default_value("4")
                    .help("Sets the number of workers")
                ),
        )
        .subcommand(
            SubCommand::with_name("checksum").about("Computes the checksum char of NOIDs").arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .multiple(true)
                    .min_values(1)
                    .value_name("FILE")
                    .help("Computes the checksum char of a list of NOIDs stored in one or more file. One ID per line")
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("FILE")
                        .help("Output file")
                )
                .arg(
                    Arg::with_name("workers")
                        .short("w")
                        .long("workers")
                        .value_name("NUMBER OF WORKERS")
                        .default_value("4")
                        .help("Sets the number of workers")
                ),
        )
        .subcommand(
            SubCommand::with_name("ws").about("Starts the NCDA Web Service").arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .value_name("PORT")
                    .help("Sets the port used by the Web Service"),
            ),
        )
        .get_matches()
}
