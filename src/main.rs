use std::process::Command;

mod install_dependencies;

use install_dependencies::install_packages;

fn main() {
    install_packages(vec!["black", "virtualenv"]);
}
