use std::sync::Arc;

use once_cell::sync::Lazy;
use quick_cache::sync::Cache;

use crate::model::cubs_model::ModelData;

const CACHE_SIZE: usize = 2;

#[derive(Clone)]
pub struct QuickCache {
    pub data: Arc<Cache<String, ModelData>>,
}

impl QuickCache {
    pub fn get(&self, key: &str) -> Option<ModelData> {
        self.data.get(key)
    }

    pub fn insert(&self, key: &str, value: &ModelData) {
        println!("[QuickCache] insert into cache with capacity: {}", self.data.capacity());
        self.data.insert(key.to_string(), value.clone());
    }
}

pub static GLOBAL_CACHE: Lazy<Arc<Cache<String, ModelData>>> = Lazy::new(|| {
    Arc::new(Cache::new({
        std::env::var("CACHE_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(CACHE_SIZE)
    }))
});

pub fn get_quick_cache() -> QuickCache {
    QuickCache {
        data: GLOBAL_CACHE.clone(),
    }
}
