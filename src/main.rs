use clap::Command;
use colored::*;
use dirs;
use fastrand;
#[allow(unused_imports)]
use serde_json::json;
use std::{fs::File, io::Read, path::Path};
mod libs;

// CONSTANTS
#[allow(dead_code)]
const API_ENDPOINT: &str = "https://api.mail.tm";

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
    // Temporary Fn
    delete_config().await;

    // Load the config file
    let config_file_path = Path::new(&dirs::home_dir().unwrap()).join(".mailsy.toml");

    // if file does not exist, then create the file
    if !config_file_path.exists() {
        let mut file = File::create(&config_file_path).unwrap();
        libs::utils::create_config_file(&mut file).await;
    }

    // read the file
    let mut file = File::open(&config_file_path).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    // deserialize the file
    let config: libs::structs::Config = toml::from_str(&contents).unwrap();

    // if the token is not epmty, then we have a valid config file.
    if !config.token.is_empty() {
        println!("Account already created {}", config.email_address.green());
        return;
    }

    genrate_new_email_address(&mut file).await;
}

#[allow(dead_code)]
async fn del() {}
#[allow(dead_code)]
async fn mail() {}

async fn read() {}

async fn genrate_new_email_address(_file: &mut File) {
    // create the client
    let client = reqwest::Client::new();

    // concatenate the api endpoint
    let endpoint = format!("{}/domains", API_ENDPOINT);

    // create the request
    let response = client.get(endpoint).send().await.unwrap();

    // deserialize the response
    let domain_response: libs::structs::DomainResponse = response.json().await.unwrap();
    // grab the first domain
    let domain = &domain_response.domain[0].domain;
    // generate a random address
    let mut email_address: String = std::iter::repeat_with(fastrand::alphanumeric)
        .take(8)
        .collect();

    // format the email address
    email_address = format!("{}@{}", email_address, domain);

    // generate a random password
    let password: String = std::iter::repeat_with(fastrand::alphanumeric)
        .take(8)
        .collect();

    // create the request
    let request = client
        .post(format!("{}/accounts", API_ENDPOINT))
        .json(&json!({
            "address": email_address,
            "password": password,
        }));

    // send the request
    let response = request.send().await.unwrap();

    // check if the request was not successful
    if !response.status().is_success() {
        // deserialize the response
        let error_response: libs::structs::ErrorResponse = response.json().await.unwrap();
        println!("{}", error_response.violations[1].message.red());
        return;
    }

    // deserialize the response
    let auth_response: libs::structs::AccountResponse = response.json().await.unwrap();

    // get token
    let token = get_token(email_address, password).await;

    // write to file
    libs::utils::write_config_file(auth_response.address, auth_response.created_at, token).await;
}

async fn get_token(email_address: String, password: String) -> String {
    // CRATE THE CLIENT
    let client = reqwest::Client::new();

    // build the request
    let request = client.post(format!("{}/token", API_ENDPOINT)).json(&json!({
        "address": email_address,
        "password": password,
    }));

    // get response
    let response = request.send().await.unwrap();

    // check if the request was not successful
    if !response.status().is_success() {
        println!("{:?}", response);
        return "".to_string();
    }

    // deserialize the response
    let auth_response: libs::structs::TokenResponse = response.json().await.unwrap();

    println!("Account created {}", auth_response.token);

    return auth_response.token;
}

async fn delete_config() {
    // Load the config file
    let config_file_path = Path::new(&dirs::home_dir().unwrap()).join(".mailsy.toml");

    // if file does exits, then delete it.
    if config_file_path.exists() {
        std::fs::remove_file(&config_file_path).unwrap();
    }
}
