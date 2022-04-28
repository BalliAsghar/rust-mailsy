use clap::Command;
use colored::*;
use dirs;
use fastrand;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use std::{
    fs::File,
    future::Future,
    io::{Read, Write},
    path::Path,
};
use tokio::runtime::Runtime;

// CONSTANTS
#[allow(dead_code)]
const API_ENDPOINT: &str = "https://api.mail.tm";

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
struct Config {
    token: String,
    email_address: String,
    account_creation_date: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct DomainResponse {
    #[serde(rename = "hydra:member")]
    domain: Vec<Domain>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Domain {
    domain: String,
    #[serde(rename = "isActive")]
    is_active: bool,
}
#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    violations: Vec<Violations>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Violations {
    message: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct AuthResponse {
    token: String,
    id: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct AuthError {
    code: u32,
    message: String,
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
        let mut file = File::create(&config_file_path).unwrap();
        create_config_file(&mut file).await;
    }

    // read the file
    let mut file = File::open(&config_file_path).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    // deserialize the file
    let config: Config = toml::from_str(&contents).unwrap();

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
#[allow(dead_code)]
async fn genrate_new_email_address(_file: &mut File) {
    // create the client
    let client = reqwest::Client::new();

    // concatenate the api endpoint
    let endpoint = format!("{}/domains", API_ENDPOINT);

    // create the request
    let response = client.get(endpoint).send().await.unwrap();

    // deserialize the response
    let domain_response: DomainResponse = response.json().await.unwrap();
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
        let error_response: ErrorResponse = response.json().await.unwrap();
        println!("{}", error_response.violations[1].message.red());
        return;
    }

    // Get JWT token
    let token_request = client.post(format!("{}/token", API_ENDPOINT)).json(&json!({
        "address": email_address,
        "password": password,
    }));

    // send the request
    let token_request = token_request.send().await.unwrap();

    println!("{}", token_request.text().await.unwrap());
}

async fn create_config_file(file: &mut File) {
    let config = Config {
        token: "".to_string(),
        email_address: "".to_string(),
        account_creation_date: "".to_string(),
    };
    let toml = toml::to_string(&config).unwrap();

    // write to file
    file.write_all(toml.as_bytes()).unwrap();

    println!("{}", "Config file created".green());
}
