use crate::model::cache::{self, QuickCache};

use super::cubs_model::{ModelData, ModelResponse};
use anyhow::anyhow;
use flate2::bufread::GzDecoder;
use sqlx::{Pool, Postgres};
use std::error::Error;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;
use std::{fs::File, io::Read};
use uuid::Uuid;
#[derive(Debug, sqlx::FromRow)]
struct SavedModel {
    pub model_id: String,
    pub vers_no: i32,
    pub saved_gzip: Vec<u8>,
}
// TODO expose version to UI

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

async fn read_model_data_from_db(
    model_id: &String,
    pool: &Pool<Postgres>,
    cache: &QuickCache,
) -> Result<ModelData, Box<dyn Error>> {
    let start_time = Instant::now();

    println!("[read_model_data_from_db] Retrieving {} model from DB...", &model_id);

    // Retrieve from DB
    println!("[read_model_data_from_db] Retreiving from DB ...");
    let saved_model = sqlx::query_as!(
        SavedModel,
        r#"SELECT model_id, vers_no, saved_gzip FROM cubs_object_model.saved_model WHERE model_id = $1 ORDER BY vers_no DESC
LIMIT 1"#,
        model_id
    )
    .fetch_one(pool)
    .await?;
    println!(
        "[read_model_data_from_db]  Load saved model with model id: {} version: {} from DB",
        saved_model.model_id, saved_model.vers_no
    );

    // Unzip
    println!("[read_model_data_from_db] Unzip ...");
    let decompressed_model = decompress_gzip_to_string(&saved_model.saved_gzip)?;

    //Convert to ModelData
    println!("[read_model_data_from_db] Convert to internal format ...");
    let model_data = serde_json::from_str(&decompressed_model)?;

    // Store in cache
    cache.insert(&model_id, &model_data);

    //Log time
    let elapsed_time = start_time.elapsed();
    println!(
        "[Execution time] {} - {:?}",
        "read_model_data_from_db + cache", elapsed_time
    );

    Ok(model_data)
}

pub async fn read_model_data(
    model_id: &String,
    pool: &Pool<Postgres>,
) -> Result<ModelData, Box<dyn Error>> {
    let start_time = Instant::now();

    println!("[read_model_data] Retrievig {} model ...", &model_id);

    // Check format
    match Uuid::parse_str(&model_id) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("[read_model_data] Error parsing uuid string {}", e.to_string());
            return Err(anyhow!("Model id is not uuid").into());
        }
    }

    // Get from cache
    let cache = cache::get_quick_cache();
    if let Some(cached_model_data) = cache.get(model_id) {
        println!("[read_model_data] Found model data {} in cache", model_id);
        return Ok(cached_model_data);
    }

    // Get from DB
    let model_data = read_model_data_from_db(model_id, pool, &cache).await;

    //Log time
    let elapsed_time = start_time.elapsed();
    println!(
        "[Execution time] {} - {:?}",
        "read_model_data", elapsed_time
    );

    model_data
}

fn decompress_gzip_to_string(gzip: &Vec<u8>) -> Result<String, Box<dyn Error>> {
    let mut decoder = GzDecoder::new(gzip.as_slice());
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data)?;
    Ok(decompressed_data)
}
