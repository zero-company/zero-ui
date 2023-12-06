use std::process::Command;

fn main() {
    let directory = "./";
    println!("cargo:rerun-if-changed={}", directory);

    Command::new("trunk")
        .current_dir(directory)
        .args(["build"])
        .spawn()
        .unwrap_or_else(|err| panic!("Failed to execute build-zero-ui: {}", err));
}

