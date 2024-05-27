//use postgres::{Client, Error};
//use tokio_postgres::{Client, Error, types::Json};
use tokio_postgres::{Client, NoTls, Error};
use std::error::Error as StdError;
use std::fmt;
use std::sync::{Arc, MutexGuard};
use serde_json::json;
use tokio_postgres::types::ToSql;
use tokio::sync::Mutex;

pub async fn write_to_files_table(
    project: &str,
    uuid: &str,
    filename: &str,
    metadata: &str,
    //db: &Client,
    //db: MutexGuard<Client>,
    db_client: &Arc<Mutex<Client>>

) -> Result<(), Box<dyn StdError>> {
    let schema = sanitize_schema_name(&format!("project_{}", project))?;
    let cmd = format!(
        //"INSERT INTO {}.files (uuid, filename, metadata) VALUES ($1, $2, $3) RETURNING uuid;",
        "INSERT INTO {}.files (filename, metadata) VALUES ($1, $2) RETURNING uuid;",
        schema
    );
    println!("Cmd: {}", cmd);

    // Note: We assume metadata is already sanitized or doesn't need sanitization.
    // You may need to sanitize metadata if it comes from an untrusted source.
    
    let v: serde_json::Value = json!({
        "size": 1024,
        "type": "text"
    });
    //let metadata_json = Json(v);
    let db = db_client.lock().await;
    println!("Got DB");
    //let lines =  db.execute(&cmd, &[&&filename, &metadata]).await?;
    let lines =  db.execute(&cmd, &[&&filename, &v]).await?;
    println!("Added {} lines", lines);
    Ok(())
}

fn sanitize_schema_name(name: &str) -> Result<String, Box<dyn StdError>> {
    // Sanitization logic, if needed, goes here.
    // Currently, we're just returning the name as is.
    Ok(name.to_owned())
}
