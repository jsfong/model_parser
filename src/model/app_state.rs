use std::sync::Arc;

use crate::model::cubs_model::ModelData;
use crate::model::database_util::{self, connect_to_db};
use crate::model::element_connector::ElementConnectorGraph;
use quick_cache::sync::Cache;
const CACHE_SIZE: usize = 2;

#[derive(Clone, Debug)]
pub struct AppState {
    pg_pool: sqlx::Pool<sqlx::Postgres>,
    model_cache: QuickCache<ModelData>,
    graph_cache: QuickCache<ElementConnectorGraph>,
}

impl AppState {
    pub async fn new() -> Self {
        // DB pool
        let pg_pool = connect_to_db().await;

        // Model Cache
        let model_cache: Arc<Cache<String, ModelData>> = Arc::new(Cache::new({
            std::env::var("CACHE_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(CACHE_SIZE)
        }));

        // Graph Cache
        let graph_cache: Arc<Cache<String, ElementConnectorGraph>> = Arc::new(Cache::new({
            std::env::var("CACHE_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(CACHE_SIZE)
        }));

        AppState {
            pg_pool,
            model_cache: QuickCache { data: model_cache },
            graph_cache: QuickCache { data: graph_cache },
        }
    }

    pub fn get_pg_pool_ref(&self) -> &sqlx::Pool<sqlx::Postgres> {
        &self.pg_pool
    }

    pub fn get_model_cache(&self) -> QuickCache<ModelData> {
        self.model_cache.clone()
    }

    pub fn get_graph_cache(&self) -> QuickCache<ElementConnectorGraph> {
        self.graph_cache.clone()
    }
}

#[derive(Clone, Debug)]
pub struct QuickCache<T> {
    pub data: Arc<Cache<String, T>>,
}

impl<T> QuickCache<T> {
    pub fn get(&self, key: &str, version: &str) -> Option<T>
    where
        T: Clone,
    {
        let key = format!("{}-{}", key, version);
        self.data.get(&key)
    }

    pub fn insert(&self, key: &str, version: &str, value: &T)
    where
        T: Clone,
    {
        println!(
            "[QuickCache] insert into cache with capacity: {}",
            self.data.capacity()
        );
         let key = format!("{}-{}", key, version);
        self.data.insert(key.to_string(), value.clone());
    }
}
