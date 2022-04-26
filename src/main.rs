use clap::Command;
use dirs;
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
    // TODO:
    // 1. first load the config file
    // 2. if the config file is not found, means user is generating a new email address for the first time
    // 3. if the config file is found, check if the user has already generated an email address
    // 4. if the user has already generated an email address, then just load the email address from the config file
    // 5. if the user has not generated an email address, then generate a new email address and save it to the config file
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
