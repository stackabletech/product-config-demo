use serde::Serialize;

#[derive(Serialize)]
pub struct ProductConfig {
    pub cluster_name: String,
    pub cluster_url: String,
}
