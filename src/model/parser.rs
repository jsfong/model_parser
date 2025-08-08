use flate2::bufread::GzDecoder;
use sqlx::{Pool, Postgres};
use std::error::Error;
use std::io::BufReader;
use std::path::Path;
use std::{fs::File, io::Read};
use std::time::Instant;

use super::cubs_model::{ModelData, ModelResponse};

#[derive(Debug, sqlx::FromRow)]
struct SavedModel {
    pub model_id: String,
    pub vers_no: i32,
    pub saved_gzip: Vec<u8>,
}

pub fn read_model_response_from_file<P>(path: P) -> Result<ModelResponse, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    //Open the fie in read-only model with buffer
    let file = File::open(path).expect("Should have been able to open the input file");
    let reader = BufReader::new(file);

    //Read the JSON contents of the file as Model Response
    let model_respose = serde_json::from_reader(reader)?;

    Ok(model_respose)
}

pub fn read_model_data_from_file<P>(path: P) -> Result<ModelData, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    //Open the fie in read-only model with buffer
    let file = File::open(path).expect("Should have been able to open the input file");
    let reader = BufReader::new(file);

    //Read the JSON contents of the file as Model Response
    let result = serde_json::from_reader(reader)?;

    Ok(result)
}

pub async fn read_model_data_from_db(
    model_id: &String,
    pool: &Pool<Postgres>,
) -> Result<ModelData, Box<dyn Error>> {
    let start_time = Instant::now();

    // Retrieve from DB
    println!("Retreiving from DB ...");
    let saved_model = sqlx::query_as!(
        SavedModel,
        r#"SELECT model_id, vers_no, saved_gzip FROM cubs_object_model.saved_model WHERE model_id = $1"#,
        model_id
    )
    .fetch_one(pool)
    .await?;

    // Unzip
    println!("Unzip ...");
    let decompressed_model = decompress_gzip_to_string(&saved_model.saved_gzip)?;

    //Convert to ModelData
    println!("Convert to internal format ...");
    let model_data = serde_json::from_str(&decompressed_model)?;

    //Log time
    let elapsed_time = start_time.elapsed();
    println!(
        "[Execution time] {} - {:?}",
        "read_model_data_from_db", elapsed_time
    );

    Ok(model_data)
}

fn decompress_gzip_to_string(gzip: &Vec<u8>) -> Result<String, Box<dyn Error>> {
    let mut decoder = GzDecoder::new(gzip.as_slice());
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data)?;
    Ok(decompressed_data)
}
