use clap::Command;
use colored::*;
use dirs;
use serde::Deserialize;
use std::{
    fs::File,
    future::Future,
    io::{Read, Write},
    path::Path,
};
use tokio::runtime::Runtime;

// CONSTANTS
#[allow(dead_code)]
const API_ENDPOINT: &str = "https://api.mail.tm/";

#[derive(Debug, Deserialize)]
struct Config {
    token: String,
    email_address: String,
    account_creation_date: String,
}

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
            asyn_runtime(gen());
        }
        Some(("del", _)) => {
            asyn_runtime(del());
        }
        Some(("mail", _)) => {
            asyn_runtime(mail());
        }
        Some(("read", _)) => {
            asyn_runtime(read());
        }
        _ => {} // Either no subcommand or one not tested for...
    }
}

async fn gen() {
    // Load the config file
    let config_file_path = Path::new(&dirs::home_dir().unwrap()).join(".mailsy.toml");

    // if file does not exist, then create the file
    if !config_file_path.exists() {
        let mut file = File::create(config_file_path).unwrap();
        asyn_runtime(genrate_new_email_address(&mut file));
        return;
    }

    // if file exists, but the user already generated an email address, then just load the email address from the config file
    let mut file = File::open(config_file_path).unwrap();

    // read the config file and deserialize it
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let config: Config = toml::from_str(&contents).unwrap();

    if !config.email_address.is_empty() {
        println!(
            "Email already created! {}",
            config.email_address.green().bold()
        );
        return;
    }

    asyn_runtime(genrate_new_email_address(&mut file));
}

#[allow(dead_code)]
async fn del() {}
#[allow(dead_code)]
async fn mail() {}

async fn read() {
    // read the output.json file
    let mut file = std::fs::File::open("output.json").unwrap();

    // convert the file to json using serde
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let json: serde_json::Value = serde_json::from_str(&contents).unwrap();

    // pretty print the json
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

fn asyn_runtime(async_fn: impl Future<Output = ()>) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async_fn);
}

async fn genrate_new_email_address(file: &mut File) {
    println!("{}", "Generating new email address".green());
}
