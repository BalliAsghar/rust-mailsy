use colored::Colorize;
use std::{fs::File, future::Future, io::Write};
use tokio::runtime::Runtime;

use crate::libs;

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
