use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub driver: String,
    pub server: String,
    pub database: String,
    pub schema: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    pub min_size: u32,
    pub max_size: u32,
    pub timeout_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchConfig {
    pub batch_size: usize,
    pub commit_frequency: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParallelConfig {
    pub enabled: bool,
    pub thread_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub dsn: DsnConfig,
    pub connection_pool: ConnectionPoolConfig,
    pub batch: BatchConfig,
    pub parallel: ParallelConfig,
    pub log_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DsnConfig {
    pub origin: DatabaseConfig,
    pub reference: DatabaseConfig,
    pub target: DatabaseConfig,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        // TODO: Implement validation logic
        Ok(())
    }
} 