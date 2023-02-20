use serde::Serialize;

use crate::versioning::Versioned;

#[derive(Serialize)]
pub struct ProductConfig {
    pub cluster_name: String,
    pub cluster_url: String,
}

impl Versioned for ProductConfig {
    fn version() -> &'static str {
        "v1"
    }
}
