extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("get")
                .about("Get a value from a given key")
                .arg(
                    Arg::with_name("key")
                        .takes_value(true)
                        .required(true)
                        .value_name("KEY")
                        .help("the key to search for"),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("set a key/value pair")
                .arg(
                    Arg::with_name("key")
                        .takes_value(true)
                        .required(true)
                        .value_name("KEY")
                        .help("the key to set"),
                )
                .arg(
                    Arg::with_name("value")
                        .takes_value(true)
                        .required(true)
                        .value_name("VALUE")
                        .help("the value to set"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("remove a key")
                .arg(
                    Arg::with_name("key")
                        .takes_value(true)
                        .required(true)
                        .value_name("KEY")
                        .help("the key to remove"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
