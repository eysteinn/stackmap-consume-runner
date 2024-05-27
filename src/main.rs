
mod geo;
mod consume;
mod database;

//mod shape;

use geo::shape;
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let dbpass = env::var("PSQL_PASS").unwrap_or_default();
    let dbhost= env::var("PSQL_HOST").unwrap_or("postgresql.default.svc.cluster.local".to_string());

    let config = format!("host={host} user={user} password={password} dbname={dbname} port={port} sslmode=disable", 
        host=dbhost,
        user="postgres",
        password=dbpass,
        dbname="postgres",
        port="5432"
    );
    database::database::init_db(config.as_str()).await?;



    consume::consumefile::consume_file("./data/incoming/somehash/").await?;
    /*match shape::new_shape_4325("./data/viirs-granule-true-color_20221123T142204.tiff") {
        Ok(shape) => println!("{}", shape.get_coords_string()),
        Err(e) => eprintln!("Error creating shape: {}", e),
    }*/
    //let s = shape::new_shape_4325("./data/viirs-granule-true-color_20221123T142204.tiff")?;
    //println!("{}", s.get_coords_string());
    //test::test1();
    Ok(())

}



