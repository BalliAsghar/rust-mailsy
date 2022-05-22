use colored::Colorize;
use serde_json::json;
use std::{fs::File, future::Future, io::Write, path::Path};
use tokio::runtime::Runtime;

use crate::libs;

// CONSTANTS
const API_ENDPOINT: &str = "https://api.mail.tm";

pub(crate) fn asyn_runtime(async_fn: impl Future<Output = ()>) {
    let rt = Runtime::new().unwrap();
    rt.block_on(async_fn);
}

pub(crate) async fn create_config_file(file: &mut File) {
    let config = libs::structs::Config {
        token: "".to_string(),
        email_address: "".to_string(),
        account_creation_date: "".to_string(),
    };
    let toml = toml::to_string(&config).unwrap();

    // write to file
    file.write_all(toml.as_bytes()).unwrap();

    println!("{}", "Config file created".green());
}

pub async fn write_config_file(
    email_address: String,
    account_creation_date: String,
    token: String,
) {
    let config = libs::structs::Config {
        token,
        email_address,
        account_creation_date,
    };
    let toml = toml::to_string(&config).unwrap();

    // write to file
    let config_file_path = Path::new(&dirs::home_dir().unwrap()).join(".mailsy.toml");
    let mut file = File::create(&config_file_path).unwrap();
    file.write_all(toml.as_bytes()).unwrap();

    println!("{}", "Config file updated".green());
}

pub async fn genrate_new_email_address(_file: &mut File) {
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

pub async fn get_token(email_address: String, password: String) -> String {
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
