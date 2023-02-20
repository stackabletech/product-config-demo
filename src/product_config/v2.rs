use serde::Serialize;

#[derive(Serialize)]
pub struct ProductConfig {
    pub cluster_name: String,
    pub cluster_url: String,
    pub tls: Option<Tls>,
}

#[derive(Serialize)]
pub struct Tls {
    pub key_file: String,
    pub cert_file: String,
    pub ca_file: String,
}
