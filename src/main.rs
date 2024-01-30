#![allow(unused)]
extern crate toml;

use color_eyre::eyre::Result;

use std::{env, fs, num::NonZeroI128, process::Command, vec};
use toml::{Value, de::Error};

fn main() -> Result<()> {
    color_eyre::install()?;

    let desired_software_list = get_desired_software_list();

    if let Some(installed_software_list) = get_installed_list() {
        for software in desired_software_list {
            is_installed(&software, &installed_software_list);
        } 
    }

    Ok(())
}

fn get_desired_software_list() -> Vec<String> {

    let mut desired_software_list = Vec::new();
    let toml_str = fs::read_to_string("software_list.toml").expect("failed to read toml file");
    let toml_value: Result<Value, Error> = toml::from_str(&toml_str);

    if let Ok(toml_value) = toml_value {
        if let Some(software_list) = toml_value.get("software").and_then(Value::as_array) {
            let current_os = std::env::consts::OS.to_lowercase();

            for software in software_list {
                if let Some(name) = software.get("name").and_then(Value::as_str) {
                    if let Some(os_list) = software.get("os").and_then(Value::as_array) {
                        if os_list.iter().any(|os| os.as_str() == Some(&current_os)) {
                            desired_software_list.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    desired_software_list
}

fn get_installed_list() -> Option<String> {
    let os = env::consts::OS;
    match os {
        "windows "=> {
            let output = Command::new("winget")
            .arg("list")
            .output()
            .expect("Failed to execute winget");
            let installed_software = String::from_utf8_lossy(&output.stdout);
            println!("{}", installed_software);
            Some(installed_software.to_string())
        }
        _ => {
            println!("OS: {} not supported yet", os);
            None
        }
    }
    
}

fn is_installed(software_name: &str, installed_list: &String) -> bool {

    if installed_list.contains(software_name) {
        println!("{} is installed!", software_name);
        true
    } else {
        println!("{} is not installed.", software_name);
        false
    }
    
}