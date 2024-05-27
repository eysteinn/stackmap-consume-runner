use tokio_postgres::{Client, NoTls, Error};
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;
use std::sync::Arc;

pub static DB_CLIENT: OnceCell<Arc<Mutex<Client>>> = OnceCell::new();

pub async fn init_db(database_url: &str) -> Result<Arc<Mutex<Client>>, Error> {
    if let Some(client) = DB_CLIENT.get() {
        return Ok(Arc::clone(client));
    }

    let (client, connection) = tokio_postgres::connect(database_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let arc_client = Arc::new(Mutex::new(client));
    DB_CLIENT.set(Arc::clone(&arc_client)).unwrap();
    println!("Database connection established");
    Ok(arc_client)
}
