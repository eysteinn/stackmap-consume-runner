use tokio_postgres::{Client, Error};
//use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::error::Error as StdError;
use std::fmt;
/*use std::sync::MutexGuard;
use tokio::sync::Mutex;*/
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn write_to_raster_table(
    project: &str,
    product: &str,
    coordinates: &str,
    file_uuid: Uuid,
    //file_uuid: &str,
    location: &str,
    timestamp: &str, //DateTime<Utc>,
    srs: &str,
    //db: &Client,
    db_client: &Arc<Mutex<Client>>
) -> Result<(), Box<dyn StdError>> {
    //let timestr = timestamp.to_rfc3339();
    let timestr = timestamp;
    //let id = Uuid::new_v4();
    let db = db_client.lock().await;

    println!("UUID:::: {}", file_uuid.to_string());
    /*let cmd = format!(
        "INSERT INTO project_{}.raster_geoms (uuid, location, src_srs, datetime, product, geom) \
         VALUES ('{}', '{}', '{}', '{}', '{}', ST_GeomFromText('MULTIPOLYGON ((({})))'));",
        project,
        file_uuid,
        location,
        srs,
        timestr,
        product,
        coordinates
    );*/
    let cmd = format!(
        "INSERT INTO project_{}.raster_geoms (uuid, location, src_srs, datetime, product, geom) \
         VALUES ('{}', '{}', '{}', '{}', '{}', ST_GeomFromText('MULTIPOLYGON ((({})))'));",
        project,
        file_uuid.to_string(),
        location,
        srs,
        timestr,
        product,
        coordinates
    );

    println!("Command {}", cmd);
    // Execute the command
    let lines = db.execute(&cmd, &[]).await?;
    println!("Added {} lines to project_{}.raster_geoms", lines, project);

    Ok(())
}