use core::panic;
use std::process::Command;
use dialoguer::{MultiSelect, theme::ColorfulTheme};

fn run_docker(args: Vec<&str>) -> String {
    let output = Command::new("docker")
        .args(args)
        .output()
        .expect(&format!("failed to execute docker"));

    if !output.status.success() {
        let err = String::from_utf8(output.stderr)
                .map(|l| l.trim().to_string())
                .unwrap();

        panic!("{}", err)
    }

    String::from_utf8(output.stdout)
        .map(|l| l.trim().to_string())
        .expect("could not convert stdout to utf8")
}

fn main() {
    println!("  Select Docker images to delete, use arrow keys and space to select/unselect");
    let images_raw = run_docker(vec!["images", "-a"]);
    let images: Vec<&str> = images_raw.split("\n").skip(1).collect();

    let chosen = MultiSelect::with_theme(&ColorfulTheme::default()).items(&images).interact().unwrap();

    for selected_index in chosen {
        if let Some(id) = images[selected_index].split("   ").nth(2) {
            let out = run_docker(vec!["rmi", "-f", id.trim()]);
            println!("{}", out);
        }
    }
}
