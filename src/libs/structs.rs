use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Config {
    pub token: String,
    pub email_address: String,
    pub password: String,
    pub account_creation_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DomainResponse {
    #[serde(rename = "hydra:member")]
    pub domain: Vec<Domain>,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Domain {
    pub domain: String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ErrorResponse {
    pub violations: Vec<Violations>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Violations {
    pub message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AuthResponse {
    pub token: String,
    pub id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AuthError {
    pub code: u32,
    pub message: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AccountResponse {
    pub address: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TokenResponse {
    pub token: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MailsResponse {
    #[serde(rename = "hydra:member")]
    pub mail: Vec<Mail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Mail {
    pub id: String,
    pub from: From,
    pub subject: String,
    pub intro: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct From {
    pub address: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MailErrorResponse {
    pub code: u32,
    pub message: String,
}
