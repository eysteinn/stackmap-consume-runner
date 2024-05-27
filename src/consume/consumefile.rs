use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use gdal::Dataset;

use std::error::Error;

use std::fs::File;
use std::io::BufReader;
use uuid::Uuid;

use crate::geo::shape;

use super::super::database;

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    project: String,
    filename: String,
    product: String,
    timestamp: String,
    uuid: String,
    filepath: String,
}


 pub async fn consume_file(folder: &str) -> Result<(), Box<dyn Error>> {

    // Open the JSON file
    let file = File::open("data/incoming/somehash/meta.json")?;
    let reader = BufReader::new(file);

    // Parse the JSON file into the Metadata struct
    let mut meta: Metadata = serde_json::from_reader(reader)?;
    if meta.filepath == "" {
        let filepath = Path::new(folder).join(&meta.filename);
        let filepath_str = filepath.to_str().ok_or("Unable to create path")?;
        meta.filepath = filepath_str.to_string();
    }
    // Print the parsed data
    println!("{:?}", meta);
  

    consume_tiff(&meta).await?;
    Ok(())
}

async fn consume_tiff(meta: &Metadata) -> Result<(), Box<dyn Error>> {

    println!("Processing TIFF file with the following metadata:");
    println!("Project: {}", meta.project);
    println!("Filename: {}", meta.filename);
    println!("Product: {}", meta.product);
    println!("Timestamp: {}", meta.timestamp);
    println!("UUID: {}", meta.uuid);
    println!("Filepath: {}", meta.filepath);
    
    let shape = shape::new_shape_4325(&meta.filepath)?;

    let coord_str = shape.get_coords_string();  
    println!("Coords: {}", coord_str);

    //let db = database::database::get_db().await?;
    //let db2 = database::database2::DB_CLIENT;
    let db_client = database::database::DB_CLIENT.get().expect("Database client is not initialized");
    println!("Got Database");
    //let client = db_client.lock().await;

    //let db = get_db().await?;
    
    // Lock the mutex to get a mutable reference to the client
    //let db_connection = db.lock().await;
    
    
    //let client = &db_connection.client;
    

    database::filestable::write_to_files_table(&meta.project, &meta.uuid, &meta.filename, "",db_client).await?;
    
    //let client = database::get_db();
    //let shape: ! = shape::new_shape_4325(local_path)?;
    
    /*let result = shape::new_shape_4325("./data/viirs-granule-true-color_20221123T142204.tiff");
    if let Ok(shape) = result {
        println!("{}", shape.get_coords_string())
    }*/
    

    /*database::write_to_files_table(
        meta.project.as_ref().ok_or("Missing project")?,
        meta.uuid.as_ref().ok_or("Missing UUID")?,
        meta.filename.as_ref().ok_or("Missing filename")?,
        "{}",
    )?;
*/
    let dataset = Dataset::open(meta.filepath.clone())?;
    //let transf = dataset.gcp_spatial_ref()?;
    
    //let tfm = dataset.geo_transform()?;
    let coordinates = shape.get_coords_string();

        //let srs =  ::get_srs(local_path)?;
    
    let srs= dataset.spatial_ref()?.to_proj4()?;

    println!("{}", srs);
    let location = &meta.filepath.as_str();
    let fileuuid = Uuid::new_v4();
    database::rastertable::write_to_raster_table(&meta.project, &meta.product, coordinates.as_str(), fileuuid, &location, &meta.timestamp, &srs, db_client).await?;
    /*database::write_to_raster_table(
        meta.project.as_ref().ok_or("Missing project")?,
        meta.product.as_ref().ok_or("Missing product")?,
        &coordinates,
        meta.uuid.as_ref().ok_or("Missing UUID")?,
        meta.timestamp.as_ref().ok_or("Missing timestamp")?,
        &srs,
    )?;*/

    Ok(())
}
/* 
pub fn consume_file(folder: &str) -> Result<(), Box<dyn Error>> {
    let meta = Meta::get_metadata(folder)?;

    if let Some(filename) = &meta.filename {
        println!("Meta.filename: {}", filename);
    }

    consume_tiff(&meta)?;

    Ok(())
}

/*fn main() {
    if let Err(e) = consume_file("./data") {
        eprintln!("Error: {}", e);
    }
}*/

// The following modules and functions need to be defined according to your implementation:
// - shape::new_shape_4325
// - shape::Shape::get_coords_string
// - database::write_to_files_table
// - database::write_to_raster_table
// - georef::get_srs
*/