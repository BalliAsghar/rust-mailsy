use clap::Command;

fn main() {
    let matches = Command::new("mail")
        .version("0.1.0")
        .author("BalliAsghar")
        .about("Send email")
        // subcommand
        .subcommand(Command::new("gen").about("Generate a new disposable email address"))
        .subcommand(Command::new("del").about("Generate a new disposable email address"))
        .subcommand(Command::new("mal").about("Generate a new disposable email address"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("gen") {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(gen());
    } else if let Some(_) = matches.subcommand_matches("del") {
        del();
    } else if let Some(_) = matches.subcommand_matches("mal") {
        mal();
    } else {
        println!("No subcommand was used");
    }
}

async fn gen() {
    // use reqwest to call the api and get the response
    let client = reqwest::Client::new();

    let res = client
        .get("https://api.randomuser.me/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36")
        .send()
        .await.unwrap();

    // parse the response
    let body = res.text().await.unwrap();

    // parse the json
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();

    // get the email
    let email = json["results"][0]["email"].as_str().unwrap();

    // print the email
    println!("{}", email);
}

fn del() {}

fn mal() {}
