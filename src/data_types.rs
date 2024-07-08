use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Client {
    pub id: Option<u32>,
    pub client_name: String,
    pub birth_date: String,
    pub document_number: String,
    pub country: String,
    pub balance: Option<f32>,
}

#[derive(Deserialize)]
pub struct Credit {
    pub id: u32,
    pub credit_amount: f32,
}

#[derive(Deserialize)]
pub struct Debit {
    pub id: u32,
    pub debit_amount: f32,
}

#[derive(Serialize)]
pub struct Balance {
    pub error: Option<String>,
    pub id: u32,
    pub balance: f32,
}
