use serde::Serialize;

use crate::versioning::{Downgrade, Versioned};

use super::v2;
pub use v2::Tls;

#[derive(Serialize)]
pub struct ProductConfig {
    pub cluster_name: String,
    pub insecure_cluster_url: String,
    pub tls: Option<Tls>,
}

impl Versioned for ProductConfig {
    fn version() -> &'static str {
        "v3"
    }
}

impl<T: Downgrade<ProductConfig>> Downgrade<v2::ProductConfig> for T {
    fn downgrade(self) -> v2::ProductConfig {
        let ProductConfig {
            cluster_name,
            insecure_cluster_url,
            tls,
        } = self.downgrade();
        v2::ProductConfig {
            cluster_name,
            cluster_url: insecure_cluster_url,
            tls,
        }
    }
}
