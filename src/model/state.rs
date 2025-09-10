use std::sync::Arc;

use crate::model::cubs_model::ModelData;
use crate::model::database_util::{self, connect_to_db};
use quick_cache::sync::Cache;
const CACHE_SIZE: usize = 2;

#[derive(Clone, Debug)]
pub struct AppState {
    pg_pool: sqlx::Pool<sqlx::Postgres>,
    cache: QuickCache,
}

impl AppState {
    pub async fn new() -> Self {
        // DB pool
        let pg_pool = connect_to_db().await;

        // Cache
        let cache: Arc<Cache<String, ModelData>> = Arc::new(Cache::new({
            std::env::var("CACHE_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(CACHE_SIZE)
        }));

        AppState {
            pg_pool,
            cache: QuickCache { data: cache },
        }
    }

    pub fn get_pg_pool_ref(&self) -> &sqlx::Pool<sqlx::Postgres> {
        &self.pg_pool
    }

    pub fn get_cache(&self) -> QuickCache {
        self.cache.clone()
    }
}

#[derive(Clone, Debug)]
pub struct QuickCache {
    pub data: Arc<Cache<String, ModelData>>,
}

impl QuickCache {
    pub fn get(&self, key: &str) -> Option<ModelData> {
        self.data.get(key)
    }

    pub fn insert(&self, key: &str, value: &ModelData) {
        println!(
            "[QuickCache] insert into cache with capacity: {}",
            self.data.capacity()
        );
        self.data.insert(key.to_string(), value.clone());
    }
}
