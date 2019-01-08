use clap::{Arg, App, SubCommand};

use std::path::Path;

const PATH: &str = "/Users/carllerche/Code/Tokio/tokio";
// const PATH: &str = "/Users/carllerche/Code/Tokio/mio";

mod action;
mod cargo;
mod changelog;
mod config;
mod git;
mod manifest;
mod package;
mod workspace;

use crate::config::Config;
use crate::workspace::Workspace;

fn main() {
    let matches = App::new("Ship It!")
        .version("0.1.0")
        .author("Carl Lerche <me@carllerche.com>")
        .subcommand({
            SubCommand::with_name("check")
                .about("Check for project compliance")
        })
        .subcommand({
            SubCommand::with_name("init")
                .about("Initialize a project for shipit")
        })
        .subcommand({
            SubCommand::with_name("status")
                .about("Show the release status")
        })
        .get_matches();

    let root = Path::new(PATH);

    let workspace = Workspace::load(root);
    let config = Config::load(&workspace);

    match matches.subcommand() {
        ("check", Some(sub_matches)) => {
            action::check(&workspace, &config.unwrap());
        }
        ("init", Some(sub_matches)) => {
            let config = match config {
                Ok(config) => Some(config),
                Err(ref err) if err.is_not_found() => {
                    None
                }
                Err(_) => {
                    unimplemented!();
                }
            };

            action::init(&workspace, config.as_ref());
        }
        ("status", Some(sub_matches)) => {
            unimplemented!();
        }
        _ => {
            unimplemented!();
        }
    }
}
