use crate::{
    data_types::{Balance, Client, Credit, Debit},
    AppState, FILE_COUNTER, USER_ID,
};
use chrono::Datelike;
use std::io::{Result, Write};
use std::sync::{atomic::Ordering, MutexGuard};
use tokio::fs::OpenOptions;
use tokio::io;
use tokio::io::AsyncWriteExt;

use actix_web::{get, post, web, HttpResponse, Responder};

//POST “new_client”
#[post("/new_client")]
async fn new_client(add_client: web::Json<Client>, data: web::Data<AppState>) -> HttpResponse {
    let mut add_client = add_client.into_inner();

    let mut clients_db = data.clients_db.lock().unwrap();

    // Check for duplicated DN
    let already = clients_db
        .iter()
        .find(|already| already.document_number == add_client.document_number);
    if already.is_some() {
        return HttpResponse::Conflict().body(format!(
            "Client with DN {} already has an account",
            add_client.document_number
        ));
    }

    match add_client.id {
        Some(_) => return HttpResponse::BadRequest().body(format!("Invalid request data")),
        None => {
            // Increase User ID
            USER_ID.fetch_add(1, Ordering::SeqCst);
            add_client.id = Some(USER_ID.load(Ordering::SeqCst))
        }
    }

    match add_client.balance {
        Some(_) => return HttpResponse::BadRequest().body(format!("Invalid request data")),
        None => {
            // Set balance to 0 for new client
            add_client.balance = Some(0.0)
        }
    }

    let response = format!("New client created with ID {}", add_client.id.unwrap());

    // Add new client to database
    clients_db.push(add_client);

    HttpResponse::Ok().body(response)
}

// POST “new_credit_transaction”
#[post("/new_credit_transaction")]
async fn new_credit_transaction(
    add_credit: web::Json<Credit>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let mut clients_db = data.clients_db.lock().unwrap();
    balance_updater(&mut clients_db, add_credit.id, add_credit.credit_amount)
}

// POST “new_debit_transaction”
#[post("/new_debit_transaction")]
async fn new_debit_transaction(
    debit_sum: web::Json<Debit>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let mut clients_db = data.clients_db.lock().unwrap();
    balance_updater(&mut clients_db, debit_sum.id, -debit_sum.debit_amount)
}

// POST “store_balances”
#[post("/store_balances")]
async fn store_balances(data: web::Data<AppState>) -> impl Responder {
    let mut clients_db = data.clients_db.lock().unwrap();
    let mut ids_to_clean: Vec<u32> = Vec::new();
    let balance_file = get_balance_filename();

    for (index, client) in clients_db.iter().enumerate() {
        let mut balance_buf = Vec::<u8>::new();
        writeln!(
            balance_buf,
            "{} {}",
            client.id.unwrap(),
            client.balance.unwrap()
        )
        .expect("Data could not be written.");

        let mut create_file: bool = false;
        if index == 0 {
            create_file = true;
        }

        append_to_file(&balance_file, &balance_buf, create_file)
            .await
            .expect("Error at writing data to file.");

        ids_to_clean.push(client.id.unwrap());
    }

    // Add Zero balance mod function
    for id in ids_to_clean.iter() {
        balance_updater(&mut clients_db, *id, 0.0);
    }

    HttpResponse::Ok()
}

// GET “client_balance”
#[get("/client_balance/{user_id}")]
async fn client_balance(
    path_id: web::Path<u32>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let clients_db = data.clients_db.lock().unwrap();
    let required_id = path_id.into_inner();
    let client_credit: Vec<_> = clients_db
        .iter()
        .filter(|x| x.id.unwrap() == required_id)
        .collect();

    if client_credit.is_empty() {
        let error = Balance {
            id: required_id,
            balance: 0.0,
            error: Some(("Bad ID provided").to_string()),
        };
        Ok(web::Json(error))
    } else {
        let balance = Balance {
            id: required_id,
            balance: client_credit[0].balance.unwrap(),
            error: Some(("Processed correctly").to_string()),
        };
        Ok(web::Json(balance))
    }
}

// Client balance updater at Credit/Debit transactions
fn balance_updater(clients_db: &mut MutexGuard<Vec<Client>>, id: u32, amount: f32) -> HttpResponse {
    let client_credit: Vec<_> = clients_db.iter().filter(|x| x.id.unwrap() == id).collect();

    if client_credit.is_empty() {
        return HttpResponse::NotFound().body(format!("Client ID {} NOT found", id));
    } else {
        let updated_balance: f32;
        if amount == 0.0 {
            updated_balance = amount;
        } else {
            updated_balance = client_credit[0].balance.unwrap() + amount;
        }

        let updated_client = Client {
            id: Some(id),
            client_name: String::from(&client_credit[0].client_name),
            birth_date: String::from(&client_credit[0].birth_date),
            document_number: String::from(&client_credit[0].document_number),
            country: String::from(&client_credit[0].country),
            balance: Some(updated_balance),
        };

        let client_db_index = clients_db.iter().position(|x| x.id.unwrap() == id);

        match client_db_index {
            Some(idx) => {
                clients_db[idx] = updated_client;
                HttpResponse::Ok().body(format!("Client ID {} new balance {}", id, updated_balance))
            }
            None => {
                return HttpResponse::NotFound().body(format!("Client ID {} NOT found", id));
            }
        }
    }
}

// Write at balance file
async fn append_to_file(file_path: &str, data: &[u8], create_file: bool) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(create_file)
        .open(file_path)
        .await?;

    let data_length = data.len() as u32;
    let mut tmp_buffer = [0u8; 4];
    tmp_buffer.copy_from_slice(&data_length.to_le_bytes());
    file.write_all(&tmp_buffer).await?;

    file.write_all(data).await?;
    Ok(())
}

fn get_balance_filename() -> String {
    // Get date
    let current_date = chrono::Utc::now();
    let year = current_date.year().to_string();
    let mut month = current_date.month().to_string();
    let mut day = current_date.day().to_string();

    if current_date.month() < 10 {
        month.insert_str(0, "0");
    }
    if current_date.day() < 10 {
        day.insert_str(0, "0");
    }

    // Increase File Counter
    let file_counter = FILE_COUNTER.fetch_add(1, Ordering::SeqCst).to_string();

    return "/tmp/".to_string() + &day + &month + &year + "_" + &file_counter + ".DAT";
}
