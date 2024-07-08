use actix_web::{web, App, HttpServer};
use std::sync::{atomic::AtomicU32, Mutex};

mod data_types;
mod endpoints;

static USER_ID: AtomicU32 = AtomicU32::new(0);
static FILE_COUNTER: AtomicU32 = AtomicU32::new(0);

struct AppState {
    clients_db: Mutex<Vec<data_types::Client>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        clients_db: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(endpoints::new_client)
            .service(endpoints::new_credit_transaction)
            .service(endpoints::new_debit_transaction)
            .service(endpoints::client_balance)
            .service(endpoints::store_balances)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
