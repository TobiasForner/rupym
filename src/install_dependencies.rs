use std::process::Command;

pub fn install_packages(package_names: Vec<&str>) {
    let package_names = package_names.join(" ");
    let test = package_names.to_string();
    println!(" installing {test}");
    let _base_command = if cfg!(target_os = "windows") {
        Command::new("pwsh")
            .args(["/C", &format!("pip install {test}")])
            .arg(package_names)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("pip install")
            .arg(package_names)
            .output()
            .expect("failed to execute process")
    };
    let out = String::from_utf8(_base_command.stdout).unwrap();
    println!("{out}");
}
