use gdal::vector::Geometry;
use gdal::Dataset;
use gdal_sys::{OGRSpatialReferenceH, OGRCoordinateTransformationH};
use std::path::Path;

pub struct Shape {
    x_geo: Vec<f64>,
    y_geo: Vec<f64>,
    npts: usize,
}

impl Shape {
    pub fn get_coords_string(&self) -> String {
        let pairs: Vec<String> = self.x_geo.iter().zip(&self.y_geo)
            .map(|(x, y)| format!("{} {}", x, y))
            .collect();
        pairs.join(",")
    }
}

fn transf(gt: [f64; 6], x: i32, y: i32) -> (f64, f64) {
    let x_geo = gt[0] + (x as f64) * gt[1] + (y as f64) * gt[2];
    let y_geo = gt[3] + (x as f64) * gt[4] + (y as f64) * gt[5];
    (x_geo, y_geo)
}

pub fn new_shape_4325(input_file: &str) -> Result<Shape, Box<dyn std::error::Error>> {
    let dataset = Dataset::open(Path::new(input_file))?;
    let tfm = dataset.geo_transform()?;

    let source_srs = dataset.spatial_ref().unwrap();
    let target_srs = gdal::spatial_ref::SpatialRef::from_epsg(4326)?;

    let transform = gdal::spatial_ref::CoordTransform::new(&source_srs, &target_srs)?;

    let mut x_geo = Vec::new();
    let mut y_geo = Vec::new();

    let xmrg = (0.05 * dataset.raster_size().0 as f64) as i32;
    let ymrg = (0.05 * dataset.raster_size().1 as f64) as i32;
    let parts = 5;

    let (mut xtmp, mut ytmp) = transf(tfm, -xmrg, -ymrg);
    x_geo.push(xtmp);
    y_geo.push(ytmp);

    for i in 1..parts {
        let dx = dataset.raster_size().0 as i32 * i / parts;
        let (xtmp, ytmp) = transf(tfm, dx, -ymrg);
        x_geo.push(xtmp);
        y_geo.push(ytmp);
    }

    let (xtmp, ytmp) = transf(tfm, dataset.raster_size().0 as i32 + xmrg, -ymrg);
    x_geo.push(xtmp);
    y_geo.push(ytmp);

    for i in 1..parts {
        let dy = dataset.raster_size().1 as i32 * i / parts;
        let (xtmp, ytmp) = transf(tfm, dataset.raster_size().0 as i32 + xmrg, dy);
        x_geo.push(xtmp);
        y_geo.push(ytmp);
    }

    let (xtmp, ytmp) = transf(tfm, dataset.raster_size().0 as i32 + xmrg, dataset.raster_size().1 as i32 + ymrg);
    x_geo.push(xtmp);
    y_geo.push(ytmp);

    for i in 1..parts {
        let dx = dataset.raster_size().0 as i32 - dataset.raster_size().0 as i32 * i / parts;
        let (xtmp, ytmp) = transf(tfm, dx, dataset.raster_size().1 as i32 + ymrg);
        x_geo.push(xtmp);
        y_geo.push(ytmp);
    }

    let (xtmp, ytmp) = transf(tfm, -xmrg, dataset.raster_size().1 as i32 + ymrg);
    x_geo.push(xtmp);
    y_geo.push(ytmp);

    for i in 1..parts {
        let dy = dataset.raster_size().1 as i32 - dataset.raster_size().1 as i32 * i / parts;
        let (xtmp, ytmp) = transf(tfm, -xmrg, dy);
        x_geo.push(xtmp);
        y_geo.push(ytmp);
    }

    let (xtmp, ytmp) = transf(tfm, -xmrg, -ymrg);
    x_geo.push(xtmp);
    y_geo.push(ytmp);

    let npts = x_geo.len();
    let mut zpoints = vec![0.0; npts];

    transform.transform_coords(&mut x_geo, &mut y_geo, &mut zpoints)?;

    Ok(Shape { x_geo, y_geo, npts })
}
/* 
struct ConsumerObject {
    file: LocalFile,
    geo: GeoInfo,
}

struct LocalFile {
    path: String,
}

struct GeoInfo {
    shape: Option<Shape>,
    srs: Option<String>,
}

impl ConsumerObject {
    fn fill_geo(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let filepath = &self.file.path;

        if self.geo.shape.is_none() {
            println!("Getting shape from: {}", filepath);
            let shape = new_shape_4325(filepath)?;
            self.geo.shape = Some(shape);
        }

        if self.geo.srs.is_none() {
            let dataset = Dataset::open(Path::new(filepath))?;
            let source_srs = dataset.spatial_ref().unwrap();
            self.geo.srs = Some(source_srs.to_proj4()?);
        }

        Ok(())
    }
}*/
/*
fn main() {
    // Example usage
    let local_file = LocalFile { path: String::from("./data/viirs-granule-true-color_20221123T142204.tiff") };
    let geo_info = GeoInfo { shape: None, srs: None };
    let mut consumer_object = ConsumerObject { file: local_file, geo: geo_info };

    if let Err(e) = consumer_object.fill_geo() {
        eprintln!("Error filling geo info: {}", e);
    }
}
*/