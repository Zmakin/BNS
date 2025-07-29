use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    // Existing fields
    pub chain: String,
    pub bitcoin_rpc_url: String,
    pub bitcoin_rpc_username: String,
    pub bitcoin_rpc_password: String,
    pub data_dir: String,
    #[serde(default)]
    pub bitmap: BitmapConfig,
}

#[derive(Deserialize, Default)]
pub struct BitmapConfig {
    pub cache_blocks: Option<usize>,
    pub validate_sat: Option<bool>,
    pub parallelism_enabled: Option<bool>,
    pub batch_size: Option<usize>,
    pub bns_history_mode: Option<String>,
    pub bootstrap_nodes: Option<Vec<String>>,
}
