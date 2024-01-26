#![allow(dead_code, unused)]

use serde::Deserialize;
use std::{env, fs};
use std::error::Error;
use std::process::exit;

#[derive(Debug, Deserialize)]
struct Software {
    name: String,
    os: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    software: Vec<Software>,
}

fn main() {

    let desired_software_list = match get_desired_software() {
        Ok(list) => list,
        Err(e) => {
            println!("Error fetching desired software list: {:?}", e);
            return;
        }
    };

    let installed_software_list = match get_installed_software_list() {
        Ok(list) => list,
        Err(e) => {
            println!("Error fetching installed software list: {:?}", e);
            return;
        }
    };
    println!("{:?}", installed_software_list);

    for software in desired_software_list {
        if installed_software_list.contains(&software) {
            println!("{} is already installed.", software);
        } else {
            println!("Installing {}...", software);
            // Call your installation function here
            install_software(&software);
        }
    }


  
}


fn get_installed_software_list() -> Result<Vec<String>, Box<dyn Error>> {
    let mut installed_software = Vec::new();
    let apps = installed::list()?;
    for app in apps {
        let name = app.name();
        installed_software.push(name.to_string());
    }
    Ok(installed_software)
}

fn get_desired_software() -> Result<Vec<String>, Box<dyn Error>> {
    let file = "software_list.toml";
    let contents = fs::read_to_string(file)?;

    let config: Config = toml::from_str(&contents)?;
    let current_os = env::consts::OS;

    let desired_software_list: Vec<String> = config.software
        .into_iter()
        .filter(|s| s.os.contains(&current_os.to_string()))
        .map(|s| s.name)
        .collect();

    Ok(desired_software_list)
}

fn install_software(software: &str) {
    println!("TODO - implement install_software to install {software}")
}