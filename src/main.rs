use clap::Command;
use colored::*;
use dirs;
use std::{fs::File, io::Read, path::Path};
mod libs;

fn main() {
    let matches = Command::new("mail")
        .version("0.1.0")
        .author("BalliAsghar")
        .about("Send email")
        // subcommand
        .subcommand(Command::new("gen").about("Generate a new disposable email address"))
        .subcommand(Command::new("del").about("Generate a new disposable email address"))
        .subcommand(Command::new("mail").about("Generate a new disposable email address"))
        .subcommand(Command::new("read").about("Generate a new disposable email address"))
        .get_matches();

    match matches.subcommand() {
        Some(("gen", _)) => {
            libs::utils::asyn_runtime(gen());
        }
        Some(("del", _)) => {
            libs::utils::asyn_runtime(del());
        }
        Some(("mail", _)) => {
            libs::utils::asyn_runtime(mail());
        }
        Some(("read", _)) => {
            libs::utils::asyn_runtime(read());
        }
        _ => {} // Either no subcommand or one not tested for...
    }
}

async fn gen() {
    // Load the config file
    let config_file_path = Path::new(&dirs::home_dir().unwrap()).join(".mailsy.toml");

    // if file does not exist, then create the file
    if !config_file_path.exists() {
        let mut file = File::create(&config_file_path).unwrap();
        libs::utils::create_config_file(&mut file).await;
        libs::utils::genrate_new_email_address(&mut file).await;
        return;
    }

    // read the file
    let mut file = File::open(&config_file_path).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    // deserialize the file
    let config: libs::structs::Config = toml::from_str(&contents).unwrap();

    // if the email is not epmty, that means the user has already generated an email address
    if !config.email_address.is_empty() {
        println!("Account already created {}", config.email_address.green())
    }
}

#[allow(dead_code)]
async fn del() {}
#[allow(dead_code)]
async fn mail() {}

async fn read() {}

async fn _delete_config() {
    // Load the config file
    let config_file_path = Path::new(&dirs::home_dir().unwrap()).join(".mailsy.toml");

    // if file does exits, then delete it.
    if config_file_path.exists() {
        std::fs::remove_file(&config_file_path).unwrap();
    }
}
