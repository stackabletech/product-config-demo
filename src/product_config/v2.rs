use serde::Serialize;

use crate::versioning::{Downgrade, Versioned};

use super::v1;

#[derive(Serialize)]
pub struct ProductConfig {
    pub cluster_name: String,
    pub cluster_url: String,
    pub tls: Option<Tls>,
}

impl Versioned for ProductConfig {
    fn version() -> &'static str {
        "v2"
    }
}

impl<T: Downgrade<ProductConfig>> Downgrade<v1::ProductConfig> for T {
    fn downgrade(self) -> v1::ProductConfig {
        let ProductConfig {
            cluster_name,
            cluster_url,
            tls,
        } = self.downgrade();
        if tls.is_some() {
            panic!("v1 does not support TLS")
        }
        v1::ProductConfig {
            cluster_name,
            cluster_url,
        }
    }
}

#[derive(Serialize)]
pub struct Tls {
    pub key_file: String,
    pub cert_file: String,
    pub ca_file: String,
}
