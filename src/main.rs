mod install_dependencies;

use clap::{command, Arg, Command};
use install_dependencies::install_packages;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds packages to your project")
                .arg(Arg::new("packages").required(true).min_values(1)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", packages)) => {
            let package_names: Vec<&str> = packages.values_of("packages").unwrap().collect();
            println!("'myapp add' was used, name is: {package_names:?}",);
            install_packages(package_names);
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
